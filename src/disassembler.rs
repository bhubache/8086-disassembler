use std::fs;

use flate2::write;

use crate::instruction::Instruction;
use crate::instruction::Opcode;
use crate::instruction::RepeatableStringInstruction;
use crate::mode::Mode;
use crate::operand::Immediate;
use crate::operand::MemoryIndex;
use crate::operand::ModRm8;
use crate::operand::ModRm16;
use crate::operand::SizedModRm;
use crate::parse_mod_reg_rm_8_from_reg;
use crate::parse_mod_reg_rm_8_to_reg;
use crate::parse_mod_reg_rm_16_from_reg;
use crate::parse_mod_reg_rm_16_to_reg;
use crate::register::GeneralRegister8;
use crate::register::GeneralRegister16;
use crate::register::SegmentRegister;
use crate::register::SizedRegister;
use crate::rm::RM;

#[derive(Debug)]
pub enum DisassemblerError {
    InvalidOpcode(u8),
    InvalidMode(u8),
    InvalidRM(u8),
    InvalidOpcodeExtension(u8),
    InvalidRepOperand(u8),
}

impl std::fmt::Display for DisassemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidOpcode(opcode) => write!(f, "invalid opcode `{:08b}`", opcode),
            Self::InvalidMode(mode) => write!(f, "invalid mode `{:02b}`", mode),
            Self::InvalidRM(rm) => write!(f, "invalid rm `{:03b}`", rm),
            Self::InvalidOpcodeExtension(value) => {
                write!(f, "invalid opcode extension `{:03b}`", value)
            }
            Self::InvalidRepOperand(value) => write!(f, "invalid rep operand `{:02X}`", value),
        }
    }
}

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

    pub fn dump(&self) -> String {
        self.instructions
            .iter()
            .map(|inst| inst.opcode.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn disassemble(&mut self) -> Result<(), DisassemblerError> {
        // dbg!(self.bytes.iter().map(|byte| format!("{}", byte)).collect::<Vec<_>>().join("\n"));
        let mut sr_override = None;
        while self.index < self.bytes.len() {
            let (opcode_index, opcode) = (self.index, self.bytes[self.index]);
            self.index += 1;

            let mnemonic = match opcode {
                0x00 => parse_mod_reg_rm_8_from_reg!(self, Opcode::AddFromReg8, sr_override),
                0x01 => parse_mod_reg_rm_16_from_reg!(self, Opcode::AddFromReg16, sr_override),
                0x02 => parse_mod_reg_rm_8_to_reg!(self, Opcode::AddToReg8, sr_override),
                0x03 => parse_mod_reg_rm_16_to_reg!(self, Opcode::AddToReg16, sr_override),
                0x04 => Opcode::AddToALFromImmed8(self.parse_byte()),
                0x05 => Opcode::AddToAXFromImmed16(self.parse_word()),
                0x06 => Opcode::PushSR(SegmentRegister::ES),
                0x07 => Opcode::PopSR(SegmentRegister::ES),
                0x08 => parse_mod_reg_rm_8_from_reg!(self, Opcode::OrFromReg8, sr_override),
                0x09 => parse_mod_reg_rm_16_from_reg!(self, Opcode::OrFromReg16, sr_override),
                0x0A => parse_mod_reg_rm_8_to_reg!(self, Opcode::OrToReg8, sr_override),
                0x0B => parse_mod_reg_rm_16_to_reg!(self, Opcode::OrToReg16, sr_override),
                0x0C => Opcode::OrToALFromImmed8(self.parse_byte()),
                0x0D => Opcode::OrToAXFromImmed16(self.parse_word()),
                0x0E => Opcode::PushSR(SegmentRegister::CS),
                0x0F => todo!(),
                0x10 => parse_mod_reg_rm_8_from_reg!(self, Opcode::AdcFromReg8, sr_override),
                0x11 => parse_mod_reg_rm_16_from_reg!(self, Opcode::AdcFromReg16, sr_override),
                0x12 => parse_mod_reg_rm_8_to_reg!(self, Opcode::AdcToReg8, sr_override),
                0x13 => parse_mod_reg_rm_16_to_reg!(self, Opcode::AdcToReg16, sr_override),
                0x14 => Opcode::AdcToALFromImmed8(self.parse_byte()),
                0x15 => Opcode::AdcToAXFromImmed16(self.parse_word()),
                0x16 => Opcode::PushSR(SegmentRegister::SS),
                0x17 => Opcode::PopSR(SegmentRegister::SS),
                0x18 => parse_mod_reg_rm_8_from_reg!(self, Opcode::SbbFromReg8, sr_override),
                0x19 => parse_mod_reg_rm_16_from_reg!(self, Opcode::SbbFromReg16, sr_override),
                0x1A => parse_mod_reg_rm_8_to_reg!(self, Opcode::SbbToReg8, sr_override),
                0x1B => parse_mod_reg_rm_16_to_reg!(self, Opcode::SbbToReg16, sr_override),
                0x1C => Opcode::SbbToALFromImmed8(self.parse_byte()),
                0x1D => Opcode::SbbToAXFromImmed16(self.parse_word()),
                0x1E => Opcode::PushSR(SegmentRegister::DS),
                0x1F => Opcode::PopSR(SegmentRegister::DS),
                0x20 => parse_mod_reg_rm_8_from_reg!(self, Opcode::AndFromReg8, sr_override),
                0x21 => parse_mod_reg_rm_16_from_reg!(self, Opcode::AndFromReg16, sr_override),
                0x22 => parse_mod_reg_rm_8_to_reg!(self, Opcode::AndToReg8, sr_override),
                0x23 => parse_mod_reg_rm_16_to_reg!(self, Opcode::AndToReg16, sr_override),
                0x24 => Opcode::AndToALFromImmed8(self.parse_byte()),
                0x25 => Opcode::AndToAXFromImmed16(self.parse_word()),
                0x26 => Opcode::SROverride(SegmentRegister::ES),
                0x27 => Opcode::Daa,
                0x28 => parse_mod_reg_rm_8_from_reg!(self, Opcode::SubFromReg8, sr_override),
                0x29 => parse_mod_reg_rm_16_from_reg!(self, Opcode::SubFromReg16, sr_override),
                0x2A => parse_mod_reg_rm_8_to_reg!(self, Opcode::SubToReg8, sr_override),
                0x2B => parse_mod_reg_rm_16_to_reg!(self, Opcode::SubToReg16, sr_override),
                0x2C => Opcode::SubToALFromImmed8(self.parse_byte()),
                0x2D => Opcode::SubToAXFromImmed16(self.parse_word()),
                0x2E => Opcode::SROverride(SegmentRegister::CS),
                0x2F => Opcode::Das,
                0x30 => parse_mod_reg_rm_8_from_reg!(self, Opcode::XorFromReg8, sr_override),
                0x31 => parse_mod_reg_rm_16_from_reg!(self, Opcode::XorFromReg16, sr_override),
                0x32 => parse_mod_reg_rm_8_to_reg!(self, Opcode::XorToReg8, sr_override),
                0x33 => parse_mod_reg_rm_16_to_reg!(self, Opcode::XorToReg16, sr_override),
                0x34 => Opcode::XorToALFromImmed8(self.parse_byte()),
                0x35 => Opcode::XorToAXFromImmed16(self.parse_word()),
                0x36 => Opcode::SROverride(SegmentRegister::SS),
                0x37 => Opcode::Aaa,
                0x38 => parse_mod_reg_rm_8_from_reg!(self, Opcode::CmpFromReg8, sr_override),
                0x39 => parse_mod_reg_rm_16_from_reg!(self, Opcode::CmpFromReg16, sr_override),
                0x3A => parse_mod_reg_rm_8_to_reg!(self, Opcode::CmpToReg8, sr_override),
                0x3B => parse_mod_reg_rm_16_to_reg!(self, Opcode::CmpToReg16, sr_override),
                0x3C => Opcode::CmpToALFromImmed8(self.parse_byte()),
                0x3D => Opcode::CmpToAXFromImmed16(self.parse_word()),
                0x3E => Opcode::SROverride(SegmentRegister::DS),
                0x3F => Opcode::Aas,
                0x40 => Opcode::IncGR16(GeneralRegister16::AX),
                0x41 => Opcode::IncGR16(GeneralRegister16::CX),
                0x42 => Opcode::IncGR16(GeneralRegister16::DX),
                0x43 => Opcode::IncGR16(GeneralRegister16::BX),
                0x44 => Opcode::IncGR16(GeneralRegister16::SP),
                0x45 => Opcode::IncGR16(GeneralRegister16::BP),
                0x46 => Opcode::IncGR16(GeneralRegister16::SI),
                0x47 => Opcode::IncGR16(GeneralRegister16::DI),
                0x48 => Opcode::DecGR16(GeneralRegister16::AX),
                0x49 => Opcode::DecGR16(GeneralRegister16::CX),
                0x4A => Opcode::DecGR16(GeneralRegister16::DX),
                0x4B => Opcode::DecGR16(GeneralRegister16::BX),
                0x4C => Opcode::DecGR16(GeneralRegister16::SP),
                0x4D => Opcode::DecGR16(GeneralRegister16::BP),
                0x4E => Opcode::DecGR16(GeneralRegister16::SI),
                0x4F => Opcode::DecGR16(GeneralRegister16::DI),
                0x50 => Opcode::PushGR16(GeneralRegister16::AX),
                0x51 => Opcode::PushGR16(GeneralRegister16::CX),
                0x52 => Opcode::PushGR16(GeneralRegister16::DX),
                0x53 => Opcode::PushGR16(GeneralRegister16::BX),
                0x54 => Opcode::PushGR16(GeneralRegister16::SP),
                0x55 => Opcode::PushGR16(GeneralRegister16::BP),
                0x56 => Opcode::PushGR16(GeneralRegister16::SI),
                0x57 => Opcode::PushGR16(GeneralRegister16::DI),
                0x58 => Opcode::PopGR16(GeneralRegister16::AX),
                0x59 => Opcode::PopGR16(GeneralRegister16::CX),
                0x5A => Opcode::PopGR16(GeneralRegister16::DX),
                0x5B => Opcode::PopGR16(GeneralRegister16::BX),
                0x5C => Opcode::PopGR16(GeneralRegister16::SP),
                0x5D => Opcode::PopGR16(GeneralRegister16::BP),
                0x5E => Opcode::PopGR16(GeneralRegister16::SI),
                0x5F => Opcode::PopGR16(GeneralRegister16::DI),
                0x60 => todo!(),
                0x61 => todo!(),
                0x62 => todo!(),
                0x63 => todo!(),
                0x64 => todo!(),
                0x65 => todo!(),
                0x66 => todo!(),
                0x67 => todo!(),
                0x68 => todo!(),
                0x69 => todo!(),
                0x6A => todo!(),
                0x6B => todo!(),
                0x6C => todo!(),
                0x6D => todo!(),
                0x6E => todo!(),
                0x6F => todo!(),
                0x70 => Opcode::Jo(self.parse_short_label()),
                0x71 => Opcode::Jno(self.parse_short_label()),
                0x72 => Opcode::Jb(self.parse_short_label()),
                0x73 => Opcode::Jnb(self.parse_short_label()),
                0x74 => Opcode::Jz(self.parse_short_label()),
                0x75 => Opcode::Jnz(self.parse_short_label()),
                0x76 => Opcode::Jbe(self.parse_short_label()),
                0x77 => Opcode::Jnbe(self.parse_short_label()),
                0x78 => Opcode::Js(self.parse_short_label()),
                0x79 => Opcode::Jns(self.parse_short_label()),
                0x7A => Opcode::Jp(self.parse_short_label()),
                0x7B => Opcode::Jnp(self.parse_short_label()),
                0x7C => Opcode::Jl(self.parse_short_label()),
                0x7D => Opcode::Jnl(self.parse_short_label()),
                0x7E => Opcode::Jle(self.parse_short_label()),
                0x7F => Opcode::Jnle(self.parse_short_label()),
                0x80 | 0x82 => {
                    let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                    let (mod_rm, _) = self.parse_mod_reg_rm(sr_override)?;
                    let immed = self.parse_byte();

                    Self::create_modrm_with_reg_mnemonic_encoding_8(
                        mnemonic_encoding,
                        mod_rm,
                        immed,
                    )?
                }
                0x81 => {
                    let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                    let (mod_rm, _) = self.parse_mod_reg_rm(sr_override)?;
                    let immed = self.parse_word();

                    Self::create_modrm_with_reg_mnemonic_encoding_16(
                        mnemonic_encoding,
                        mod_rm,
                        immed,
                    )?
                }
                0x83 => {
                    let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                    let (mod_rm, _) = self.parse_mod_reg_rm(sr_override)?;
                    let immed = self.parse_byte();

                    Self::create_modrm_with_reg_mnemonic_encoding_8_sx(
                        mnemonic_encoding,
                        mod_rm,
                        immed,
                    )?
                }
                0x84 => parse_mod_reg_rm_8_from_reg!(self, Opcode::TestFromReg8, sr_override),
                0x85 => parse_mod_reg_rm_16_from_reg!(self, Opcode::TestFromReg16, sr_override),
                0x86 => parse_mod_reg_rm_8_to_reg!(self, Opcode::XchgToReg8, sr_override),
                0x87 => parse_mod_reg_rm_16_to_reg!(self, Opcode::XchgToReg16, sr_override),
                0x88 => parse_mod_reg_rm_8_from_reg!(self, Opcode::MovFromReg8, sr_override),
                0x89 => parse_mod_reg_rm_16_from_reg!(self, Opcode::MovFromReg16, sr_override),
                0x8A => parse_mod_reg_rm_8_to_reg!(self, Opcode::MovToReg8, sr_override),
                0x8B => parse_mod_reg_rm_16_to_reg!(self, Opcode::MovToReg16, sr_override),
                0x8C => {
                    let (mod_rm, sr) = self.parse_mod_sr_rm(sr_override)?;
                    Opcode::MovFromSR(mod_rm, sr)
                }
                0x8D => {
                    let (fst, snd) = self.parse_mod_rm_as_mem_index(sr_override)?;
                    Opcode::LeaToGR16(fst, snd)
                }
                0x8E => {
                    let (mod_rm, sr) = self.parse_mod_sr_rm(sr_override)?;
                    Opcode::MovToSR(sr, mod_rm)
                }
                0x8F => {
                    let (mod_rm, _) = self.parse_mod_reg_rm(sr_override)?;

                    Opcode::PopModRm(mod_rm)
                }
                0x90 => Opcode::Nop,
                0x91 => Opcode::XchgToAXFromCX,
                0x92 => Opcode::XchgToAXFromDX,
                0x93 => Opcode::XchgToAXFromBX,
                0x94 => Opcode::XchgToAXFromSP,
                0x95 => Opcode::XchgToAXFromBP,
                0x96 => Opcode::XchgToAXFromSI,
                0x97 => Opcode::XchgToAXFromDI,
                0x98 => Opcode::Cbw,
                0x99 => Opcode::Cwd,
                0x9A => {
                    let displacement = self.parse_word();
                    let segment = self.parse_word();
                    Opcode::CallFarProc(format!("{:04X}h:{:04X}h", segment, displacement))
                }
                0x9B => Opcode::Wait,
                0x9C => Opcode::PushF,
                0x9D => Opcode::PopF,
                0x9E => Opcode::SahF,
                0x9F => Opcode::LahF,
                0xA0 => Opcode::MovToALFromMem8(MemoryIndex::with_immediate(
                    Immediate::Word(self.parse_word()),
                    sr_override,
                )),
                0xA1 => Opcode::MovToAXFromMem16(MemoryIndex::with_immediate(
                    Immediate::Word(self.parse_word()),
                    sr_override,
                )),
                0xA2 => Opcode::MovToMem8FromAL(MemoryIndex::with_immediate(
                    Immediate::Word(self.parse_word()),
                    sr_override,
                )),
                0xA3 => Opcode::MovToMem16FromAL(MemoryIndex::with_immediate(
                    Immediate::Word(self.parse_word()),
                    sr_override,
                )),
                0xA4 => Opcode::MovS8(sr_override),
                0xA5 => Opcode::MovS16(sr_override),
                0xA6 => Opcode::CmpS8(sr_override),
                0xA7 => Opcode::CmpS16(sr_override),
                0xA8 => Opcode::TestToALFromImmed8(self.parse_byte()),
                0xA9 => Opcode::TestToAXFromImmed16(self.parse_word()),
                0xAA => todo!(),
                0xAB => todo!(),
                0xAC => todo!(),
                0xAD => todo!(),
                0xAE => todo!(),
                0xAF => todo!(),
                0xB0 => Opcode::MovToGR8FromImmed8(GeneralRegister8::AL, self.parse_byte()),
                0xB1 => Opcode::MovToGR8FromImmed8(GeneralRegister8::CL, self.parse_byte()),
                0xB2 => Opcode::MovToGR8FromImmed8(GeneralRegister8::DL, self.parse_byte()),
                0xB3 => Opcode::MovToGR8FromImmed8(GeneralRegister8::BL, self.parse_byte()),
                0xB4 => Opcode::MovToGR8FromImmed8(GeneralRegister8::AH, self.parse_byte()),
                0xB5 => Opcode::MovToGR8FromImmed8(GeneralRegister8::CH, self.parse_byte()),
                0xB6 => Opcode::MovToGR8FromImmed8(GeneralRegister8::DH, self.parse_byte()),
                0xB7 => Opcode::MovToGR8FromImmed8(GeneralRegister8::BH, self.parse_byte()),
                0xB8 => Opcode::MovToGR16FromImmed16(GeneralRegister16::AX, self.parse_word()),
                0xB9 => Opcode::MovToGR16FromImmed16(GeneralRegister16::CX, self.parse_word()),
                0xBA => Opcode::MovToGR16FromImmed16(GeneralRegister16::DX, self.parse_word()),
                0xBB => Opcode::MovToGR16FromImmed16(GeneralRegister16::BX, self.parse_word()),
                0xBC => Opcode::MovToGR16FromImmed16(GeneralRegister16::SP, self.parse_word()),
                0xBD => Opcode::MovToGR16FromImmed16(GeneralRegister16::BP, self.parse_word()),
                0xBE => Opcode::MovToGR16FromImmed16(GeneralRegister16::SI, self.parse_word()),
                0xBF => Opcode::MovToGR16FromImmed16(GeneralRegister16::DI, self.parse_word()),
                0xC0 => todo!(),
                0xC1 => todo!(),
                0xC2 => Opcode::RetIntraSegImmed16(self.parse_word()),
                0xC3 => Opcode::RetIntraSeg,
                0xC4 => {
                    let (fst, snd) = self.parse_mod_rm_as_mem_index(sr_override)?;
                    Opcode::LesToReg(fst, snd)
                }
                0xC5 => {
                    let (fst, snd) = self.parse_mod_rm_as_mem_index(sr_override)?;
                    Opcode::LdsToReg(fst, snd)
                }
                0xC6 => {
                    let (mod_rm, _) = self.parse_mod_reg_rm(sr_override)?;
                    Opcode::MovToMem8FromImmed8(mod_rm, self.parse_byte())
                }
                0xC7 => {
                    let (mod_rm, _) = self.parse_mod_reg_rm(sr_override)?;
                    Opcode::MovToMem16FromImmed16(mod_rm, self.parse_word())
                }
                0xC8 => todo!(),
                0xC9 => todo!(),
                0xCA => Opcode::RetInterSegImmed16(self.parse_word()),
                0xCB => Opcode::RetInterSeg,
                0xCC => Opcode::Int3,
                0xCD => Opcode::IntFromImmed8(self.parse_byte()),
                0xCE => Opcode::Into,
                0xCF => Opcode::Iret,
                0xD0 => {
                    let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                    let (mod_rm, _) = self.parse_mod_reg_rm(sr_override)?;
                    // let immed = self.parse_byte();

                    Self::create_modrm_with_reg_mnemonic_encoding_8_2(mnemonic_encoding, mod_rm)?
                }
                0xD1 => {
                    let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                    let (mod_rm, _) = self.parse_mod_reg_rm(sr_override)?;
                    // let immed = self.parse_byte();

                    Self::create_modrm_with_reg_mnemonic_encoding_16_2(mnemonic_encoding, mod_rm)?
                }
                0xD2 => {
                    let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                    let (mod_rm, _) = self.parse_mod_reg_rm(sr_override)?;
                    // let immed = self.parse_byte();

                    Self::create_modrm_with_reg_mnemonic_encoding_8_2_cl(mnemonic_encoding, mod_rm)?
                }
                0xD3 => {
                    let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                    let (mod_rm, _) = self.parse_mod_reg_rm(sr_override)?;
                    // let immed = self.parse_byte();

                    Self::create_modrm_with_reg_mnemonic_encoding_16_2_cl(
                        mnemonic_encoding,
                        mod_rm,
                    )?
                }
                0xD4 => Opcode::Aam(self.parse_byte()),
                0xD5 => Opcode::Aad(self.parse_byte()),
                0xD6 => todo!(),
                0xD7 => Opcode::Xlat,
                0xD8 | 0xD9 | 0xDA | 0xDB | 0xDC | 0xDD | 0xDE | 0xDF => {
                    let (mod_rm, _) = self.parse_mod_reg_rm(sr_override)?;
                    Opcode::Esc(mod_rm)
                }
                0xE0 => Opcode::Loopne(self.parse_short_label()),
                0xE1 => Opcode::Loope(self.parse_short_label()),
                0xE2 => Opcode::Loop(self.parse_short_label()),
                0xE3 => Opcode::Jcxz(self.parse_short_label()),
                0xE4 => Opcode::InToALFromImmed8(self.parse_byte()),
                0xE5 => Opcode::InToAXFromImmed8(self.parse_byte()),
                0xE6 => Opcode::OutToALFromImmed8(self.parse_byte()),
                0xE7 => Opcode::OutToAXFromImmed8(self.parse_byte()),
                0xE8 => Opcode::CallNearProc(format!("{:04X}h", self.parse_jump_word())),
                0xE9 => Opcode::JmpNearLabel(format!("{:04X}h", self.parse_jump_word())),
                0xEA => {
                    let displacement = self.parse_word();
                    let segment = self.parse_word();
                    Opcode::JmpFarLabel(format!("{:04X}h:{:04X}h", segment, displacement))
                }
                0xEB => Opcode::JmpShortLabel(self.parse_short_label()),
                0xEC => Opcode::InToALFromDX,
                0xED => Opcode::InToAXFromDX,
                0xEE => Opcode::OutToDXFromAL,
                0xEF => Opcode::OutToDXFromAX,
                0xF0 => Opcode::Lock,
                0xF1 => todo!(),
                0xF2 => Opcode::Repne(sr_override, self.parse_rep_op()?),
                0xF3 => Opcode::Rep(sr_override, self.parse_rep_op()?),
                0xF4 => Opcode::Hlt,
                0xF5 => Opcode::Cmc,
                0xF6 => {
                    let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                    let (mod_rm, _) = self.parse_mod_reg_rm(sr_override)?;

                    self.create_modrm_with_reg_mnemonic_encoding_8_3(mnemonic_encoding, mod_rm)?
                }
                0xF7 => {
                    let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                    let (mod_rm, _) = self.parse_mod_reg_rm(sr_override)?;

                    self.create_modrm_with_reg_mnemonic_encoding_16_3(mnemonic_encoding, mod_rm)?
                }
                0xF8 => Opcode::Clc,
                0xF9 => Opcode::Stc,
                0xFA => Opcode::Cli,
                0xFB => Opcode::Sti,
                0xFC => Opcode::Cld,
                0xFD => Opcode::Std,
                0xFE => {
                    let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                    let (mod_rm, _) = self.parse_mod_reg_rm(sr_override)?;

                    self.create_modrm_with_reg_mnemonic_encoding_8_4(mnemonic_encoding, mod_rm)?
                }
                0xFF => {
                    let mnemonic_encoding: u8 = (self.bytes[self.index] & 0b00111000) >> 3;
                    let (mod_rm, _) = self.parse_mod_reg_rm(sr_override)?;

                    self.create_modrm_with_reg_mnemonic_encoding_16_4(mnemonic_encoding, mod_rm)?
                }
                _ => return Err(DisassemblerError::InvalidOpcode(opcode)),
            };

            match mnemonic {
                Opcode::SROverride(sr) => {
                    sr_override = Some(sr);
                }
                _ => {
                    sr_override = None;
                    self.instructions
                        .push(Instruction::new(opcode_index, mnemonic));
                }
            }
        }

        Ok(())
    }

    fn parse_mod_rm_as_mem_index(
        &mut self,
        sr_override: Option<SegmentRegister>,
    ) -> Result<(GeneralRegister16, MemoryIndex), DisassemblerError> {
        let (mod_rm, dst_reg) = self.parse_mod_reg_rm(sr_override)?;
        // FIXME: Is it appropriate to panic here?
        match mod_rm {
            ModRm16::Register(reg) => panic!(
                "unexpectedly parsed MOD/RM as register `{}` rather than a memory index",
                reg
            ),
            ModRm16::EffectiveAddr(mem_index) => Ok((dst_reg, mem_index)),
        }
    }

    fn parse_rep_op(&mut self) -> Result<RepeatableStringInstruction, DisassemblerError> {
        let rsi = match self.parse_byte() {
            0xA4 => RepeatableStringInstruction::Movsb,
            0xA5 => RepeatableStringInstruction::Movsw,
            0xA6 => RepeatableStringInstruction::Cmpsb,
            0xA7 => RepeatableStringInstruction::Cmpsw,
            op => return Err(DisassemblerError::InvalidRepOperand(op)),
        };

        Ok(rsi)
    }

    fn create_modrm_with_reg_mnemonic_encoding_8_4(
        &mut self,
        mnemonic_encoding: u8,
        mod_rm: ModRm8,
    ) -> Result<Opcode, DisassemblerError> {
        let mnemonic = match mnemonic_encoding {
            0b000 => Opcode::IncModRm8(mod_rm),
            0b001 => Opcode::DecModRm8(mod_rm),
            0b010 => todo!(),
            0b011 => todo!(),
            0b100 => todo!(),
            0b101 => todo!(),
            0b110 => todo!(),
            0b111 => todo!(),
            _ => return Err(DisassemblerError::InvalidOpcodeExtension(mnemonic_encoding)),
        };

        Ok(mnemonic)
    }

    fn create_modrm_with_reg_mnemonic_encoding_16_4(
        &mut self,
        mnemonic_encoding: u8,
        mod_rm: ModRm16,
    ) -> Result<Opcode, DisassemblerError> {
        let mnemonic = match mnemonic_encoding {
            0b000 => Opcode::IncModRm16(mod_rm),
            0b001 => Opcode::DecModRm16(mod_rm),
            0b010 => Opcode::CallModRm16(mod_rm),
            0b011 => match mod_rm {
                ModRm16::Register(_) => panic!("Uhoh!!"),
                ModRm16::EffectiveAddr(mem_index) => Opcode::CallMem16(mem_index),
            },
            0b100 => Opcode::JmpModRm16(mod_rm),
            0b101 => match mod_rm {
                ModRm16::Register(_) => panic!("Uhoh!!"),
                ModRm16::EffectiveAddr(mem_index) => Opcode::JmpMem16(mem_index),
            },
            0b110 => {
                Opcode::PushModRm16(mod_rm)
                // match mod_rm {
                //     ModRm16::Register(_) => panic!("Uhoh!!"),
                //     ModRm16::EffectiveAddr(mem_index) => Opcode::PushMem16(mem_index),
                // }
            }
            0b111 => Opcode::PushModRm16(mod_rm),
            _ => return Err(DisassemblerError::InvalidOpcodeExtension(mnemonic_encoding)),
        };

        Ok(mnemonic)
    }

    fn create_modrm_with_reg_mnemonic_encoding_8_3(
        &mut self,
        mnemonic_encoding: u8,
        mod_rm: ModRm8,
    ) -> Result<Opcode, DisassemblerError> {
        let mnemonic = match mnemonic_encoding {
            0b000 => Opcode::TestToModRmFromImmed8(mod_rm, self.parse_byte()),
            0b001 => Opcode::TestToModRmFromImmed8(mod_rm, self.parse_byte()),
            0b010 => Opcode::NotToModRm8(mod_rm),
            0b011 => Opcode::NegToModRm8(mod_rm),
            0b100 => Opcode::MulToModRm8(mod_rm),
            0b101 => Opcode::ImulToModRm8(mod_rm),
            0b110 => Opcode::DivToModRm8(mod_rm),
            0b111 => Opcode::IdivToModRm8(mod_rm),
            _ => return Err(DisassemblerError::InvalidOpcodeExtension(mnemonic_encoding)),
        };

        Ok(mnemonic)
    }

    fn create_modrm_with_reg_mnemonic_encoding_16_3(
        &mut self,
        mnemonic_encoding: u8,
        mod_rm: ModRm16,
    ) -> Result<Opcode, DisassemblerError> {
        let mnemonic = match mnemonic_encoding {
            0b000 => Opcode::TestToModRmFromImmed16(mod_rm, self.parse_word()),
            0b001 => Opcode::TestToModRmFromImmed16(mod_rm, self.parse_word()),
            0b010 => Opcode::NotToModRm16(mod_rm),
            0b011 => Opcode::NegToModRm16(mod_rm),
            0b100 => Opcode::MulToModRm16(mod_rm),
            0b101 => Opcode::ImulToModRm16(mod_rm),
            0b110 => Opcode::DivToModRm16(mod_rm),
            0b111 => Opcode::IdivToModRm16(mod_rm),
            _ => return Err(DisassemblerError::InvalidOpcodeExtension(mnemonic_encoding)),
        };

        Ok(mnemonic)
    }

    // TODO: Differentiate between the opcode extension variant in the error type
    fn create_modrm_with_reg_mnemonic_encoding_8_2(
        mnemonic_encoding: u8,
        mod_rm: ModRm8,
    ) -> Result<Opcode, DisassemblerError> {
        let mnemonic = match mnemonic_encoding {
            0b000 => Opcode::RolToModRm8(mod_rm),
            0b001 => Opcode::RorToModRm8(mod_rm),
            0b010 => Opcode::RclToModRm8(mod_rm),
            0b011 => Opcode::RcrToModRm8(mod_rm),
            0b100 => Opcode::ShlToModRm8(mod_rm),
            0b101 => Opcode::ShrToModRm8(mod_rm),

            // TODO: The manual says 0b110 is unused but there exist tests for it
            0b110 => Opcode::SetmoToModRm8(mod_rm),

            0b111 => Opcode::SarToModRm8(mod_rm),
            _ => return Err(DisassemblerError::InvalidOpcodeExtension(mnemonic_encoding)),
        };

        Ok(mnemonic)
    }

    fn create_modrm_with_reg_mnemonic_encoding_16_2(
        mnemonic_encoding: u8,
        mod_rm: ModRm16,
    ) -> Result<Opcode, DisassemblerError> {
        let mnemonic = match mnemonic_encoding {
            0b000 => Opcode::RolToModRm16(mod_rm),
            0b001 => Opcode::RorToModRm16(mod_rm),
            0b010 => Opcode::RclToModRm16(mod_rm),
            0b011 => Opcode::RcrToModRm16(mod_rm),
            0b100 => Opcode::ShlToModRm16(mod_rm),
            0b101 => Opcode::ShrToModRm16(mod_rm),

            // TODO: The manual says 0b110 is unused but there exist tests for it
            0b110 => Opcode::SetmoToModRm16(mod_rm),

            0b111 => Opcode::SarToModRm16(mod_rm),
            _ => return Err(DisassemblerError::InvalidOpcodeExtension(mnemonic_encoding)),
        };

        Ok(mnemonic)
    }

    fn create_modrm_with_reg_mnemonic_encoding_8_2_cl(
        mnemonic_encoding: u8,
        mod_rm: ModRm8,
    ) -> Result<Opcode, DisassemblerError> {
        let mnemonic = match mnemonic_encoding {
            0b000 => Opcode::RolToModRm8CL(mod_rm),
            0b001 => Opcode::RorToModRm8CL(mod_rm),
            0b010 => Opcode::RclToModRm8CL(mod_rm),
            0b011 => Opcode::RcrToModRm8CL(mod_rm),
            0b100 => Opcode::ShlToModRm8CL(mod_rm),
            0b101 => Opcode::ShrToModRm8CL(mod_rm),

            // TODO: The manual says 0b110 is unused but there exist tests for it
            0b110 => Opcode::SetmoToModRm8CL(mod_rm),

            0b111 => Opcode::SarToModRm8CL(mod_rm),
            _ => return Err(DisassemblerError::InvalidOpcodeExtension(mnemonic_encoding)),
        };

        Ok(mnemonic)
    }

    fn create_modrm_with_reg_mnemonic_encoding_16_2_cl(
        mnemonic_encoding: u8,
        mod_rm: ModRm16,
    ) -> Result<Opcode, DisassemblerError> {
        let mnemonic = match mnemonic_encoding {
            0b000 => Opcode::RolToModRm16CL(mod_rm),
            0b001 => Opcode::RorToModRm16CL(mod_rm),
            0b010 => Opcode::RclToModRm16CL(mod_rm),
            0b011 => Opcode::RcrToModRm16CL(mod_rm),
            0b100 => Opcode::ShlToModRm16CL(mod_rm),
            0b101 => Opcode::ShrToModRm16CL(mod_rm),

            // TODO: The manual says 0b110 is unused but there exist tests for it
            0b110 => Opcode::SetmoToModRm16CL(mod_rm),

            0b111 => Opcode::SarToModRm16CL(mod_rm),
            _ => return Err(DisassemblerError::InvalidOpcodeExtension(mnemonic_encoding)),
        };

        Ok(mnemonic)
    }

    fn create_modrm_with_reg_mnemonic_encoding_8(
        mnemonic_encoding: u8,
        mod_rm: ModRm8,
        immed: u8,
    ) -> Result<Opcode, DisassemblerError> {
        let mnemonic = match mnemonic_encoding {
            0b000 => Opcode::AddToModRmFromImmed8(mod_rm, immed),
            0b001 => Opcode::OrToModRmFromImmed8(mod_rm, immed),
            0b010 => Opcode::AdcToModRmFromImmed8(mod_rm, immed),
            0b011 => Opcode::SbbToModRmFromImmed8(mod_rm, immed),
            0b100 => Opcode::AndToModRmFromImmed8(mod_rm, immed),
            0b101 => Opcode::SubToModRmFromImmed8(mod_rm, immed),
            0b110 => Opcode::XorToModRmFromImmed8(mod_rm, immed),
            0b111 => Opcode::CmpToModRmFromImmed8(mod_rm, immed),
            _ => return Err(DisassemblerError::InvalidOpcodeExtension(mnemonic_encoding)),
        };

        Ok(mnemonic)
    }

    fn create_modrm_with_reg_mnemonic_encoding_16(
        mnemonic_encoding: u8,
        mod_rm: ModRm16,
        immed: u16,
    ) -> Result<Opcode, DisassemblerError> {
        let mnemonic = match mnemonic_encoding {
            0b000 => Opcode::AddToModRmFromImmed16(mod_rm, immed),
            0b001 => Opcode::OrToModRmFromImmed16(mod_rm, immed),
            0b010 => Opcode::AdcToModRmFromImmed16(mod_rm, immed),
            0b011 => Opcode::SbbToModRmFromImmed16(mod_rm, immed),
            0b100 => Opcode::AndToModRmFromImmed16(mod_rm, immed),
            0b101 => Opcode::SubToModRmFromImmed16(mod_rm, immed),
            0b110 => Opcode::XorToModRmFromImmed16(mod_rm, immed),
            0b111 => Opcode::CmpToModRmFromImmed16(mod_rm, immed),
            _ => return Err(DisassemblerError::InvalidOpcodeExtension(mnemonic_encoding)),
        };

        Ok(mnemonic)
    }

    fn create_modrm_with_reg_mnemonic_encoding_8_sx(
        mnemonic_encoding: u8,
        mod_rm: ModRm16,
        immed: u8,
    ) -> Result<Opcode, DisassemblerError> {
        let immed_xs = match ((immed & 0b10000000) >> 7) == 1 {
            true => (0b11111111 << 8) | immed as u16,
            false => immed as u16,
        };

        let mnemonic = match mnemonic_encoding {
            0b000 => Opcode::AddToModRmFromImmed16(mod_rm, immed_xs),
            0b001 => Opcode::OrToModRmFromImmed16(mod_rm, immed_xs),
            0b010 => Opcode::AdcToModRmFromImmed16(mod_rm, immed_xs),
            0b011 => Opcode::SbbToModRmFromImmed16(mod_rm, immed_xs),
            0b100 => Opcode::AndToModRmFromImmed16(mod_rm, immed_xs),
            0b101 => Opcode::SubToModRmFromImmed16(mod_rm, immed_xs),
            0b110 => Opcode::XorToModRmFromImmed16(mod_rm, immed_xs),
            0b111 => Opcode::CmpToModRmFromImmed16(mod_rm, immed_xs),
            _ => return Err(DisassemblerError::InvalidOpcodeExtension(mnemonic_encoding)),
        };

        Ok(mnemonic)
    }

    fn parse_short_label(&mut self) -> i16 {
        ((self.parse_byte() as i8 as isize) + (self.index as isize)) as i16
    }

    fn parse_jump_word(&mut self) -> i16 {
        ((self.parse_word() as i16 as isize) + (self.index as isize)) as i16
    }

    fn parse_mod_sr_rm(
        &mut self,
        sr_override: Option<SegmentRegister>,
    ) -> Result<(ModRm16, SegmentRegister), DisassemblerError> {
        let (mod_rm, gen_reg) = self.parse_mod_reg_rm(sr_override)?;

        Ok((mod_rm, gen_reg.to_sr()))
    }

    // TODO: If this knew the D bit, then it could order the operands correctly
    fn parse_mod_reg_rm<M: SizedModRm>(
        &mut self,
        sr_override: Option<SegmentRegister>,
    ) -> Result<(M, M::Reg), DisassemblerError> {
        let mode = Mode::from_byte(self.bytes[self.index])?;
        let reg = M::Reg::from_byte(self.bytes[self.index]);
        let rm = RM::from_byte(self.bytes[self.index])?;
        self.index += 1;

        let mod_rm = match mode {
            Mode::Register => M::from_register(M::Reg::from_reg_encoding(rm.to_encoding())),
            _ => {
                let (base, index_reg) = match rm {
                    RM::Rm000 => (GeneralRegister16::BX, Some(GeneralRegister16::SI)),
                    RM::Rm001 => (GeneralRegister16::BX, Some(GeneralRegister16::DI)),
                    RM::Rm010 => (GeneralRegister16::BP, Some(GeneralRegister16::SI)),
                    RM::Rm011 => (GeneralRegister16::BP, Some(GeneralRegister16::DI)),
                    RM::Rm100 => (GeneralRegister16::SI, None),
                    RM::Rm101 => (GeneralRegister16::DI, None),
                    RM::Rm110 => (GeneralRegister16::BP, None),
                    RM::Rm111 => (GeneralRegister16::BX, None),
                };

                let sr = match sr_override {
                    Some(reg) => reg,
                    None => match base {
                        GeneralRegister16::BP => SegmentRegister::SS,
                        _ => SegmentRegister::DS,
                    },
                };

                if mode == Mode::Mem8BitDisplacement {
                    M::from_mem(MemoryIndex {
                        displacement: Some(Immediate::Byte(self.parse_byte())),
                        base: Some(base),
                        index: index_reg,
                        sr,
                    })
                } else if mode == Mode::Mem16BitDisplacement {
                    M::from_mem(MemoryIndex {
                        displacement: Some(Immediate::Word(self.parse_word())),
                        base: Some(base),
                        index: index_reg,
                        sr,
                    })
                } else if mode == Mode::MemNoDisplacement {
                    if rm == RM::Rm110 {
                        let sr = match sr_override {
                            Some(reg) => reg,
                            None => SegmentRegister::DS,
                        };

                        M::from_mem(MemoryIndex {
                            displacement: Some(Immediate::Word(self.parse_word())),
                            base: None,
                            index: None,
                            sr,
                        })
                    } else {
                        M::from_mem(MemoryIndex {
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

    fn parse_byte(&mut self) -> u8 {
        let byte = self.bytes[self.index];
        self.index += 1;

        byte
    }

    fn parse_word(&mut self) -> u16 {
        let word = u16::from_le_bytes([self.bytes[self.index], self.bytes[self.index + 1]]);
        self.index += 2;

        word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::io;
    use std::io::Write;

    use flate2::bufread::GzDecoder;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct TestSpec {
        name: String,
        bytes: Vec<u8>,
    }

    fn run_tests_in_file(opcode: &str) {
        dbg!(opcode);
        let filename = format!("/tmp/test_files/{}", opcode);
        let bytes = match fs::read(&filename) {
            Ok(bytes) => bytes,
            Err(err) => {
                match err.kind() {
                    io::ErrorKind::NotFound => {
                        let response = reqwest::blocking::get(format!("https://github.com/SingleStepTests/8088/raw/refs/heads/main/v2/{}.json.gz", opcode)).unwrap();
                        let bytes = response.bytes().unwrap();

                        let decoder = GzDecoder::new(&bytes[..]);
                        let test_list: Vec<TestSpec> = serde_json::from_reader(decoder).unwrap();

                        let parent_dir = std::path::Path::new(&filename).parent().unwrap();
                        fs::create_dir_all(parent_dir).unwrap();

                        let file = fs::File::create(filename)
                            .expect("Should be able to create the file as it won't exist");
                        let mut buf_writer = io::BufWriter::new(file);

                        let _ = serde_json::to_writer_pretty(&mut buf_writer, &test_list);

                        buf_writer.flush().unwrap();

                        serde_json::to_vec(&test_list).unwrap()
                    }
                    _ => panic!("unexpected error when reading file {}: {}", filename, err),
                }
            }
        };

        let test_list: Vec<TestSpec> = serde_json::from_slice(&bytes).unwrap();

        let mut num_wrong = 0;
        for test_spec in test_list {
            let mut disassembler = Disassembler::from_bytes(test_spec.bytes.clone());
            println!("{:#?}", test_spec);
            disassembler.disassemble().unwrap();
            let observed_name = disassembler.dump();

            if observed_name != test_spec.name {
                num_wrong += 1;

                println!(
                    "observed: \"{}\"\nexpected: {:#?}\n",
                    observed_name, test_spec
                );
            }
        }

        dbg!(opcode);
        assert_eq!(num_wrong, 0);
    }

    // #[test]
    fn test_specific() {
        let bytes = vec![46, 243, 246, 248];
        let mut disassembler = Disassembler::from_bytes(bytes.clone());
        disassembler.disassemble().unwrap();
        let observed_name = disassembler.dump();

        // 0b11001111 0b10101100

        assert_eq!(observed_name, "idiv al");
    }

    #[test]
    fn test_a_lot() {
        // NOTE: Tests are not available for [0xF0, 0xF4]
        // TODO: Look into why tests for 0x9B are missing
        // TODO: Complete tests 0xA4-0xA7 and 0xAA-0xAF and 0xC8-0xC9
        // TODO: Figure out what to do with opcodes that the manual says are unused but tests
        // TODO: 0xE6 and 0xE7 in the tests appear to have the operands flipped?
        // produce instructions for
        // TODO: 0xF6 (similarly for 0xF7) has [46, 243, 246, 248] `idiv al`. I believe this is illegal according to the
        // manual but the CPU still does something because the hardware didn't yet handle illegal
        // opcodes
        let unused_opcodes = vec![
            0x0F, 0x26, 0x2E, 0x36, 0x3E, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
            0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x9B, 0xA4, 0xA5, 0xA6, 0xA7, 0xAA, 0xAB,
            0xAC, 0xAD, 0xAE, 0xAF, 0xC0, 0xC1, 0xC8, 0xC9, 0xD6, 0xE6, 0xE7, 0xF0, 0xF1, 0xF2,
            0xF3, 0xF4, 0xF6, 0xF7,
        ];
        for opcode in 0x00..0x100 {
            if unused_opcodes.contains(&opcode) {
                continue;
            }

            if opcode == 0x80
                || opcode == 0x81
                || opcode == 0x82
                || opcode == 0x83
                || opcode == 0xD0
                || opcode == 0xD1
                || opcode == 0xD2
                || opcode == 0xD3
                || opcode == 0xF6
                || opcode == 0xF7
                || opcode == 0xFE
                || opcode == 0xFF
            {
                for variant in [0, 1, 2, 3, 4, 5, 6, 7] {
                    if opcode == 0xFE && variant > 1 {
                        continue;
                    }
                    run_tests_in_file(&format!("{:02X}.{}", opcode, variant));
                }
            } else {
                run_tests_in_file(&format!("{:02X}", opcode));
            }
        }
    }
}
