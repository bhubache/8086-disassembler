use std::fmt;

use crate::rm::RmCode;

pub trait GeneralRegister: From<RmCode> + From<RegCode> + fmt::Display + fmt::Debug {}

#[derive(Debug)]
pub struct InvalidRegEncoding(pub u8);

impl fmt::Display for InvalidRegEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid register encoding {:03b}; expected a value in the range 0b000 - 0b111",
            self.0
        )
    }
}

impl std::error::Error for InvalidRegEncoding {}

#[derive(Debug)]
pub enum RegCode {
    Reg000,
    Reg001,
    Reg010,
    Reg011,
    Reg100,
    Reg101,
    Reg110,
    Reg111,
}

impl RegCode {
    pub fn from_modrm_reg(value: u8) -> Self {
        Self::from_3_bits((value & 0b00111000) >> 3)
    }

    fn from_3_bits(value: u8) -> Self {
        match value {
            0b000 => RegCode::Reg000,
            0b001 => RegCode::Reg001,
            0b010 => RegCode::Reg010,
            0b011 => RegCode::Reg011,
            0b100 => RegCode::Reg100,
            0b101 => RegCode::Reg101,
            0b110 => RegCode::Reg110,
            0b111 => RegCode::Reg111,
            _ => unreachable!("caller guarantees 3-bit value"),
        }
    }
}

impl From<GeneralRegister16> for RegCode {
    fn from(value: GeneralRegister16) -> Self {
        match value {
            GeneralRegister16::AX => Self::Reg000,
            GeneralRegister16::CX => Self::Reg001,
            GeneralRegister16::DX => Self::Reg010,
            GeneralRegister16::BX => Self::Reg011,
            GeneralRegister16::SP => Self::Reg100,
            GeneralRegister16::BP => Self::Reg101,
            GeneralRegister16::SI => Self::Reg110,
            GeneralRegister16::DI => Self::Reg111,
        }
    }
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

impl From<RegCode> for GeneralRegister8 {
    fn from(value: RegCode) -> Self {
        match value {
            RegCode::Reg000 => Self::AL,
            RegCode::Reg001 => Self::CL,
            RegCode::Reg010 => Self::DL,
            RegCode::Reg011 => Self::BL,
            RegCode::Reg100 => Self::AH,
            RegCode::Reg101 => Self::CH,
            RegCode::Reg110 => Self::DH,
            RegCode::Reg111 => Self::BH,
        }
    }
}

impl From<RmCode> for GeneralRegister8 {
    fn from(value: RmCode) -> Self {
        match value {
            RmCode::Rm000 => Self::AL,
            RmCode::Rm001 => Self::CL,
            RmCode::Rm010 => Self::DL,
            RmCode::Rm011 => Self::BL,
            RmCode::Rm100 => Self::AH,
            RmCode::Rm101 => Self::CH,
            RmCode::Rm110 => Self::DH,
            RmCode::Rm111 => Self::BH,
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

impl GeneralRegister for GeneralRegister8 {}

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

impl From<RegCode> for GeneralRegister16 {
    fn from(value: RegCode) -> Self {
        match value {
            RegCode::Reg000 => Self::AX,
            RegCode::Reg001 => Self::CX,
            RegCode::Reg010 => Self::DX,
            RegCode::Reg011 => Self::BX,
            RegCode::Reg100 => Self::SP,
            RegCode::Reg101 => Self::BP,
            RegCode::Reg110 => Self::SI,
            RegCode::Reg111 => Self::DI,
        }
    }
}

impl From<RmCode> for GeneralRegister16 {
    fn from(value: RmCode) -> Self {
        match value {
            RmCode::Rm000 => Self::AX,
            RmCode::Rm001 => Self::CX,
            RmCode::Rm010 => Self::DX,
            RmCode::Rm011 => Self::BX,
            RmCode::Rm100 => Self::SP,
            RmCode::Rm101 => Self::BP,
            RmCode::Rm110 => Self::SI,
            RmCode::Rm111 => Self::DI,
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

impl GeneralRegister for GeneralRegister16 {}

#[derive(Debug)]
pub enum SrCode {
    Sr00,
    Sr01,
    Sr10,
    Sr11,
}

impl From<RegCode> for SrCode {
    fn from(value: RegCode) -> Self {
        match value {
            RegCode::Reg000 => Self::Sr00,
            RegCode::Reg001 => Self::Sr01,
            RegCode::Reg010 => Self::Sr10,
            RegCode::Reg011 => Self::Sr11,
            RegCode::Reg100 => Self::Sr00,
            RegCode::Reg101 => Self::Sr01,
            RegCode::Reg110 => Self::Sr10,
            RegCode::Reg111 => Self::Sr11,
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

impl From<SrCode> for SegmentRegister {
    fn from(value: SrCode) -> Self {
        match value {
            SrCode::Sr00 => Self::ES,
            SrCode::Sr01 => Self::CS,
            SrCode::Sr10 => Self::SS,
            SrCode::Sr11 => Self::DS,
        }
    }
}

impl From<GeneralRegister16> for SegmentRegister {
    fn from(value: GeneralRegister16) -> Self {
        let reg_code: RegCode = value.into();
        let sr_code: SrCode = reg_code.into();

        SegmentRegister::from(sr_code)
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
