use std::collections::HashMap;

use crate::val::{
    byte::Byte,
    word::{Word, BYTES_IN_WORD},
};

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Calldata {
    inner: Vec<Byte>,
    // Variable name -> offset word read out of
    pub vars: Option<Vec<(String, usize)>>,
}

impl Calldata {
    pub fn new(data: Vec<Byte>) -> Self {
        Self {
            inner: data,
            vars: None,
        }
    }

    pub fn size(&self) -> Word {
        self.inner.len().into()
    }

    pub fn read_word(&self, idx: Word) -> Word {
        self.read_word_concrete(idx.into())
    }

    pub fn read_word_concrete(&self, idx: usize) -> Word {
        Word::from_bytes_vec(&self.inner, idx, BYTES_IN_WORD, true)
    }

    // TODO(will) - n_symbolic_bytes should be usize
    pub fn symbolic(function_selector: [u8; 4], n_symbolic_bytes: u8) -> Self {
        let mut calldata: Vec<Byte> = Vec::from(function_selector)
            .into_iter()
            .map(|x| x.into())
            .collect();

        let args = (5_u8..(5 + n_symbolic_bytes)).map(|idx| {
            let mut s: String = "calldata[".into();
            s.push_str(&idx.to_string());
            s.push_str("]");
            Byte::S(s)
        });

        calldata.extend(args);

        calldata.into()
    }

    pub fn symbolic_vars(function_selector: [u8; 4], vars: Vec<(String, usize)>) -> Self {
        // If only function selector
        let mut n_symbolic_bytes = 0;

        vars.iter().for_each(|(_, start_idx)| {
            let max_symbolic_bytes = start_idx + 32 - 4;

            if max_symbolic_bytes > n_symbolic_bytes {
                n_symbolic_bytes = max_symbolic_bytes
            }
        });

        let mut rv = Self::symbolic(function_selector, n_symbolic_bytes.try_into().unwrap());
        rv.vars = Some(vars);
        rv
    }

    pub fn inner(&self) -> &Vec<Byte> {
        &self.inner
    }

    pub fn solve(&self, solutions: &HashMap<Byte, u8>) -> String {
        let concrete_calldata: Vec<u8> = self
            .inner
            .iter()
            .map(|sym_byte| solutions.get(sym_byte).unwrap().clone())
            .collect();

        hex::encode(concrete_calldata)
    }

    pub fn variables(&self) -> Vec<(String, Word)> {
        match &self.vars {
            Some(d) => d
                .iter()
                .map(|(variable_name, offset)| {
                    let word = self.read_word_concrete(*offset);
                    (variable_name.clone(), word)
                })
                .collect(),
            None => vec![],
        }
    }

    pub fn variables_name_lookup(&self) -> HashMap<String, Word> {
        self.variables().into_iter().collect()
    }

    pub fn variables_word_lookup(&self) -> HashMap<Word, String> {
        self.variables()
            .into_iter()
            .map(|(variable_name, word)| (word, variable_name))
            .collect()
    }
}

impl From<Vec<Byte>> for Calldata {
    fn from(data: Vec<Byte>) -> Self {
        Self::new(data)
    }
}

impl From<Vec<u8>> for Calldata {
    fn from(data: Vec<u8>) -> Self {
        let d: Vec<Byte> = data.into_iter().map(|x| x.into()).collect();
        d.into()
    }
}

impl Into<String> for Calldata {
    fn into(self) -> String {
        let mut rv = String::new();

        self.inner.into_iter().for_each(|sym_byte| match sym_byte {
            Byte::S(s) => {
                rv.push_str("(");
                rv.push_str(&s);
                rv.push_str(")");
            }
            Byte::C(x) => rv.push_str(&hex::encode(vec![x])),
            Byte::Idx(_w, _idx) => rv.push_str("(TODO - compound expression)"),
        });

        rv
    }
}
