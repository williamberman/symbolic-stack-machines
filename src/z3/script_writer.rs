use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};

use im::Vector;
use log::info;
use uuid::Uuid;
use z3::{
    ast::{Ast, Bool, BV},
    Context,
};

use crate::val::{byte::Byte, constraint::Constraint, word::Word};

use super::{make_z3_bitvec_from_word, make_z3_constraint};

// TODO(will) - remove struct and use directly. Was previously needed for use by external consumers.
struct Smtlib2ScriptFileWriter {
    f: File,
    file_path: PathBuf,
}

pub fn write_script<'ctx>(
    ctx: &'ctx Context,
    constraints: &Vector<Constraint>,
    words: &Vec<Word>,
    bytes: &Vec<Byte>,
    variables: &HashMap<Word, String>,
) {
    let mut script_writer = Smtlib2ScriptFileWriter::new();

    info!(
        "writing constraints to: {}",
        script_writer.file_path.to_str().unwrap()
    );

    bytes.into_iter().for_each(|b| match b {
        Byte::S(s) => {
            script_writer.write_byte(s);
        }
        _ => {}
    });

    script_writer.write_newline();

    words.into_iter().for_each(|w| match w {
        Word::S(s) => {
            script_writer.write_word(s);
        }
        _ => {}
    });

    script_writer.write_newline();

    let empty = HashMap::new();
    variables.iter().for_each(|(word, variable_name)| {
        let bv = make_z3_bitvec_from_word(ctx, word, &empty).simplify();
        script_writer.write_variable(variable_name, &bv);
    });

    script_writer.write_newline();

    constraints.iter().for_each(|c| {
        let z3_constraint = make_z3_constraint(&ctx, c, variables).simplify();
        script_writer.write_constraint(&z3_constraint);
    });

    script_writer.write_newline();
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

    fn write_byte(&mut self, s: &String) {
        let formatted = format!("(declare-const |{}| (_ BitVec 8))\n", s);
        self.f.write(formatted.as_bytes()).unwrap();
    }

    fn write_word(&mut self, s: &String) {
        let formatted = format!("(declare-const |{}| (_ BitVec 256))\n", s);
        self.f.write(formatted.as_bytes()).unwrap();
    }

    pub fn write_variable<'ctx>(&mut self, variable_name: &String, w: &BV<'ctx>) {
        let s = format!("(define-const |{}| (_ BitVec 256) {})\n", variable_name, w.to_string());
        self.f.write(s.as_bytes()).unwrap();
    }

    fn write_constraint<'ctx>(&mut self, c: &Bool<'ctx>) {
        let s = format!("(assert {})\n\n", c.to_string());
        self.f.write(s.as_bytes()).unwrap();
    }

    fn write_newline(&mut self) {
        self.f.write("\n".as_bytes()).unwrap();
    }
}
