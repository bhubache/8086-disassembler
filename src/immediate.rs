use std::fmt;

use crate::immediate_parsing::ByteReader;
use crate::immediate_parsing::ParseImmediate;

pub trait Immediate: fmt::UpperHex + fmt::Debug {}

#[derive(Debug)]
pub struct Immediate8(pub u8);

impl Immediate8 {
    pub fn sign_extend(&self) -> u16 {
        match ((self.0 & 0b10000000) >> 7) == 1 {
            true => (0b11111111 << 8) | self.0 as u16,
            false => self.0 as u16,
        }
    }
}

impl From<u8> for Immediate8 {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl fmt::Display for Immediate8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::UpperHex for Immediate8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}h", self.0)
    }
}

impl Immediate for Immediate8 {}

impl ParseImmediate for Immediate8 {
    fn parse_immediate<R: ByteReader>(byte_reader: &mut R) -> Result<Self, R::Error> {
        Ok(Self(byte_reader.read_byte()?))
    }
}

#[derive(Debug)]
pub struct Immediate16(pub u16);

impl From<u16> for Immediate16 {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl fmt::Display for Immediate16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::UpperHex for Immediate16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}h", self.0)
    }
}

#[derive(Debug)]
pub enum ImmedGroupImmediate16 {
    Full(Immediate16),
    SignExtended8(Immediate8),
}

impl fmt::Display for ImmedGroupImmediate16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Full(immed) => write!(f, "{}", immed),
            Self::SignExtended8(immed) => write!(f, "{}", immed),
        }
    }
}

impl fmt::UpperHex for ImmedGroupImmediate16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Full(immed) => write!(f, "{:X}", immed),
            Self::SignExtended8(immed) => write!(f, "{:X}h", immed.sign_extend()),
        }
    }
}

impl Immediate for Immediate16 {}

impl ParseImmediate for Immediate16 {
    fn parse_immediate<R: ByteReader>(byte_reader: &mut R) -> Result<Self, R::Error> {
        Ok(Self(byte_reader.read_word()?))
    }
}
