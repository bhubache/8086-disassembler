use std::fmt;

pub trait SizedRegister {
    fn from_byte(byte: u8) -> Self;
    fn from_reg_encoding(reg: u8) -> Self;
    fn to_sr(&self) -> SegmentRegister;
}

#[derive(Debug, Copy, Clone)]
pub enum GeneralRegister8 {
    // low 8 bits
    AL,
    CL,
    DL,
    BL,

    // high 8 bits
    AH,
    CH,
    DH,
    BH,
}

impl SizedRegister for GeneralRegister8 {
    fn from_byte(byte: u8) -> Self {
        Self::from_reg_encoding((byte & 0b00111000) >> 3)
    }

    fn from_reg_encoding(reg: u8) -> Self {
        match reg {
            0b000 => Self::AL,
            0b001 => Self::CL,
            0b010 => Self::DL,
            0b011 => Self::BL,
            0b100 => Self::AH,
            0b101 => Self::CH,
            0b110 => Self::DH,
            0b111 => Self::BH,
            _ => panic!("Unexpected register encoded as byte {:08b}", reg),
        }
    }

    fn to_sr(&self) -> SegmentRegister {
        match self {
            Self::AL => SegmentRegister::ES,
            Self::CL => SegmentRegister::CS,
            Self::DL => SegmentRegister::SS,
            Self::BL => SegmentRegister::DS,

            Self::AH => SegmentRegister::ES,
            Self::CH => SegmentRegister::CS,
            Self::DH => SegmentRegister::SS,
            Self::BH => SegmentRegister::DS,
        }
    }
}

impl fmt::Display for GeneralRegister8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AL => write!(f, "al"),
            Self::CL => write!(f, "cl"),
            Self::DL => write!(f, "dl"),
            Self::BL => write!(f, "bl"),
            Self::AH => write!(f, "ah"),
            Self::CH => write!(f, "ch"),
            Self::DH => write!(f, "dh"),
            Self::BH => write!(f, "bh"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum GeneralRegister16 {
    AX, // Accumulator
    CX, // Count
    DX, // Data
    BX, // Base
    SP, // Stack pointer
    BP, // Base pointer
    SI, // Source index
    DI, // Destination index
}

impl SizedRegister for GeneralRegister16 {
    fn from_byte(byte: u8) -> Self {
        Self::from_reg_encoding((byte & 0b00111000) >> 3)
    }

    fn from_reg_encoding(reg: u8) -> Self {
        match reg {
            0b000 => Self::AX,
            0b001 => Self::CX,
            0b010 => Self::DX,
            0b011 => Self::BX,
            0b100 => Self::SP,
            0b101 => Self::BP,
            0b110 => Self::SI,
            0b111 => Self::DI,
            _ => panic!("Unexpected register encoded as word {:8b}", reg),
        }
    }

    fn to_sr(&self) -> SegmentRegister {
        match self {
            Self::AX => SegmentRegister::ES,
            Self::CX => SegmentRegister::CS,
            Self::DX => SegmentRegister::SS,
            Self::BX => SegmentRegister::DS,

            Self::SP => SegmentRegister::ES,
            Self::BP => SegmentRegister::CS,
            Self::SI => SegmentRegister::SS,
            Self::DI => SegmentRegister::DS,
        }
    }
}

impl fmt::Display for GeneralRegister16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AX => write!(f, "ax"),
            Self::CX => write!(f, "cx"),
            Self::DX => write!(f, "dx"),
            Self::BX => write!(f, "bx"),
            Self::SP => write!(f, "sp"),
            Self::BP => write!(f, "bp"),
            Self::SI => write!(f, "si"),
            Self::DI => write!(f, "di"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SegmentRegister {
    ES,
    CS,
    SS,
    DS,
}

impl SegmentRegister {
    pub fn from_byte(byte: u8) -> Self {
        SegmentRegister::from_encoding((byte & 0b00011000) >> 3)
    }

    pub fn from_encoding(reg: u8) -> Self {
        match reg {
            0b00 => Self::ES,
            0b01 => Self::CS,
            0b10 => Self::SS,
            0b11 => Self::DS,
            _ => panic!("Unexpected segment register encoding {}", reg),
        }
    }
}

impl std::fmt::Display for SegmentRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ES => write!(f, "es"),
            Self::CS => write!(f, "cs"),
            Self::SS => write!(f, "ss"),
            Self::DS => write!(f, "ds"),
        }
    }
}
