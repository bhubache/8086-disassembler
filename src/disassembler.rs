use std::fs;

use crate::immediate::ImmedGroupImmediate16;
use crate::immediate_parsing::ByteReader;
use crate::immediate_parsing::ParseImmediate;
use crate::instruction::Instruction;
use crate::instruction::Opcode;
use crate::instruction::Operation;
use crate::instruction::RepeatableStringInstruction;
use crate::mode::InvalidModeEncoding;
use crate::mode::Mode;
use crate::op_8;
use crate::op_16;
use crate::operand::Immediate;
use crate::operand::MemoryIndex;
use crate::operand::ModRm;
use crate::parse_mod_reg_rm_8_from_reg;
use crate::parse_mod_reg_rm_8_to_reg;
use crate::parse_mod_reg_rm_16_from_reg;
use crate::parse_mod_reg_rm_16_to_reg;
use crate::prefixes::Prefixes;
use crate::register::GeneralRegister8;
use crate::register::GeneralRegister16;
use crate::register::InvalidRegEncoding;
use crate::register::RegCode;
use crate::register::SegmentRegister;
use crate::rm::InvalidRmEncoding;
use crate::rm::RmCode;
use crate::width::OpWidth;
use crate::width::Width8;
use crate::width::Width16;

#[derive(Debug)]
pub enum DisassemblerError {
    InvalidMode(InvalidModeEncoding),
    InvalidRm(InvalidRmEncoding),
    InvalidReg(InvalidRegEncoding),
    InvalidOpcodeExtension(u8),
    InvalidRepOperand(u8),
    EOF,
}

impl std::fmt::Display for DisassemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidMode(error) => write!(f, "{}", error),
            Self::InvalidRm(error) => write!(f, "{}", error),
            Self::InvalidReg(error) => write!(f, "{}", error),
            Self::InvalidOpcodeExtension(value) => {
                write!(f, "invalid opcode extension `{:03b}`", value)
            }
            Self::InvalidRepOperand(value) => write!(f, "invalid rep operand `{:02X}`", value),
            Self::EOF => write!(f, "unexpectedly reached EOF"),
        }
    }
}

impl From<InvalidModeEncoding> for DisassemblerError {
    fn from(value: InvalidModeEncoding) -> Self {
        Self::InvalidMode(value)
    }
}

impl From<InvalidRmEncoding> for DisassemblerError {
    fn from(value: InvalidRmEncoding) -> Self {
        Self::InvalidRm(value)
    }
}

impl From<InvalidRegEncoding> for DisassemblerError {
    fn from(value: InvalidRegEncoding) -> Self {
        Self::InvalidReg(value)
    }
}

// TODO: Impl methods
impl std::error::Error for DisassemblerError {}

pub struct Disassembler {
    bytes: Vec<u8>,
    index: usize,
    instructions: Vec<Instruction>,
}

