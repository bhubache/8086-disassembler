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
