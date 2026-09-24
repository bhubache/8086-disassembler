use std::fmt;

use crate::immediate::Immediate8;
use crate::immediate::Immediate16;
use crate::prefixes::Prefixes;
use crate::register::SegmentRegister;

use crate::rm::RmCode;
use crate::width::OpWidth;

#[derive(Debug)]
pub enum ModRm<W: OpWidth> {
    Register(W::Register),
    EffectiveAddr(MemoryIndex),
}

impl<W: OpWidth> ModRm<W> {
    pub fn from_register(reg: W::Register) -> Self {
        Self::Register(reg)
    }

    pub fn from_mem(mem: MemoryIndex) -> Self {
        Self::EffectiveAddr(mem)
    }
}

impl<W: OpWidth> fmt::Display for ModRm<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Register(gen_reg) => write!(f, "{}", gen_reg),
            Self::EffectiveAddr(mem_index) => write!(f, "{} {}", W::MEMORY_SIZE, mem_index),
        }
    }
}

#[derive(Debug)]
pub enum AddressCalculation {
    BxSi,
    BxDi,
    BpSi,
    BpDi,
    Si,
    Di,
    Bx,
    Bp,
}

impl From<RmCode> for AddressCalculation {
    fn from(value: RmCode) -> Self {
        match value {
            RmCode::Rm000 => Self::BxSi,
            RmCode::Rm001 => Self::BxDi,
            RmCode::Rm010 => Self::BpSi,
            RmCode::Rm011 => Self::BpDi,
            RmCode::Rm100 => Self::Si,
            RmCode::Rm101 => Self::Di,
            RmCode::Rm110 => Self::Bp,
            RmCode::Rm111 => Self::Bx,
        }
    }
}

impl fmt::Display for AddressCalculation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BxSi => write!(f, "bx+si"),
            Self::BxDi => write!(f, "bx+di"),
            Self::BpSi => write!(f, "bp+si"),
            Self::BpDi => write!(f, "bp+di"),
            Self::Si => write!(f, "si"),
            Self::Di => write!(f, "di"),
            Self::Bx => write!(f, "bx"),
            Self::Bp => write!(f, "bp"),
        }
    }
}

#[derive(Debug)]
pub enum Displacement {
    None,
    D8(Immediate8),
    D16(Immediate16),
}

impl fmt::UpperHex for Displacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, ""),
            Self::D8(immed) => fmt::UpperHex::fmt(immed, f),
            Self::D16(immed) => fmt::UpperHex::fmt(immed, f),
        }
    }
}

impl From<Immediate8> for Displacement {
    fn from(value: Immediate8) -> Self {
        Self::D8(value)
    }
}

impl From<Immediate16> for Displacement {
    fn from(value: Immediate16) -> Self {
        Self::D16(value)
    }
}

#[derive(Debug)]
pub enum MemoryIndex {
    Based {
        sr: SegmentRegister,
        addr_calc: AddressCalculation,
        displacement: Displacement,
    },
    Direct {
        sr: SegmentRegister,
        address: Immediate16,
    },
}

impl MemoryIndex {
    pub fn with_address(address: Immediate16, prefixes: &Prefixes) -> MemoryIndex {
        let sr = match prefixes.sr_override {
            Some(seg_reg) => seg_reg,
            None => SegmentRegister::DS,
        };

        MemoryIndex::Direct { address, sr }
    }
}

impl fmt::Display for MemoryIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Direct { sr, address } => write!(f, "[{}:{:X}]", sr, address),
            Self::Based {
                sr,
                addr_calc,
                displacement,
            } => {
                write!(
                    f,
                    "[{}:{}{}]",
                    sr,
                    addr_calc,
                    match *displacement {
                        Displacement::None => String::new(),
                        Displacement::D8(Immediate8(byte)) => {
                            let signed_value = byte as i8;

                            format!(
                                "{}{:X}h",
                                if signed_value < 0 { "-" } else { "+" },
                                signed_value.wrapping_abs(),
                            )
                        }
                        Displacement::D16(Immediate16(word)) => {
                            let signed_value = word as i16;

                            format!(
                                "{}{:X}h",
                                if signed_value < 0 { "-" } else { "+" },
                                signed_value.wrapping_abs(),
                            )
                        }
                    }
                )
            }
        }
    }
}
