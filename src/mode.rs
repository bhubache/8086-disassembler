use crate::disassembler::DisassemblerError;

#[derive(Debug, PartialEq)]
pub enum Mode {
    MemNoDisplacement,
    Mem8BitDisplacement,
    Mem16BitDisplacement,
    Register,
}

impl Mode {
    pub fn from_byte(byte: u8) -> Result<Self, DisassemblerError> {
        Mode::from_encoding((byte & 0b11000000) >> 6)
    }

    pub fn from_encoding(mode: u8) -> Result<Self, DisassemblerError> {
        let parsed_mode = match mode {
            0b00 => Self::MemNoDisplacement,
            0b01 => Self::Mem8BitDisplacement,
            0b10 => Self::Mem16BitDisplacement,
            0b11 => Self::Register,
            _ => return Err(DisassemblerError::InvalidMode(mode)),
        };

        Ok(parsed_mode)
    }
}
