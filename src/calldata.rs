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

    pub fn symbolic(function_selector: [u8; 4], n_symbolic_bytes: usize) -> Self {
        let mut calldata: Vec<Byte> = Vec::from(function_selector)
            .into_iter()
            .enumerate()
            .map(|(idx, x)| Byte::C(x, Some(calldata_idx_string(idx, false))))
            .collect();

        let args = (4..(4 + n_symbolic_bytes)).map(|idx| Byte::S(calldata_idx_string(idx, false)));

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
        Byte::solve(self.inner.clone(), solutions)
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
            Byte::C(x, _) => rv.push_str(&hex::encode(vec![x])),
            Byte::Idx(_w, _idx) => rv.push_str("(TODO - compound expression)"),
        });

        rv
    }
}

pub fn calldata_idx_string(i: usize, symbol_bars: bool) -> String {
    let mut s: String = if symbol_bars {
        "|calldata[".into()
    } else {
        "calldata[".into()
    };

    s.push_str(&i.to_string());

    if symbol_bars {
        s.push_str("]|");
    } else {
        s.push_str("]");
    }

    s
}
