use std::fmt::UpperHex;

use crate::immediate::ImmedGroupImmediate16;
use crate::immediate::Immediate8;
use crate::immediate::Immediate16;
use crate::immediate_parsing::ParseImmediate;
use crate::register::GeneralRegister;
use crate::register::GeneralRegister8;
use crate::register::GeneralRegister16;

pub trait OpWidth {
    type Register: GeneralRegister;
    type Immediate: ParseImmediate;
    type ImmedGroupImmediate: UpperHex;
    const MEMORY_SIZE: &str;
}

#[derive(Debug)]
pub struct Width8;

#[derive(Debug)]
pub struct Width16;

impl OpWidth for Width8 {
    type Register = GeneralRegister8;
    type Immediate = Immediate8;
    type ImmedGroupImmediate = Immediate8;
    const MEMORY_SIZE: &str = "byte";
}

impl OpWidth for Width16 {
    type Register = GeneralRegister16;
    type Immediate = Immediate16;
    type ImmedGroupImmediate = ImmedGroupImmediate16;
    const MEMORY_SIZE: &str = "word";
}
