use std::{fs::File, io::Write, path::PathBuf};

use uuid::Uuid;
use z3::ast::{Bool, BV};

pub struct Smtlib2ScriptFileWriter {
    f: File,
    pub file_path: PathBuf,
}

impl Smtlib2ScriptFileWriter {
    pub fn new() -> Self {
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

    pub fn write_byte<'ctx>(&mut self, b: &BV<'ctx>) {
        let s = format!("(define-fun {} () (_ BitVec 8))\n", b.to_string());
        self.f.write(s.as_bytes()).unwrap();
    }

    pub fn write_word<'ctx>(&mut self, w: &BV<'ctx>) {
        let s = format!("(define-fun {} () (_ BitVec 256))\n", w.to_string());
        self.f.write(s.as_bytes()).unwrap();
    }

    pub fn write_constraint<'ctx>(&mut self, c: &Bool<'ctx>) {
        let s = format!("(assert {})\n\n", c.to_string());
        self.f.write(s.as_bytes()).unwrap();
    }

    pub fn write_newline(&mut self) {
        self.f.write("\n".as_bytes()).unwrap();
    }
}
