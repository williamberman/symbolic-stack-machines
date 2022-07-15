use std::fs;

pub static PRIMALITY_CHECK_FUNCTION_SELECTOR_ARR: [u8; 4] = [0xd5_u8, 0xa2, 0x42, 0x49];

thread_local! {
    pub static PRIMALITY_CHECK_BYTECODE: String = fs::read_to_string("test-data/PrimalityCheck.bin-runtime").unwrap();
}
