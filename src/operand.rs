use std::fmt;

use crate::prefixes::Prefixes;
use crate::register::GeneralRegister16;
use crate::register::SegmentRegister;

use crate::width::OpWidth;

#[derive(Debug, Copy, Clone)]
pub enum Immediate {
    Byte(u8),
    Word(u16),
}

impl fmt::Display for Immediate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Byte(byte) => write!(f, "{}", byte),
            Self::Word(word) => write!(f, "{}", word),
        }
    }
}

impl fmt::UpperHex for Immediate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Byte(byte) => write!(f, "{:X}", byte),
            Self::Word(word) => write!(f, "{:X}", word),
        }
    }
}

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
pub struct MemoryIndex {
    pub sr: SegmentRegister,
    pub base: Option<GeneralRegister16>,
    pub index: Option<GeneralRegister16>,
    pub displacement: Option<Immediate>,
}

impl MemoryIndex {
    pub fn with_immediate(displacement: Immediate, prefixes: &Prefixes) -> MemoryIndex {
        let sr = match prefixes.sr_override {
            Some(seg_reg) => seg_reg,
            None => SegmentRegister::DS,
        };

        MemoryIndex {
            base: None,
            index: None,
            sr,
            displacement: Some(displacement),
        }
    }
}

impl fmt::Display for MemoryIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}:{}{}{}]",
            self.sr,
            match self.base {
                Some(base) => base.to_string(),
                None => String::new(),
            },
            match self.index {
                Some(index) => format!("+{}", index),
                None => String::new(),
            },
            match self.displacement {
                Some(displacement) => {
                    if self.base.is_none() && self.index.is_none() {
                        // The displacement is not being added to or
                        // subtracted from another value
                        format!("{:X}h", displacement)
                    } else {
                        match displacement {
                            Immediate::Byte(byte) => {
                                let signed_value = byte as i8;

                                format!(
                                    "{}{:X}h",
                                    if signed_value < 0 { "-" } else { "+" },
                                    signed_value.wrapping_abs(),
                                )
                            }
                            Immediate::Word(word) => {
                                let signed_value = word as i16;

                                format!(
                                    "{}{:X}h",
                                    if signed_value < 0 { "-" } else { "+" },
                                    signed_value.wrapping_abs(),
                                )
                            }
                        }
                    }
                }
                None => String::new(),
            },
        )
    }
}
