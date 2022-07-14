// // SPDX-License-Identifier: UNLICENSED
// pragma solidity ^0.8.9;
// 
// contract ReturnTest {
//     function foo() public pure returns(uint) {
//       return 1337;
//     }
// }

use std::rc::Rc;

use symbolic_stack_machines::{instructions::parse_bytecode, machine::{Machine, mem_ptr::MemPtr}};

static BYTECODE: &str = "6080604052348015600f57600080fd5b506004361060285760003560e01c8063c298557814602d575b600080fd5b60336047565b604051603e91906068565b60405180910390f35b6000610539905090565b6000819050919050565b6062816051565b82525050565b6000602082019050607b6000830184605b565b9291505056fea264697066735822122029e20e112b5d95df14a44bfdaa1c515dd1d110fb0be15a60d962e1103c89f53c64736f6c634300080c0033";

static FUNCTION_SELECTOR_ARR: [u8; 4] = [0xc2, 0x98, 0x55, 0x78];

#[test]
pub fn test_return() {
    let pgm = parse_bytecode(BYTECODE);
    let mut m = Machine::new(pgm);

    m.calldata = Rc::new(Vec::from(FUNCTION_SELECTOR_ARR).into());

    let res = m.run_sym();

    assert_eq!(res.pruned.len(), 0);
    assert_eq!(res.leaves.len(), 1);

    let returned = res.leaves.get(0).unwrap();

    assert_eq!(returned.return_ptr.as_ref().unwrap(), &MemPtr {
        offset: 128.into(),
        length: 32.into()
    });

    assert_eq!(returned.return_string().unwrap(), "0000000000000000000000000000000000000000000000000000000000000539");
}
