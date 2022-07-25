use itertools::Itertools;
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};

use im::Vector;
use log::info;
use uuid::Uuid;
use z3::{
    ast::{Ast, Bool, BV},
    Context,
};

use crate::{
    calldata::{calldata_idx_string, Calldata},
    machine::Machine,
    val::{byte::Byte, constraint::Constraint, word::Word},
};

use super::{make_z3_bitvec_from_word, make_z3_constraint};

// TODO(will) - remove struct and use directly. Was previously needed for use by external consumers.
struct Smtlib2ScriptFileWriter {
    f: File,
    file_path: PathBuf,
}

// TODO(will) - probably makes sense to consolidate inputs to function
// by pulling words and bytes and variables from the machine.
pub fn write_script<'ctx>(
    ctx: &'ctx Context,
    constraints: &Vector<Constraint>,
    words: &Vec<Word>,
    bytes: &Vec<Byte>,
    variables: &HashMap<Word, String>,
    m: &Machine,
) {
    let mut script_writer = Smtlib2ScriptFileWriter::new();

    let empty_hashmap = HashMap::new();

    let mut variable_names: Vec<String> = variables
        .iter()
        .map(|(_, var_name)| var_name.clone())
        .collect();
    variable_names.push("calldata".to_string());

    info!(
        "writing constraints to: {}",
        script_writer.file_path.to_str().unwrap()
    );

    bytes.into_iter().for_each(|b| match b {
        Byte::C(b, Some(s)) => {
            script_writer.define_byte(s, *b);
        }
        Byte::S(s) => {
            script_writer.declare_byte(s);
        }
        _ => {}
    });

    script_writer.write_newline();

    words.into_iter().for_each(|w| match w {
        Word::S(s) => {
            script_writer.declare_word(s);
        }
        _ => {}
    });

    script_writer.write_newline();

    script_writer.write_calldata(&m.calldata);

    script_writer.write_newline();

    variables.iter().for_each(|(word, variable_name)| {
        let bv = make_z3_bitvec_from_word(ctx, word, &empty_hashmap).simplify();
        script_writer.define_word(variable_name, &bv);
    });

    script_writer.write_newline();

    if let Some(returns) = m.return_word() {
        let bv = make_z3_bitvec_from_word(ctx, &returns, variables).simplify();
        script_writer.define_word(&"returns".to_string(), &bv);
        variable_names.push("returns".to_string());
    }

    script_writer.write_newline();

    constraints.iter().for_each(|c| {
        let z3_constraint = make_z3_constraint(&ctx, c, variables).simplify();
        script_writer.write_constraint(&z3_constraint);
    });

    script_writer.write_newline();

    script_writer.write_postlude(variable_names);
}

impl Smtlib2ScriptFileWriter {
    fn new() -> Self {
        let file_name = format!("{}.smtlib2", Uuid::new_v4().to_string());
        let file_path = std::env::temp_dir().join(file_name);
        let f = File::create(file_path.clone()).unwrap();
        let mut rv = Self { f, file_path };
        rv.write_prelude();
        rv
    }

    fn write_prelude(&mut self) {
        let mut f = &self.f;
        f.write("(set-option :print-success true)\n".as_bytes())
            .unwrap();
        f.write("(set-option :smtlib2_compliant true)\n".as_bytes())
            .unwrap();
        f.write("(set-option :diagnostic-output-channel \"stdout\")\n".as_bytes())
            .unwrap();
        f.write("(set-option :timeout 60000)\n".as_bytes()).unwrap();
        f.write("(set-option :produce-models true)\n".as_bytes())
            .unwrap();
        f.write("(set-logic ALL)\n".as_bytes()).unwrap();

        self.write_newline();
    }

    fn define_byte(&mut self, s: &String, value: u8) {
        let formatted = format!("(define-const |{}| (_ BitVec 8) #x{:02x})\n", s, value);
        self.f.write(formatted.as_bytes()).unwrap();
    }

    fn declare_byte(&mut self, s: &String) {
        let formatted = format!("(declare-const |{}| (_ BitVec 8))\n", s);
        self.f.write(formatted.as_bytes()).unwrap();
    }

    fn define_word<'ctx>(&mut self, variable_name: &String, w: &BV<'ctx>) {
        let s = format!(
            "(define-const |{}| (_ BitVec 256) {})\n",
            variable_name,
            w.to_string()
        );
        self.f.write(s.as_bytes()).unwrap();
    }

    fn declare_word(&mut self, s: &String) {
        let formatted = format!("(declare-const |{}| (_ BitVec 256))\n", s);
        self.f.write(formatted.as_bytes()).unwrap();
    }

    fn write_calldata(&mut self, calldata: &Calldata) {
        #[allow(unstable_name_collisions)]
        let bytes: String = calldata
            .inner()
            .iter()
            .enumerate()
            .map(|(i, _)| calldata_idx_string(i, true))
            .intersperse(" ".into())
            .collect();

        let s = format!(
            "(define-const calldata (_ BitVec {}) (concat {}))\n",
            calldata.inner().len() * 8,
            bytes
        );

        self.f.write(s.as_bytes()).unwrap();
    }

    fn write_constraint<'ctx>(&mut self, c: &Bool<'ctx>) {
        let s = format!("(assert {})\n\n", c.to_string());
        self.f.write(s.as_bytes()).unwrap();
    }

    fn write_postlude(&mut self, variable_names: Vec<String>) {
        #[allow(unstable_name_collisions)]
        let variable_names_str: String = variable_names
            .into_iter()
            .intersperse(" ".to_string())
            .collect();
        let get_values = format!("(get-value ({}))\n", variable_names_str);

        self.f.write("(check-sat)\n".as_bytes()).unwrap();
        self.f.write(get_values.as_bytes()).unwrap();
    }

    fn write_newline(&mut self) {
        self.f.write("\n".as_bytes()).unwrap();
    }
}