impl Disassembler {
    pub fn from_file(path: &str) -> Result<Disassembler, std::io::Error> {
        let bytes: Vec<u8> = fs::read(path)?;

        Ok(Disassembler::from_bytes(bytes))
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Disassembler {
        Disassembler {
            bytes,
            index: 0,
            instructions: Vec::new(),
        }
    }

    pub fn dump_operations(&self) -> String {
        self.instructions
            .iter()
            .map(|inst| inst.operation.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn dump(&self) -> String {
        self.instructions
            .iter()
            .map(|inst| inst.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn disassemble(&mut self) -> Result<(), DisassemblerError> {
        while self.index < self.bytes.len() {
            let address = self.index;
            let mut prefixes = Prefixes::new();
            let opcode_byte = self.read_byte()?;

            let maybe_operation = self.parse_operation(opcode_byte, &mut prefixes);

            match maybe_operation {
                Ok(operation) => {
                    let instruction = Instruction::new(address, operation);

                    self.instructions.push(instruction);
                }
                Err(err) => {
                    // TODO: Add context
                    println!("Encountered an error during disassembly: {err}");

                    return Err(err);
                }
            }
        }

        Ok(())
    }

    fn parse_operation(
        &mut self,
        opcode_byte: u8,
        prefixes: &mut Prefixes,
    ) -> Result<Operation, DisassemblerError> {
        let operation = match opcode_byte {
            0x00 => parse_mod_reg_rm_8_from_reg!(self, Opcode::AddFromReg, prefixes),
            0x01 => parse_mod_reg_rm_16_from_reg!(self, Opcode::AddFromReg, prefixes),
            0x02 => parse_mod_reg_rm_8_to_reg!(self, Opcode::AddToReg, prefixes),
            0x03 => parse_mod_reg_rm_16_to_reg!(self, Opcode::AddToReg, prefixes),
            0x04 => op_8!(Opcode::AddToALFromImmed8(self.read_byte()?.into())),
            0x05 => op_16!(Opcode::AddToAXFromImmed16(self.read_word()?.into())),
            0x06 => op_16!(Opcode::PushSR(SegmentRegister::ES)),
            0x07 => op_16!(Opcode::PopSR(SegmentRegister::ES)),
            0x08 => parse_mod_reg_rm_8_from_reg!(self, Opcode::OrFromReg, prefixes),
            0x09 => parse_mod_reg_rm_16_from_reg!(self, Opcode::OrFromReg, prefixes),
            0x0A => parse_mod_reg_rm_8_to_reg!(self, Opcode::OrToReg, prefixes),
            0x0B => parse_mod_reg_rm_16_to_reg!(self, Opcode::OrToReg, prefixes),
            0x0C => op_8!(Opcode::OrToALFromImmed8(self.read_byte()?.into())),
            0x0D => op_16!(Opcode::OrToAXFromImmed16(self.read_word()?.into())),
            0x0E => op_16!(Opcode::PushSR(SegmentRegister::CS)),
            0x0F => todo!(),
            0x10 => parse_mod_reg_rm_8_from_reg!(self, Opcode::AdcFromReg, prefixes),
            0x11 => parse_mod_reg_rm_16_from_reg!(self, Opcode::AdcFromReg, prefixes),
            0x12 => parse_mod_reg_rm_8_to_reg!(self, Opcode::AdcToReg, prefixes),
            0x13 => parse_mod_reg_rm_16_to_reg!(self, Opcode::AdcToReg, prefixes),
            0x14 => op_8!(Opcode::AdcToALFromImmed8(self.read_byte()?.into())),
            0x15 => op_16!(Opcode::AdcToAXFromImmed16(self.read_word()?.into())),
            0x16 => op_16!(Opcode::PushSR(SegmentRegister::SS)),
            0x17 => op_16!(Opcode::PopSR(SegmentRegister::SS)),
            0x18 => parse_mod_reg_rm_8_from_reg!(self, Opcode::SbbFromReg, prefixes),
            0x19 => parse_mod_reg_rm_16_from_reg!(self, Opcode::SbbFromReg, prefixes),
            0x1A => parse_mod_reg_rm_8_to_reg!(self, Opcode::SbbToReg, prefixes),
            0x1B => parse_mod_reg_rm_16_to_reg!(self, Opcode::SbbToReg, prefixes),
            0x1C => op_8!(Opcode::SbbToALFromImmed8(self.read_byte()?.into())),
            0x1D => op_16!(Opcode::SbbToAXFromImmed16(self.read_word()?.into())),
            0x1E => op_16!(Opcode::PushSR(SegmentRegister::DS)),
            0x1F => op_16!(Opcode::PopSR(SegmentRegister::DS)),
            0x20 => parse_mod_reg_rm_8_from_reg!(self, Opcode::AndFromReg, prefixes),
            0x21 => parse_mod_reg_rm_16_from_reg!(self, Opcode::AndFromReg, prefixes),
            0x22 => parse_mod_reg_rm_8_to_reg!(self, Opcode::AndToReg, prefixes),
            0x23 => parse_mod_reg_rm_16_to_reg!(self, Opcode::AndToReg, prefixes),
            0x24 => op_8!(Opcode::AndToALFromImmed8(self.read_byte()?.into())),
            0x25 => op_16!(Opcode::AndToAXFromImmed16(self.read_word()?.into())),
            0x26 => {
                prefixes.sr_override = Some(SegmentRegister::ES);
                let byte = self.read_byte()?;
                self.parse_operation(byte, prefixes)?
            }
            0x27 => op_8!(Opcode::Daa),
            0x28 => parse_mod_reg_rm_8_from_reg!(self, Opcode::SubFromReg, prefixes),
            0x29 => parse_mod_reg_rm_16_from_reg!(self, Opcode::SubFromReg, prefixes),
            0x2A => parse_mod_reg_rm_8_to_reg!(self, Opcode::SubToReg, prefixes),
            0x2B => parse_mod_reg_rm_16_to_reg!(self, Opcode::SubToReg, prefixes),
            0x2C => op_8!(Opcode::SubToALFromImmed8(self.read_byte()?.into())),
            0x2D => op_16!(Opcode::SubToAXFromImmed16(self.read_word()?.into())),
            0x2E => {
                prefixes.sr_override = Some(SegmentRegister::CS);
                let byte = self.read_byte()?;
                self.parse_operation(byte, prefixes)?
            }
            0x2F => op_8!(Opcode::Das),
            0x30 => parse_mod_reg_rm_8_from_reg!(self, Opcode::XorFromReg, prefixes),
            0x31 => parse_mod_reg_rm_16_from_reg!(self, Opcode::XorFromReg, prefixes),
            0x32 => parse_mod_reg_rm_8_to_reg!(self, Opcode::XorToReg, prefixes),
            0x33 => parse_mod_reg_rm_16_to_reg!(self, Opcode::XorToReg, prefixes),
            0x34 => op_8!(Opcode::XorToALFromImmed8(self.read_byte()?.into())),
            0x35 => op_16!(Opcode::XorToAXFromImmed16(self.read_word()?.into())),
            0x36 => {
                prefixes.sr_override = Some(SegmentRegister::SS);
                let byte = self.read_byte()?;
                self.parse_operation(byte, prefixes)?
            }
            0x37 => op_16!(Opcode::Aaa),
            0x38 => parse_mod_reg_rm_8_from_reg!(self, Opcode::CmpFromReg, prefixes),
            0x39 => parse_mod_reg_rm_16_from_reg!(self, Opcode::CmpFromReg, prefixes),
            0x3A => parse_mod_reg_rm_8_to_reg!(self, Opcode::CmpToReg, prefixes),
            0x3B => parse_mod_reg_rm_16_to_reg!(self, Opcode::CmpToReg, prefixes),
            0x3C => op_8!(Opcode::CmpToALFromImmed8(self.read_byte()?.into())),
            0x3D => op_16!(Opcode::CmpToAXFromImmed16(self.read_word()?.into())),
            0x3E => {
                prefixes.sr_override = Some(SegmentRegister::DS);
                let byte = self.read_byte()?;
                self.parse_operation(byte, prefixes)?
            }
            0x3F => op_16!(Opcode::Aas),
            0x40 => op_16!(Opcode::IncGR16(GeneralRegister16::AX)),
            0x41 => op_16!(Opcode::IncGR16(GeneralRegister16::CX)),
            0x42 => op_16!(Opcode::IncGR16(GeneralRegister16::DX)),
            0x43 => op_16!(Opcode::IncGR16(GeneralRegister16::BX)),
            0x44 => op_16!(Opcode::IncGR16(GeneralRegister16::SP)),
            0x45 => op_16!(Opcode::IncGR16(GeneralRegister16::BP)),
            0x46 => op_16!(Opcode::IncGR16(GeneralRegister16::SI)),
            0x47 => op_16!(Opcode::IncGR16(GeneralRegister16::DI)),
            0x48 => op_16!(Opcode::DecGR16(GeneralRegister16::AX)),
            0x49 => op_16!(Opcode::DecGR16(GeneralRegister16::CX)),
            0x4A => op_16!(Opcode::DecGR16(GeneralRegister16::DX)),
            0x4B => op_16!(Opcode::DecGR16(GeneralRegister16::BX)),
            0x4C => op_16!(Opcode::DecGR16(GeneralRegister16::SP)),
            0x4D => op_16!(Opcode::DecGR16(GeneralRegister16::BP)),
            0x4E => op_16!(Opcode::DecGR16(GeneralRegister16::SI)),
            0x4F => op_16!(Opcode::DecGR16(GeneralRegister16::DI)),
            0x50 => op_16!(Opcode::PushGR16(GeneralRegister16::AX)),
            0x51 => op_16!(Opcode::PushGR16(GeneralRegister16::CX)),
            0x52 => op_16!(Opcode::PushGR16(GeneralRegister16::DX)),
            0x53 => op_16!(Opcode::PushGR16(GeneralRegister16::BX)),
            0x54 => op_16!(Opcode::PushGR16(GeneralRegister16::SP)),
            0x55 => op_16!(Opcode::PushGR16(GeneralRegister16::BP)),
            0x56 => op_16!(Opcode::PushGR16(GeneralRegister16::SI)),
            0x57 => op_16!(Opcode::PushGR16(GeneralRegister16::DI)),
            0x58 => op_16!(Opcode::PopGR16(GeneralRegister16::AX)),
            0x59 => op_16!(Opcode::PopGR16(GeneralRegister16::CX)),
            0x5A => op_16!(Opcode::PopGR16(GeneralRegister16::DX)),
            0x5B => op_16!(Opcode::PopGR16(GeneralRegister16::BX)),
            0x5C => op_16!(Opcode::PopGR16(GeneralRegister16::SP)),
            0x5D => op_16!(Opcode::PopGR16(GeneralRegister16::BP)),
            0x5E => op_16!(Opcode::PopGR16(GeneralRegister16::SI)),
            0x5F => op_16!(Opcode::PopGR16(GeneralRegister16::DI)),

            // 0x60 - 0x6F are documented as unused but there exist hardware generated tests
            // for them
            0x60 => op_16!(Opcode::Jo(self.parse_short_label()?)),
            0x61 => op_16!(Opcode::Jno(self.parse_short_label()?)),
            0x62 => op_16!(Opcode::Jb(self.parse_short_label()?)),
            0x63 => op_16!(Opcode::Jnb(self.parse_short_label()?)),
            0x64 => op_16!(Opcode::Jz(self.parse_short_label()?)),
            0x65 => op_16!(Opcode::Jnz(self.parse_short_label()?)),
            0x66 => op_16!(Opcode::Jbe(self.parse_short_label()?)),
            0x67 => op_16!(Opcode::Jnbe(self.parse_short_label()?)),
            0x68 => op_16!(Opcode::Js(self.parse_short_label()?)),
            0x69 => op_16!(Opcode::Jns(self.parse_short_label()?)),
            0x6A => op_16!(Opcode::Jp(self.parse_short_label()?)),
            0x6B => op_16!(Opcode::Jnp(self.parse_short_label()?)),
            0x6C => op_16!(Opcode::Jl(self.parse_short_label()?)),
            0x6D => op_16!(Opcode::Jnl(self.parse_short_label()?)),
            0x6E => op_16!(Opcode::Jle(self.parse_short_label()?)),
            0x6F => op_16!(Opcode::Jnle(self.parse_short_label()?)),

            0x70 => op_16!(Opcode::Jo(self.parse_short_label()?)),
            0x71 => op_16!(Opcode::Jno(self.parse_short_label()?)),
            0x72 => op_16!(Opcode::Jb(self.parse_short_label()?)),
            0x73 => op_16!(Opcode::Jnb(self.parse_short_label()?)),
            0x74 => op_16!(Opcode::Jz(self.parse_short_label()?)),
            0x75 => op_16!(Opcode::Jnz(self.parse_short_label()?)),
            0x76 => op_16!(Opcode::Jbe(self.parse_short_label()?)),
            0x77 => op_16!(Opcode::Jnbe(self.parse_short_label()?)),
            0x78 => op_16!(Opcode::Js(self.parse_short_label()?)),
            0x79 => op_16!(Opcode::Jns(self.parse_short_label()?)),
            0x7A => op_16!(Opcode::Jp(self.parse_short_label()?)),
            0x7B => op_16!(Opcode::Jnp(self.parse_short_label()?)),
            0x7C => op_16!(Opcode::Jl(self.parse_short_label()?)),
            0x7D => op_16!(Opcode::Jnl(self.parse_short_label()?)),
            0x7E => op_16!(Opcode::Jle(self.parse_short_label()?)),
            0x7F => op_16!(Opcode::Jnle(self.parse_short_label()?)),
            0x80 | 0x82 => {
                // TODO: Make Group<num>Code for each encoding to encapsulate the bit shifting?
                let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                let (mod_rm, _) = self.parse_mod_reg_rm(prefixes)?;
                let immed = self.read_byte()?.into();

                op_8!(Self::create_modrm_with_reg_mnemonic_encoding_immed(
                    mnemonic_encoding,
                    mod_rm,
                    immed,
                )?)
            }
            0x81 => {
                let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                let (mod_rm, _) = self.parse_mod_reg_rm(prefixes)?;
                let immed = ImmedGroupImmediate16::Full(self.read_word()?.into());

                op_16!(Self::create_modrm_with_reg_mnemonic_encoding_immed(
                    mnemonic_encoding,
                    mod_rm,
                    immed,
                )?)
            }
            0x83 => {
                let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                let (mod_rm, _) = self.parse_mod_reg_rm(prefixes)?;
                let immed = ImmedGroupImmediate16::SignExtended8(self.read_byte()?.into());

                op_16!(Self::create_modrm_with_reg_mnemonic_encoding_immed(
                    mnemonic_encoding,
                    mod_rm,
                    immed,
                )?)
            }
            0x84 => parse_mod_reg_rm_8_from_reg!(self, Opcode::TestFromReg, prefixes),
            0x85 => parse_mod_reg_rm_16_from_reg!(self, Opcode::TestFromReg, prefixes),
            0x86 => parse_mod_reg_rm_8_to_reg!(self, Opcode::XchgToReg, prefixes),
            0x87 => parse_mod_reg_rm_16_to_reg!(self, Opcode::XchgToReg, prefixes),
            0x88 => parse_mod_reg_rm_8_from_reg!(self, Opcode::MovFromReg, prefixes),
            0x89 => parse_mod_reg_rm_16_from_reg!(self, Opcode::MovFromReg, prefixes),
            0x8A => parse_mod_reg_rm_8_to_reg!(self, Opcode::MovToReg, prefixes),
            0x8B => parse_mod_reg_rm_16_to_reg!(self, Opcode::MovToReg, prefixes),
            0x8C => {
                let (mod_rm, sr) = self.parse_mod_sr_rm(prefixes)?;
                op_16!(Opcode::MovFromSR(mod_rm, sr))
            }
            0x8D => {
                let (fst, snd) = self.parse_mod_rm_as_mem_index(prefixes)?;
                op_16!(Opcode::LeaToGR16(fst, snd))
            }
            0x8E => {
                let (mod_rm, sr) = self.parse_mod_sr_rm(prefixes)?;
                op_16!(Opcode::MovToSR(sr, mod_rm))
            }
            0x8F => {
                let (mod_rm, _) = self.parse_mod_reg_rm(prefixes)?;

                op_16!(Opcode::PopModRm(mod_rm))
            }
            0x90 => op_8!(Opcode::Nop),
            0x91 => op_8!(Opcode::XchgToAXFromCX),
            0x92 => op_8!(Opcode::XchgToAXFromDX),
            0x93 => op_8!(Opcode::XchgToAXFromBX),
            0x94 => op_8!(Opcode::XchgToAXFromSP),
            0x95 => op_8!(Opcode::XchgToAXFromBP),
            0x96 => op_8!(Opcode::XchgToAXFromSI),
            0x97 => op_8!(Opcode::XchgToAXFromDI),
            0x98 => op_8!(Opcode::Cbw),
            0x99 => op_8!(Opcode::Cwd),
            0x9A => {
                let displacement = self.read_word()?;
                let segment = self.read_word()?;
                op_16!(Opcode::CallFarProc(format!(
                    "{:04X}h:{:04X}h",
                    segment, displacement
                )))
            }
            0x9B => op_8!(Opcode::Wait),
            0x9C => op_8!(Opcode::PushF),
            0x9D => op_8!(Opcode::PopF),
            0x9E => op_8!(Opcode::SahF),
            0x9F => op_8!(Opcode::LahF),

            // TODO: These should use the new immediate types
            0xA0 => op_16!(Opcode::MovToALFromMem8(MemoryIndex::with_immediate(
                Immediate::Word(self.read_word()?),
                prefixes,
            ))),
            0xA1 => op_8!(Opcode::MovToAXFromMem16(MemoryIndex::with_immediate(
                Immediate::Word(self.read_word()?),
                prefixes,
            ))),
            0xA2 => op_8!(Opcode::MovToMem8FromAL(MemoryIndex::with_immediate(
                Immediate::Word(self.read_word()?),
                prefixes,
            ))),
            0xA3 => op_8!(Opcode::MovToMem16FromAL(MemoryIndex::with_immediate(
                Immediate::Word(self.read_word()?),
                prefixes,
            ))),
            0xA4 => op_8!(Opcode::MovS8(prefixes.sr_override)),
            0xA5 => op_16!(Opcode::MovS16(prefixes.sr_override)),
            0xA6 => op_8!(Opcode::CmpS8(prefixes.sr_override)),
            0xA7 => op_16!(Opcode::CmpS16(prefixes.sr_override)),
            0xA8 => op_8!(Opcode::TestToALFromImmed8(self.read_byte()?.into())),
            0xA9 => op_16!(Opcode::TestToAXFromImmed16(self.read_word()?.into())),
            0xAA => op_8!(Opcode::StoS8(prefixes.sr_override)),
            0xAB => op_16!(Opcode::StoS16(prefixes.sr_override)),
            0xAC => op_8!(Opcode::LodS8(prefixes.sr_override)),
            0xAD => op_16!(Opcode::LodS16(prefixes.sr_override)),
            0xAE => op_8!(Opcode::ScaS8(prefixes.sr_override)),
            0xAF => op_16!(Opcode::ScaS16(prefixes.sr_override)),
            0xB0 => op_8!(Opcode::MovToGRFromImmed(
                GeneralRegister8::AL,
                self.read_byte()?.into()
            )),
            0xB1 => op_8!(Opcode::MovToGRFromImmed(
                GeneralRegister8::CL,
                self.read_byte()?.into()
            )),
            0xB2 => op_8!(Opcode::MovToGRFromImmed(
                GeneralRegister8::DL,
                self.read_byte()?.into()
            )),
            0xB3 => op_8!(Opcode::MovToGRFromImmed(
                GeneralRegister8::BL,
                self.read_byte()?.into()
            )),
            0xB4 => op_8!(Opcode::MovToGRFromImmed(
                GeneralRegister8::AH,
                self.read_byte()?.into()
            )),
            0xB5 => op_8!(Opcode::MovToGRFromImmed(
                GeneralRegister8::CH,
                self.read_byte()?.into()
            )),
            0xB6 => op_8!(Opcode::MovToGRFromImmed(
                GeneralRegister8::DH,
                self.read_byte()?.into()
            )),
            0xB7 => op_8!(Opcode::MovToGRFromImmed(
                GeneralRegister8::BH,
                self.read_byte()?.into()
            )),
            0xB8 => op_16!(Opcode::MovToGRFromImmed(
                GeneralRegister16::AX,
                self.read_word()?.into()
            )),
            0xB9 => op_16!(Opcode::MovToGRFromImmed(
                GeneralRegister16::CX,
                self.read_word()?.into()
            )),
            0xBA => op_16!(Opcode::MovToGRFromImmed(
                GeneralRegister16::DX,
                self.read_word()?.into()
            )),
            0xBB => op_16!(Opcode::MovToGRFromImmed(
                GeneralRegister16::BX,
                self.read_word()?.into()
            )),
            0xBC => op_16!(Opcode::MovToGRFromImmed(
                GeneralRegister16::SP,
                self.read_word()?.into()
            )),
            0xBD => op_16!(Opcode::MovToGRFromImmed(
                GeneralRegister16::BP,
                self.read_word()?.into()
            )),
            0xBE => op_16!(Opcode::MovToGRFromImmed(
                GeneralRegister16::SI,
                self.read_word()?.into()
            )),
            0xBF => op_16!(Opcode::MovToGRFromImmed(
                GeneralRegister16::DI,
                self.read_word()?.into()
            )),
            0xC0 => op_16!(Opcode::RetIntraSegImmed16(self.read_word()?.into())),
            0xC1 => op_8!(Opcode::RetIntraSeg),
            0xC2 => op_16!(Opcode::RetIntraSegImmed16(self.read_word()?.into())),
            0xC3 => op_8!(Opcode::RetIntraSeg),
            0xC4 => {
                let (fst, snd) = self.parse_mod_rm_as_mem_index(prefixes)?;
                op_8!(Opcode::LesToReg(fst, snd))
            }
            0xC5 => {
                let (fst, snd) = self.parse_mod_rm_as_mem_index(prefixes)?;
                op_8!(Opcode::LdsToReg(fst, snd))
            }
            0xC6 => {
                let (mod_rm, _) = self.parse_mod_reg_rm(prefixes)?;
                op_8!(Opcode::MovToMemFromImmed(mod_rm, self.read_byte()?.into()))
            }
            0xC7 => {
                let (mod_rm, _) = self.parse_mod_reg_rm(prefixes)?;
                op_16!(Opcode::MovToMemFromImmed(mod_rm, self.read_word()?.into()))
            }
            0xC8 => op_16!(Opcode::RetInterSegImmed16(self.read_word()?.into())),
            0xC9 => op_8!(Opcode::RetInterSeg),
            0xCA => op_16!(Opcode::RetInterSegImmed16(self.read_word()?.into())),
            0xCB => op_8!(Opcode::RetInterSeg),
            0xCC => op_8!(Opcode::Int3),
            0xCD => op_8!(Opcode::IntFromImmed8(self.read_byte()?.into())),
            0xCE => op_8!(Opcode::Into),
            0xCF => op_8!(Opcode::Iret),
            0xD0 => {
                let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                let (mod_rm, _) = self.parse_mod_reg_rm(prefixes)?;

                op_8!(Self::create_modrm_with_reg_mnemonic_encoding_shift(
                    mnemonic_encoding,
                    mod_rm
                )?)
            }
            0xD1 => {
                let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                let (mod_rm, _) = self.parse_mod_reg_rm(prefixes)?;

                op_16!(Self::create_modrm_with_reg_mnemonic_encoding_shift(
                    mnemonic_encoding,
                    mod_rm
                )?)
            }
            0xD2 => {
                let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                let (mod_rm, _) = self.parse_mod_reg_rm(prefixes)?;

                op_8!(Self::create_modrm_with_reg_mnemonic_encoding_shift_cl(
                    mnemonic_encoding,
                    mod_rm
                )?)
            }
            0xD3 => {
                let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                let (mod_rm, _) = self.parse_mod_reg_rm(prefixes)?;

                op_16!(Self::create_modrm_with_reg_mnemonic_encoding_shift_cl(
                    mnemonic_encoding,
                    mod_rm
                )?)
            }
            0xD4 => op_8!(Opcode::Aam(self.read_byte()?.into())),
            0xD5 => op_8!(Opcode::Aad(self.read_byte()?.into())),
            0xD6 => op_8!(Opcode::Salc),
            0xD7 => op_8!(Opcode::Xlat),
            0xD8..=0xDF => {
                let (mod_rm, _) = self.parse_mod_reg_rm(prefixes)?;
                op_8!(Opcode::Esc(mod_rm))
            }
            0xE0 => op_16!(Opcode::Loopne(self.parse_short_label()?)),
            0xE1 => op_16!(Opcode::Loope(self.parse_short_label()?)),
            0xE2 => op_16!(Opcode::Loop(self.parse_short_label()?)),
            0xE3 => op_16!(Opcode::Jcxz(self.parse_short_label()?)),
            0xE4 => op_8!(Opcode::InToALFromImmed8(self.read_byte()?.into())),
            0xE5 => op_8!(Opcode::InToAXFromImmed8(self.read_byte()?.into())),
            0xE6 => op_8!(Opcode::OutToPort8FromAL(self.read_byte()?.into())),
            0xE7 => op_8!(Opcode::OutToPort8FromAX(self.read_byte()?.into())),
            0xE8 => op_16!(Opcode::CallNearProc(format!(
                "{:04X}h",
                self.parse_jump_word()?
            ))),
            0xE9 => op_16!(Opcode::JmpNearLabel(format!(
                "{:04X}h",
                self.parse_jump_word()?
            ))),
            0xEA => {
                let displacement = self.read_word()?;
                let segment = self.read_word()?;
                op_16!(Opcode::JmpFarLabel(format!(
                    "{:04X}h:{:04X}h",
                    segment, displacement
                )))
            }
            0xEB => op_16!(Opcode::JmpShortLabel(self.parse_short_label()?)),
            0xEC => op_8!(Opcode::InToALFromDX),
            0xED => op_8!(Opcode::InToAXFromDX),
            0xEE => op_8!(Opcode::OutToDXFromAL),
            0xEF => op_8!(Opcode::OutToDXFromAX),
            0xF0 => op_8!(Opcode::Lock),
            0xF1 => todo!(),
            0xF2 => op_8!(Opcode::Repne(prefixes.sr_override, self.parse_rep_op()?)),
            0xF3 => op_8!(Opcode::Rep(prefixes.sr_override, self.parse_rep_op()?)),
            0xF4 => op_8!(Opcode::Hlt),
            0xF5 => op_8!(Opcode::Cmc),
            0xF6 => {
                let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                let (mod_rm, _) = self.parse_mod_reg_rm(prefixes)?;

                op_8!(
                    self.create_modrm_with_reg_mnemonic_encoding_group1(mnemonic_encoding, mod_rm)?
                )
            }
            0xF7 => {
                let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                let (mod_rm, _) = self.parse_mod_reg_rm(prefixes)?;

                op_16!(
                    self.create_modrm_with_reg_mnemonic_encoding_group1(mnemonic_encoding, mod_rm)?
                )
            }
            0xF8 => op_8!(Opcode::Clc),
            0xF9 => op_8!(Opcode::Stc),
            0xFA => op_8!(Opcode::Cli),
            0xFB => op_8!(Opcode::Sti),
            0xFC => op_8!(Opcode::Cld),
            0xFD => op_8!(Opcode::Std),
            0xFE => {
                let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                let (mod_rm, _) = self.parse_mod_reg_rm(prefixes)?;

                op_8!(Self::create_modrm_with_reg_mnemonic_encoding_group2_8bit(
                    mnemonic_encoding,
                    mod_rm
                )?)
            }
            0xFF => {
                let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                let (mod_rm, _) = self.parse_mod_reg_rm(prefixes)?;

                op_16!(Self::create_modrm_with_reg_mnemonic_encoding_group2_16bit(
                    mnemonic_encoding,
                    mod_rm
                )?)
            }
        };

        Ok(operation)
    }

    fn parse_mod_rm_as_mem_index(
        &mut self,
        prefixes: &mut Prefixes,
    ) -> Result<(GeneralRegister16, MemoryIndex), DisassemblerError> {
        let (mod_rm, dst_reg) = self.parse_mod_reg_rm(prefixes)?;
        // FIXME: Is it appropriate to panic here?
        match mod_rm {
            ModRm::<Width16>::Register(reg) => panic!(
                "unexpectedly parsed MOD/RM as register `{}` rather than a memory index",
                reg
            ),
            ModRm::<Width16>::EffectiveAddr(mem_index) => Ok((dst_reg, mem_index)),
        }
    }

    fn parse_rep_op(&mut self) -> Result<RepeatableStringInstruction, DisassemblerError> {
        let rsi = match self.read_byte()? {
            0xA4 => RepeatableStringInstruction::Movsb,
            0xA5 => RepeatableStringInstruction::Movsw,
            0xA6 => RepeatableStringInstruction::Cmpsb,
            0xA7 => RepeatableStringInstruction::Cmpsw,
            0xAA => RepeatableStringInstruction::Stosb,
            0xAB => RepeatableStringInstruction::Stosw,
            0xAC => RepeatableStringInstruction::Lodsb,
            0xAD => RepeatableStringInstruction::Lodsw,
            0xAE => RepeatableStringInstruction::Scasb,
            0xAF => RepeatableStringInstruction::Scasw,
            op => return Err(DisassemblerError::InvalidRepOperand(op)),
        };

        Ok(rsi)
    }

    fn create_modrm_with_reg_mnemonic_encoding_group2_8bit(
        mnemonic_encoding: u8,
        mod_rm: ModRm<Width8>,
    ) -> Result<Opcode<Width8>, DisassemblerError> {
        let opcode = match mnemonic_encoding {
            0b000 => Opcode::IncModRm(mod_rm),
            0b001 => Opcode::DecModRm(mod_rm),
            0b010 => todo!(),
            0b011 => todo!(),
            0b100 => todo!(),
            0b101 => todo!(),
            0b110 => todo!(),
            0b111 => todo!(),
            _ => return Err(DisassemblerError::InvalidOpcodeExtension(mnemonic_encoding)),
        };

        Ok(opcode)
    }

    fn create_modrm_with_reg_mnemonic_encoding_group2_16bit(
        mnemonic_encoding: u8,
        mod_rm: ModRm<Width16>,
    ) -> Result<Opcode<Width16>, DisassemblerError> {
        let opcode = match mnemonic_encoding {
            0b000 => Opcode::IncModRm(mod_rm),
            0b001 => Opcode::DecModRm(mod_rm),
            0b010 => Opcode::CallModRm16(mod_rm),
            0b011 => match mod_rm {
                ModRm::Register(_) => panic!("Uhoh!!"),
                ModRm::EffectiveAddr(mem_index) => Opcode::CallMem16(mem_index),
            },
            0b100 => Opcode::JmpModRm16(mod_rm),
            0b101 => match mod_rm {
                ModRm::Register(_) => panic!("Uhoh!!"),
                ModRm::EffectiveAddr(mem_index) => Opcode::JmpMem16(mem_index),
            },
            0b110 => Opcode::PushModRm16(mod_rm),
            0b111 => Opcode::PushModRm16(mod_rm),
            _ => return Err(DisassemblerError::InvalidOpcodeExtension(mnemonic_encoding)),
        };

        Ok(opcode)
    }

    fn create_modrm_with_reg_mnemonic_encoding_group1<W: OpWidth>(
        &mut self,
        mnemonic_encoding: u8,
        mod_rm: ModRm<W>,
    ) -> Result<Opcode<W>, DisassemblerError> {
        let mnemonic = match mnemonic_encoding {
            0b000 => Opcode::TestToModRmFromImmed(mod_rm, W::Immediate::parse_immediate(self)?),
            0b001 => Opcode::TestToModRmFromImmed(mod_rm, W::Immediate::parse_immediate(self)?),
            0b010 => Opcode::NotToModRm(mod_rm),
            0b011 => Opcode::NegToModRm(mod_rm),
            0b100 => Opcode::MulToModRm(mod_rm),
            0b101 => Opcode::ImulToModRm(mod_rm),
            0b110 => Opcode::DivToModRm(mod_rm),
            0b111 => Opcode::IdivToModRm(mod_rm),
            _ => return Err(DisassemblerError::InvalidOpcodeExtension(mnemonic_encoding)),
        };

        Ok(mnemonic)
    }

    fn create_modrm_with_reg_mnemonic_encoding_shift<W: OpWidth>(
        mnemonic_encoding: u8,
        mod_rm: ModRm<W>,
    ) -> Result<Opcode<W>, DisassemblerError> {
        let mnemonic = match mnemonic_encoding {
            0b000 => Opcode::RolToModRm(mod_rm),
            0b001 => Opcode::RorToModRm(mod_rm),
            0b010 => Opcode::RclToModRm(mod_rm),
            0b011 => Opcode::RcrToModRm(mod_rm),
            0b100 => Opcode::ShlToModRm(mod_rm),
            0b101 => Opcode::ShrToModRm(mod_rm),
            0b110 => Opcode::SetmoToModRm(mod_rm),
            0b111 => Opcode::SarToModRm(mod_rm),
            _ => return Err(DisassemblerError::InvalidOpcodeExtension(mnemonic_encoding)),
        };

        Ok(mnemonic)
    }

    fn create_modrm_with_reg_mnemonic_encoding_shift_cl<W: OpWidth>(
        mnemonic_encoding: u8,
        mod_rm: ModRm<W>,
    ) -> Result<Opcode<W>, DisassemblerError> {
        let mnemonic = match mnemonic_encoding {
            0b000 => Opcode::RolToModRmCL(mod_rm),
            0b001 => Opcode::RorToModRmCL(mod_rm),
            0b010 => Opcode::RclToModRmCL(mod_rm),
            0b011 => Opcode::RcrToModRmCL(mod_rm),
            0b100 => Opcode::ShlToModRmCL(mod_rm),
            0b101 => Opcode::ShrToModRmCL(mod_rm),
            0b110 => Opcode::SetmoToModRmCL(mod_rm),
            0b111 => Opcode::SarToModRmCL(mod_rm),
            _ => return Err(DisassemblerError::InvalidOpcodeExtension(mnemonic_encoding)),
        };

        Ok(mnemonic)
    }

    fn create_modrm_with_reg_mnemonic_encoding_immed<W: OpWidth>(
        mnemonic_encoding: u8,
        mod_rm: ModRm<W>,
        immed: W::ImmedGroupImmediate,
    ) -> Result<Opcode<W>, DisassemblerError> {
        let mnemonic = match mnemonic_encoding {
            0b000 => Opcode::AddToModRmFromImmed(mod_rm, immed),
            0b001 => Opcode::OrToModRmFromImmed(mod_rm, immed),
            0b010 => Opcode::AdcToModRmFromImmed(mod_rm, immed),
            0b011 => Opcode::SbbToModRmFromImmed(mod_rm, immed),
            0b100 => Opcode::AndToModRmFromImmed(mod_rm, immed),
            0b101 => Opcode::SubToModRmFromImmed(mod_rm, immed),
            0b110 => Opcode::XorToModRmFromImmed(mod_rm, immed),
            0b111 => Opcode::CmpToModRmFromImmed(mod_rm, immed),
            _ => return Err(DisassemblerError::InvalidOpcodeExtension(mnemonic_encoding)),
        };

        Ok(mnemonic)
    }

    fn parse_short_label(&mut self) -> Result<i16, DisassemblerError> {
        Ok(((self.read_byte()? as i8 as isize) + (self.index as isize)) as i16)
    }

    fn parse_jump_word(&mut self) -> Result<i16, DisassemblerError> {
        Ok(((self.read_word()? as i16 as isize) + (self.index as isize)) as i16)
    }

    fn parse_mod_sr_rm(
        &mut self,
        prefixes: &mut Prefixes,
    ) -> Result<(ModRm<Width16>, SegmentRegister), DisassemblerError> {
        let (mod_rm, gen_reg) = self.parse_mod_reg_rm(prefixes)?;

        Ok((mod_rm, SegmentRegister::from(gen_reg)))
    }

    // TODO: If this knew the D bit, then it could order the operands correctly
    fn parse_mod_reg_rm<W: OpWidth>(
        &mut self,
        prefixes: &mut Prefixes,
    ) -> Result<(ModRm<W>, W::Register), DisassemblerError> {
        let mode = Mode::from_modrm(self.bytes[self.index]);
        let reg = W::Register::from(RegCode::from_modrm_reg(self.bytes[self.index]));
        let rm = RmCode::from_modrm(self.bytes[self.index]);
        self.index += 1;

        let mod_rm = match mode {
            Mode::Register => ModRm::Register(W::Register::from(rm)),
            _ => {
                let (base, index_reg) = match rm {
                    RmCode::Rm000 => (GeneralRegister16::BX, Some(GeneralRegister16::SI)),
                    RmCode::Rm001 => (GeneralRegister16::BX, Some(GeneralRegister16::DI)),
                    RmCode::Rm010 => (GeneralRegister16::BP, Some(GeneralRegister16::SI)),
                    RmCode::Rm011 => (GeneralRegister16::BP, Some(GeneralRegister16::DI)),
                    RmCode::Rm100 => (GeneralRegister16::SI, None),
                    RmCode::Rm101 => (GeneralRegister16::DI, None),
                    RmCode::Rm110 => (GeneralRegister16::BP, None),
                    RmCode::Rm111 => (GeneralRegister16::BX, None),
                };

                let sr = match prefixes.sr_override {
                    Some(reg) => reg,
                    None => match base {
                        GeneralRegister16::BP => SegmentRegister::SS,
                        _ => SegmentRegister::DS,
                    },
                };

                if mode == Mode::Mem8BitDisplacement {
                    ModRm::from_mem(MemoryIndex {
                        displacement: Some(Immediate::Byte(self.read_byte()?)),
                        base: Some(base),
                        index: index_reg,
                        sr,
                    })
                } else if mode == Mode::Mem16BitDisplacement {
                    ModRm::from_mem(MemoryIndex {
                        displacement: Some(Immediate::Word(self.read_word()?)),
                        base: Some(base),
                        index: index_reg,
                        sr,
                    })
                } else if mode == Mode::MemNoDisplacement {
                    if rm == RmCode::Rm110 {
                        let sr = match prefixes.sr_override {
                            Some(reg) => reg,
                            None => SegmentRegister::DS,
                        };

                        ModRm::from_mem(MemoryIndex {
                            displacement: Some(Immediate::Word(self.read_word()?)),
                            base: None,
                            index: None,
                            sr,
                        })
                    } else {
                        ModRm::from_mem(MemoryIndex {
                            displacement: None,
                            base: Some(base),
                            index: index_reg,
                            sr,
                        })
                    }
                } else {
                    panic!("Unexpected memory mode {:?} with rm {:?}", mode, rm);
                }
            }
        };

        Ok((mod_rm, reg))
    }

    // fn read_byte(&mut self) -> u8 {
    //     let byte = self.bytes[self.index];
    //     self.index += 1;

    //     byte
    // }

    // fn read_word(&mut self) -> u16 {
    //     let word = u16::from_le_bytes([self.bytes[self.index], self.bytes[self.index + 1]]);
    //     self.index += 2;

    //     word
    // }
}

impl ByteReader for Disassembler {
    type Error = DisassemblerError;

    fn read_byte(&mut self) -> Result<u8, Self::Error> {
        match self.bytes.get(self.index) {
            None => Err(DisassemblerError::EOF),
            Some(byte) => {
                self.index += 1;

                Ok(*byte)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;
    use std::io;
    use std::io::Write;
    use std::path::Path;

    use std::{fs, io::Read};

    use flate2::bufread::GzDecoder;
    use reqwest::StatusCode;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct TestMetadata {
        opcodes: HashMap<String, OpcodeMetadata>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Normal {
        status: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Status {
        status: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct OpcodeExtension {
        reg: HashMap<String, Status>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "lowercase", untagged)]
    enum OpcodeMetadata {
        Normal(Normal),
        OpcodeExtension(OpcodeExtension),
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct TestSpec {
        name: String,
        bytes: Vec<u8>,
    }

    #[test]
    fn hardware_generated_tests() {
        let base_url = "https://github.com/SingleStepTests/8088/raw/refs/heads/main/v2/";

        let mut response = reqwest::blocking::get(format!("{}{}", base_url, "metadata.json"))
            .unwrap()
            .error_for_status()
            .unwrap();
        let mut response_str = String::new();
        let _ = response.read_to_string(&mut response_str).unwrap();

        let test_metadata: TestMetadata = serde_json::from_str(&response_str).unwrap();

        let curr_file_path = Path::new(file!());
        let tests_folder_path = curr_file_path
            .parent()
            .unwrap()
            .join("hardware_generated_tests");
        fs::create_dir_all(&tests_folder_path).unwrap();

        for (opcode, metadata) in test_metadata.opcodes.iter() {
            // TODO: 0xF6 (similarly for 0xF7) has [46, 243, 246, 248] `idiv al`. I believe this is illegal according to the
            // manual but the CPU still does something because the hardware didn't yet handle illegal
            // opcodes
            if opcode == "F6" || opcode == "F7" {
                continue;
            }

            let filenames = match metadata {
                OpcodeMetadata::Normal(Normal { status }) => {
                    if status == "prefix" {
                        vec![]
                    } else {
                        vec![opcode.clone()]
                    }
                }
                OpcodeMetadata::OpcodeExtension(ext) => ext
                    .reg
                    .keys()
                    .map(|ext_component| format!("{}.{}", opcode, ext_component))
                    .collect(),
            };

            for name in filenames {
                let filename = tests_folder_path.as_path().join(&name);
                let bytes = match fs::read(&filename) {
                    Ok(bytes) => bytes,
                    Err(err) => match err.kind() {
                        io::ErrorKind::NotFound => {
                            let response = match reqwest::blocking::get(format!(
                                "{}{}.json.gz",
                                base_url, name
                            ))
                            .unwrap()
                            .error_for_status()
                            {
                                Ok(resp) => resp,
                                Err(err) if err.status() == Some(StatusCode::NOT_FOUND) => continue,
                                Err(err) => panic!("Request failed: {}", err),
                            };

                            let bytes = response.bytes().unwrap();

                            let decoder = GzDecoder::new(&bytes[..]);
                            let test_list: Vec<TestSpec> = serde_json::from_reader(decoder)
                                .unwrap_or_else(|_| panic!("Unable to deserialize {:?}", filename));

                            let parent_dir = Path::new(&filename).parent().unwrap();
                            fs::create_dir_all(parent_dir).unwrap();

                            let file = fs::File::create(filename)
                                .expect("Should be able to create the file as it won't exist");
                            let mut buf_writer = io::BufWriter::new(file);

                            let _ = serde_json::to_writer_pretty(&mut buf_writer, &test_list);

                            buf_writer.flush().unwrap();

                            serde_json::to_vec(&test_list).unwrap()
                        }
                        _ => panic!("unexpected error when reading file {:?}: {}", filename, err),
                    },
                };

                let test_list: Vec<TestSpec> = serde_json::from_slice(&bytes).unwrap();

                let mut num_wrong = 0;
                for test_spec in test_list {
                    let mut disassembler = Disassembler::from_bytes(test_spec.bytes.clone());
                    disassembler.disassemble().unwrap();
                    let observed_name = disassembler.dump_operations();

                    if observed_name != test_spec.name {
                        num_wrong += 1;

                        println!(
                            "observed: \"{}\"\nexpected: {:#?}\n",
                            observed_name, test_spec
                        );
                    }
                }

                assert_eq!(num_wrong, 0);
            }
        }
    }

    #[test]
    fn test_wait() {
        let bytes = vec![0x9B];
        let mut disassembler = Disassembler::from_bytes(bytes.clone());
        disassembler.disassemble().unwrap();
        let observed_name = disassembler.dump_operations();

        assert_eq!(observed_name, "wait");
    }
}
