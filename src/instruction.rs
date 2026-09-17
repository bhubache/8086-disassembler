use std::fmt;

pub struct Instruction {
    pub address: usize,
    pub opcode: Opcode,
}

impl Instruction {
    pub fn new(address: usize, opcode: Opcode) -> Self {
        Instruction { address, opcode }
    }
}

#[derive(Debug)]
pub enum Opcode {}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "opcode")
    }
}
