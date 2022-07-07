use crate::val::{
    byte::Byte,
    word::{Word, BYTES_IN_WORD},
};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Calldata {
    inner: Vec<Byte>,
}

impl Calldata {
    pub fn new(data: Vec<Byte>) -> Self {
        Self { inner: data }
    }

    pub fn size(&self) -> Word {
        self.inner.len().into()
    }

    pub fn read_word(&self, idx: Word) -> Word {
        let idx_unwrapped: usize = idx.into();
        Word::from_bytes_vec(&self.inner, idx_unwrapped, BYTES_IN_WORD, true)
    }

    // TODO(will) - n_symbolic_bytes should be usize
    pub fn symbolic(function_selector: [u8; 4], n_symbolic_bytes: u8) -> Self {
        let mut calldata: Vec<Byte> = Vec::from(function_selector)
            .into_iter()
            .map(|x| x.into())
            .collect();

        let args = (5_u8..(5 + n_symbolic_bytes)).map(|idx| {
            let mut s: String = "calldata".into();
            s.push_str(&idx.to_string());
            Byte::S(s)
        });

        calldata.extend(args);

        calldata.into()
    }

    pub fn inner(&self) -> &Vec<Byte> {
        &self.inner
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
