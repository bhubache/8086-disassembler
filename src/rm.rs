use std::fmt;

#[derive(Debug)]
pub struct InvalidRmEncoding(u8);

impl fmt::Display for InvalidRmEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid rm encoding {:03b}; expected a value in the range 0b000 - 0b111",
            self.0
        )
    }
}

impl std::error::Error for InvalidRmEncoding {}

#[derive(Debug, PartialEq)]
pub enum RmCode {
    Rm000,
    Rm001,
    Rm010,
    Rm011,
    Rm100,
    Rm101,
    Rm110,
    Rm111,
}

impl RmCode {
    pub fn from_modrm(value: u8) -> Self {
        Self::from_3_bits(value & 0b000111)
    }

    fn from_3_bits(value: u8) -> Self {
        match value {
            0b000 => Self::Rm000,
            0b001 => Self::Rm001,
            0b010 => Self::Rm010,
            0b011 => Self::Rm011,
            0b100 => Self::Rm100,
            0b101 => Self::Rm101,
            0b110 => Self::Rm110,
            0b111 => Self::Rm111,
            _ => unreachable!("caller guarantees a 3-bit value"),
        }
    }
}

impl TryFrom<u8> for RmCode {
    type Error = InvalidRmEncoding;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 0b111 {
            Err(InvalidRmEncoding(value))
        } else {
            Ok(Self::from_3_bits(value))
        }
    }
}
