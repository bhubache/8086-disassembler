use std::fmt;

#[derive(Debug)]
pub struct InvalidModeEncoding(u8);

impl fmt::Display for InvalidModeEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid mode encoding {:02b}; expected a value in the range 0b00 - 0b11",
            self.0
        )
    }
}

impl std::error::Error for InvalidModeEncoding {}

#[derive(Debug, PartialEq)]
pub enum Mode {
    MemNoDisplacement,
    Mem8BitDisplacement,
    Mem16BitDisplacement,
    Register,
}

impl Mode {
    pub fn from_modrm(value: u8) -> Self {
        Self::from_2_bits((value & 0b11000000) >> 6)
    }

    fn from_2_bits(value: u8) -> Self {
        match value {
            0b00 => Self::MemNoDisplacement,
            0b01 => Self::Mem8BitDisplacement,
            0b10 => Self::Mem16BitDisplacement,
            0b11 => Self::Register,
            _ => unreachable!("caller guarantees a 3-bit value"),
        }
    }
}
