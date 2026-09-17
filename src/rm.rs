use crate::disassembler::DisassemblerError;

#[derive(Debug, PartialEq)]
pub enum RM {
    Rm000,
    Rm001,
    Rm010,
    Rm011,
    Rm100,
    Rm101,
    Rm110,
    Rm111,
}

impl RM {
    pub fn from_byte(byte: u8) -> Result<RM, DisassemblerError> {
        RM::from_encoding(byte & 0b000111)
    }

    pub fn from_encoding(rm: u8) -> Result<RM, DisassemblerError> {
        match rm {
            0b000 => Ok(RM::Rm000),
            0b001 => Ok(RM::Rm001),
            0b010 => Ok(RM::Rm010),
            0b011 => Ok(RM::Rm011),
            0b100 => Ok(RM::Rm100),
            0b101 => Ok(RM::Rm101),
            0b110 => Ok(RM::Rm110),
            0b111 => Ok(RM::Rm111),
            _ => Err(DisassemblerError::InvalidRM(rm)),
        }
    }

    pub fn to_encoding(&self) -> u8 {
        match self {
            Self::Rm000 => 0b000,
            Self::Rm001 => 0b001,
            Self::Rm010 => 0b010,
            Self::Rm011 => 0b011,
            Self::Rm100 => 0b100,
            Self::Rm101 => 0b101,
            Self::Rm110 => 0b110,
            Self::Rm111 => 0b111,
        }
    }
}
