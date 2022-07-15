object "Neg" {
  code {
    // Deploy the contract
    datacopy(0, dataoffset("runtime"), datasize("runtime"))
    return(0, datasize("runtime"))
  }
  object "runtime" {
    code {
      let v := calldataload(4)
      if iszero(iszero(and(v, not(0xffffffffffffffffffffffffffffffffffffffff)))) {
        invalid()
      }
    }
  }
}
