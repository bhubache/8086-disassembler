use std::fmt;

use crate::operand::MemoryIndex;
use crate::operand::ModRm;
use crate::register::GeneralRegister16;
use crate::register::SegmentRegister;
use crate::width::OpWidth;
use crate::width::Width8;
use crate::width::Width16;

pub struct Instruction {
    pub address: usize,
    pub operation: Operation,
}

impl Instruction {
    pub fn new(address: usize, operation: Operation) -> Self {
        Instruction { address, operation }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04X}    {}", self.address, self.operation)
    }
}

#[derive(Debug)]
pub enum Operation {
    Width8(Opcode<Width8>),
    Width16(Opcode<Width16>),
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Width8(opcode) => write!(f, "{}", opcode),
            Self::Width16(opcode) => write!(f, "{}", opcode),
        }
    }
}

#[derive(Debug)]
pub enum Opcode<W: OpWidth> {
    // rep
    Rep(Option<SegmentRegister>, RepeatableStringInstruction),
    Repne(Option<SegmentRegister>, RepeatableStringInstruction),

    // add
    AddFromReg(ModRm<W>, W::Register),
    AddToReg(W::Register, ModRm<W>),
    AddToALFromImmed8(W::Immediate),
    AddToAXFromImmed16(W::Immediate),
    AddToModRmFromImmed(ModRm<W>, W::ImmedGroupImmediate),

    // push
    PushSR(SegmentRegister),
    PushGR16(GeneralRegister16),

    // TODO: The manual says this is MEM16, but there's at least one hardware generated test that has a
    // register as input
    PushModRm16(ModRm<Width16>),

    // pop
    PopSR(SegmentRegister),
    PopGR16(GeneralRegister16),
    PopModRm(ModRm<Width16>),

    // or
    OrFromReg(ModRm<W>, W::Register),
    OrToReg(W::Register, ModRm<W>),
    OrToALFromImmed8(W::Immediate),
    OrToAXFromImmed16(W::Immediate),
    OrToModRmFromImmed(ModRm<W>, W::ImmedGroupImmediate),

    // adc
    AdcFromReg(ModRm<W>, W::Register),
    AdcToReg(W::Register, ModRm<W>),
    AdcToALFromImmed8(W::Immediate),
    AdcToAXFromImmed16(W::Immediate),
    AdcToModRmFromImmed(ModRm<W>, W::ImmedGroupImmediate),

    // sbb
    SbbFromReg(ModRm<W>, W::Register),
    SbbToReg(W::Register, ModRm<W>),
    SbbToALFromImmed8(W::Immediate),
    SbbToAXFromImmed16(W::Immediate),
    SbbToModRmFromImmed(ModRm<W>, W::ImmedGroupImmediate),

    // and
    AndFromReg(ModRm<W>, W::Register),
    AndToReg(W::Register, ModRm<W>),
    AndToALFromImmed8(W::Immediate),
    AndToAXFromImmed16(W::Immediate),
    AndToModRmFromImmed(ModRm<W>, W::ImmedGroupImmediate),

    // daa
    Daa,

    // sub
    SubFromReg(ModRm<W>, W::Register),
    SubToReg(W::Register, ModRm<W>),
    SubToALFromImmed8(W::Immediate),
    SubToAXFromImmed16(W::Immediate),
    SubToModRmFromImmed(ModRm<W>, W::ImmedGroupImmediate),

    // das
    Das,

    // xor
    XorFromReg(ModRm<W>, W::Register),
    XorToReg(W::Register, ModRm<W>),
    XorToALFromImmed8(W::Immediate),
    XorToAXFromImmed16(W::Immediate),
    XorToModRmFromImmed(ModRm<W>, W::ImmedGroupImmediate),

    // aaa
    Aaa,

    // aas
    Aas,

    // cmp
    CmpFromReg(ModRm<W>, W::Register),
    CmpToReg(W::Register, ModRm<W>),
    CmpToALFromImmed8(W::Immediate),
    CmpToAXFromImmed16(W::Immediate),
    CmpToModRmFromImmed(ModRm<W>, W::ImmedGroupImmediate),

    // inc
    IncGR16(GeneralRegister16),
    IncModRm(ModRm<W>),

    // dec
    DecGR16(GeneralRegister16),
    DecModRm(ModRm<W>),

    // Is i16 correct?
    Jb(i16),
    Jbe(i16),
    Jo(i16),
    Jno(i16),
    Jnb(i16),
    Jz(i16),
    Jnz(i16),
    Jnbe(i16),
    Js(i16),
    Jns(i16),
    Jp(i16),
    Jnp(i16),
    Jl(i16),
    Jnl(i16),
    Jle(i16),
    Jnle(i16),
    Jcxz(i16),

    // test
    TestFromReg(ModRm<W>, W::Register),
    TestToALFromImmed8(W::Immediate),
    TestToAXFromImmed16(W::Immediate),
    TestToModRmFromImmed(ModRm<W>, W::Immediate),

    // xchg
    XchgToReg(W::Register, ModRm<W>),
    XchgToAXFromCX,
    XchgToAXFromDX,
    XchgToAXFromBX,
    XchgToAXFromSP,
    XchgToAXFromBP,
    XchgToAXFromSI,
    XchgToAXFromDI,

    // mov
    MovFromReg(ModRm<W>, W::Register),
    MovToReg(W::Register, ModRm<W>),
    MovFromSR(ModRm<Width16>, SegmentRegister),
    MovToSR(SegmentRegister, ModRm<Width16>),
    MovToALFromMem8(MemoryIndex),
    MovToAXFromMem16(MemoryIndex),
    MovToMem8FromAL(MemoryIndex),
    MovToMem16FromAL(MemoryIndex),
    MovToGRFromImmed(W::Register, W::Immediate),
    MovToMemFromImmed(ModRm<W>, W::Immediate),

    // lea
    // TODO: I believe ModRm16 should actually be something like MemoryIndex16
    LeaToGR16(GeneralRegister16, MemoryIndex),

    // nop
    Nop,

    // cbw
    Cbw,

    // cwd
    Cwd,

    // call
    CallFarProc(String),
    CallNearProc(String),
    CallModRm16(ModRm<Width16>),
    CallMem16(MemoryIndex),
    Wait,
    PushF,
    PopF,
    SahF,
    LahF,

    // TODO: Opcode should actually be constrainted to something like RepeatableOperation
    // movs
    MovS8(Option<SegmentRegister>),
    MovS16(Option<SegmentRegister>),

    // cmps
    CmpS8(Option<SegmentRegister>),
    CmpS16(Option<SegmentRegister>),

    // stos
    StoS8(Option<SegmentRegister>),
    StoS16(Option<SegmentRegister>),

    // lods
    LodS8(Option<SegmentRegister>),
    LodS16(Option<SegmentRegister>),

    // scas
    ScaS8(Option<SegmentRegister>),
    ScaS16(Option<SegmentRegister>),

    // ret
    // TODO: Should these immediates explicitly be 16 bits because there's no opcode that accepts 8
    // bits?
    RetIntraSegImmed16(W::Immediate),
    RetIntraSeg,
    RetInterSegImmed16(W::Immediate),
    RetInterSeg,

    // les
    LesToReg(GeneralRegister16, MemoryIndex),

    // lds
    LdsToReg(GeneralRegister16, MemoryIndex),

    // int
    Int3,
    IntFromImmed8(W::Immediate),

    // into
    Into,

    // iret
    Iret,

    // rol
    RolToModRm(ModRm<W>),
    RolToModRmCL(ModRm<W>),

    // ror
    RorToModRm(ModRm<W>),
    RorToModRmCL(ModRm<W>),

    // rcl
    RclToModRm(ModRm<W>),
    RclToModRmCL(ModRm<W>),

    // rcr
    RcrToModRm(ModRm<W>),
    RcrToModRmCL(ModRm<W>),

    // shl
    ShlToModRm(ModRm<W>),
    ShlToModRmCL(ModRm<W>),

    // shr
    ShrToModRm(ModRm<W>),
    ShrToModRmCL(ModRm<W>),

    // sar
    SetmoToModRm(ModRm<W>),
    SetmoToModRmCL(ModRm<W>),
    SarToModRm(ModRm<W>),
    SarToModRmCL(ModRm<W>),

    // aam
    Aam(W::Immediate),

    // aad
    Aad(W::Immediate),

    // xlat
    Xlat,

    // esc
    Esc(ModRm<Width16>),

    // loopnz/loopne
    Loopne(i16),

    // loopz/loope
    Loope(i16),

    // loop
    Loop(i16),

    // in
    InToALFromImmed8(W::Immediate),
    InToAXFromImmed8(W::Immediate),
    InToALFromDX,
    InToAXFromDX,

    // out
    OutToPort8FromAL(W::Immediate),
    OutToPort8FromAX(W::Immediate),
    OutToDXFromAL,
    OutToDXFromAX,

    // lock
    Lock,

    // hlt
    Hlt,

    // cmc
    Cmc,

    // not
    NotToModRm(ModRm<W>),

    // neg
    NegToModRm(ModRm<W>),

    // mul
    MulToModRm(ModRm<W>),

    // imul
    ImulToModRm(ModRm<W>),

    // div
    DivToModRm(ModRm<W>),

    // idiv
    IdivToModRm(ModRm<W>),

    // clc
    Clc,

    // stc
    Stc,

    // cli
    Cli,

    // Sti
    Sti,

    // cld
    Cld,

    // std
    Std,

    // jmp
    JmpNearLabel(String),
    JmpFarLabel(String),
    JmpShortLabel(i16),
    JmpModRm16(ModRm<Width16>),
    JmpMem16(MemoryIndex),

    // salc - set AL to Carry
    Salc,
}

// TODO: Impl Into to convert to Opcode?
#[derive(Debug)]
pub enum RepeatableStringInstruction {
    Movsb,
    Movsw,
    Cmpsb,
    Cmpsw,
    Stosb,
    Stosw,
    Lodsb,
    Lodsw,
    Scasb,
    Scasw,
}

impl fmt::Display for RepeatableStringInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Movsb => write!(f, "movsb"),
            Self::Movsw => write!(f, "movsw"),
            Self::Cmpsb => write!(f, "cmpsb"),
            Self::Cmpsw => write!(f, "cmpsw"),
            Self::Stosb => write!(f, "stosb"),
            Self::Stosw => write!(f, "stosw"),
            Self::Lodsb => write!(f, "lodsb"),
            Self::Lodsw => write!(f, "lodsw"),
            Self::Scasb => write!(f, "scasb"),
            Self::Scasw => write!(f, "scasw"),
        }
    }
}

impl<W: OpWidth> fmt::Display for Opcode<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AddFromReg(mod_rm, reg) => write!(f, "add {}, {}", mod_rm, reg),
            Self::AddToReg(reg, mod_rm) => write!(f, "add {}, {}", reg, mod_rm),
            Self::AddToALFromImmed8(byte) => write!(f, "add al, {:X}", byte),
            Self::AddToAXFromImmed16(word) => write!(f, "add ax, {:X}", word),
            Self::AddToModRmFromImmed(mod_rm, immed) => write!(f, "add {}, {:X}", mod_rm, immed),

            Self::OrFromReg(mod_rm, reg) => write!(f, "or {}, {}", mod_rm, reg),
            Self::OrToReg(reg, mod_rm) => write!(f, "or {}, {}", reg, mod_rm),
            Self::OrToALFromImmed8(byte) => write!(f, "or al, {:X}", byte),
            Self::OrToAXFromImmed16(word) => write!(f, "or ax, {:X}", word),
            Self::OrToModRmFromImmed(mod_rm, immed) => write!(f, "or {}, {:X}", mod_rm, immed),

            Self::AdcFromReg(mod_rm, reg) => write!(f, "adc {}, {}", mod_rm, reg),
            Self::AdcToReg(reg, mod_rm) => write!(f, "adc {}, {}", reg, mod_rm),
            Self::AdcToALFromImmed8(byte) => write!(f, "adc al, {:X}", byte),
            Self::AdcToAXFromImmed16(word) => write!(f, "adc ax, {:X}", word),
            Self::AdcToModRmFromImmed(mod_rm, immed) => write!(f, "adc {}, {:X}", mod_rm, immed),

            Self::SbbFromReg(mod_rm, reg) => write!(f, "sbb {}, {}", mod_rm, reg),
            Self::SbbToReg(reg, mod_rm) => write!(f, "sbb {}, {}", reg, mod_rm),
            Self::SbbToALFromImmed8(byte) => write!(f, "sbb al, {:X}", byte),
            Self::SbbToAXFromImmed16(word) => write!(f, "sbb ax, {:X}", word),
            Self::SbbToModRmFromImmed(mod_rm, immed) => write!(f, "sbb {}, {:X}", mod_rm, immed),

            Self::AndFromReg(mod_rm, reg) => write!(f, "and {}, {}", mod_rm, reg),
            Self::AndToReg(reg, mod_rm) => write!(f, "and {}, {}", reg, mod_rm),
            Self::AndToALFromImmed8(byte) => write!(f, "and al, {:X}", byte),
            Self::AndToAXFromImmed16(word) => write!(f, "and ax, {:X}", word),
            Self::AndToModRmFromImmed(mod_rm, immed) => write!(f, "and {}, {:X}", mod_rm, immed),

            Self::SubFromReg(mod_rm, reg) => write!(f, "sub {}, {}", mod_rm, reg),
            Self::SubToReg(reg, mod_rm) => write!(f, "sub {}, {}", reg, mod_rm),
            Self::SubToALFromImmed8(byte) => write!(f, "sub al, {:X}", byte),
            Self::SubToAXFromImmed16(word) => write!(f, "sub ax, {:X}", word),
            Self::SubToModRmFromImmed(mod_rm, immed) => write!(f, "sub {}, {:X}", mod_rm, immed),

            Self::XorFromReg(mod_rm, reg) => write!(f, "xor {}, {}", mod_rm, reg),
            Self::XorToReg(reg, mod_rm) => write!(f, "xor {}, {}", reg, mod_rm),
            Self::XorToALFromImmed8(byte) => write!(f, "xor al, {:X}", byte),
            Self::XorToAXFromImmed16(word) => write!(f, "xor ax, {:X}", word),
            Self::XorToModRmFromImmed(mod_rm, immed) => write!(f, "xor {}, {:X}", mod_rm, immed),

            Self::CmpFromReg(mod_rm, reg) => write!(f, "cmp {}, {}", mod_rm, reg),
            Self::CmpToReg(reg, mod_rm) => write!(f, "cmp {}, {}", reg, mod_rm),
            Self::CmpToALFromImmed8(byte) => write!(f, "cmp al, {:X}", byte),
            Self::CmpToAXFromImmed16(word) => write!(f, "cmp ax, {:X}", word),
            Self::CmpToModRmFromImmed(mod_rm, immed) => write!(f, "cmp {}, {:X}", mod_rm, immed),

            Self::TestFromReg(mod_rm, reg) => write!(f, "test {}, {}", mod_rm, reg),
            Self::TestToALFromImmed8(byte) => write!(f, "test al, {:X}", byte),
            Self::TestToAXFromImmed16(word) => write!(f, "test ax, {:X}", word),
            Self::TestToModRmFromImmed(mod_rm, immed) => write!(f, "test {}, {:X}", mod_rm, immed),

            Self::Rep(sr, rep_str_inst) => {
                let sr_str = match sr {
                    Some(sr) => format!("{} ", sr),
                    None => String::new(),
                };

                let rep_str = match rep_str_inst {
                    RepeatableStringInstruction::Scasb => "repe",
                    RepeatableStringInstruction::Scasw => "repe",
                    RepeatableStringInstruction::Cmpsb => "repe",
                    RepeatableStringInstruction::Cmpsw => "repe",
                    _ => "rep",
                };

                write!(f, "{}{} {}", sr_str, rep_str, rep_str_inst)
            }
            Self::Repne(sr, rep_str_inst) => {
                let sr_str = match sr {
                    Some(sr) => format!("{} ", sr),
                    None => String::new(),
                };

                let rep_str = match rep_str_inst {
                    RepeatableStringInstruction::Scasb => "repne",
                    RepeatableStringInstruction::Scasw => "repne",
                    RepeatableStringInstruction::Cmpsb => "repne",
                    RepeatableStringInstruction::Cmpsw => "repne",
                    _ => "rep",
                };

                write!(f, "{}{} {}", sr_str, rep_str, rep_str_inst)
            }

            Self::PushSR(sr) => write!(f, "push {}", sr),
            Self::PushGR16(reg) => write!(f, "push {}", reg),
            Self::PushModRm16(mod_rm) => write!(f, "push {}", mod_rm),

            Self::PopSR(sr) => write!(f, "pop {}", sr),
            Self::PopGR16(reg) => write!(f, "pop {}", reg),
            Self::PopModRm(mod_rm) => write!(f, "pop {}", mod_rm),

            Self::Daa => write!(f, "daa"),
            Self::Das => write!(f, "das"),
            Self::Aaa => write!(f, "aaa"),
            Self::Aas => write!(f, "aas"),

            Self::IncGR16(reg) => write!(f, "inc {}", reg),
            Self::IncModRm(mod_rm) => write!(f, "inc {}", mod_rm),
            Self::DecGR16(reg) => write!(f, "dec {}", reg),
            Self::DecModRm(mod_rm) => write!(f, "dec {}", mod_rm),

            Self::Jb(short_label) => write!(f, "jb {:04X}h", short_label),
            Self::Jbe(short_label) => write!(f, "jbe {:04X}h", short_label),
            Self::Jo(short_label) => write!(f, "jo {:04X}h", short_label),
            Self::Jno(short_label) => write!(f, "jno {:04X}h", short_label),
            Self::Jnb(short_label) => write!(f, "jnb {:04X}h", short_label),
            Self::Jz(short_label) => write!(f, "jz {:04X}h", short_label),
            Self::Jnz(short_label) => write!(f, "jnz {:04X}h", short_label),
            Self::Jnbe(short_label) => write!(f, "jnbe {:04X}h", short_label),
            Self::Js(short_label) => write!(f, "js {:04X}h", short_label),
            Self::Jns(short_label) => write!(f, "jns {:04X}h", short_label),
            Self::Jp(short_label) => write!(f, "jp {:04X}h", short_label),
            Self::Jnp(short_label) => write!(f, "jnp {:04X}h", short_label),
            Self::Jl(short_label) => write!(f, "jl {:04X}h", short_label),
            Self::Jnl(short_label) => write!(f, "jnl {:04X}h", short_label),
            Self::Jle(short_label) => write!(f, "jle {:04X}h", short_label),
            Self::Jnle(short_label) => write!(f, "jnle {:04X}h", short_label),
            Self::Jcxz(short_label) => write!(f, "jcxz {:04X}h", short_label),

            Self::MovFromReg(mod_rm, reg) => write!(f, "mov {}, {}", mod_rm, reg),
            Self::MovToReg(reg, mod_rm) => write!(f, "mov {}, {}", reg, mod_rm),
            Self::MovFromSR(mod_rm, reg) => write!(f, "mov {}, {}", mod_rm, reg),
            Self::MovToSR(reg, mod_rm) => write!(f, "mov {}, {}", reg, mod_rm),
            Self::MovToALFromMem8(mem_index) => write!(f, "mov al, byte {}", mem_index),
            Self::MovToAXFromMem16(mem_index) => write!(f, "mov ax, word {}", mem_index),
            Self::MovToMem8FromAL(mem_index) => write!(f, "mov byte {}, al", mem_index),
            Self::MovToMem16FromAL(mem_index) => write!(f, "mov word {}, ax", mem_index),
            Self::MovToGRFromImmed(reg, immed) => write!(f, "mov {}, {:X}", reg, immed),
            Self::MovToMemFromImmed(mod_rm, immed) => write!(f, "mov {}, {:X}", mod_rm, immed),

            Self::LeaToGR16(reg, mem_index) => write!(f, "lea {}, {}", reg, mem_index),

            Self::Nop => write!(f, "nop"),

            Self::XchgToReg(reg, mod_rm) => write!(f, "xchg {}, {}", reg, mod_rm),
            Self::XchgToAXFromCX => write!(f, "xchg cx, ax"),
            Self::XchgToAXFromDX => write!(f, "xchg dx, ax"),
            Self::XchgToAXFromBX => write!(f, "xchg bx, ax"),
            Self::XchgToAXFromSP => write!(f, "xchg sp, ax"),
            Self::XchgToAXFromBP => write!(f, "xchg bp, ax"),
            Self::XchgToAXFromSI => write!(f, "xchg si, ax"),
            Self::XchgToAXFromDI => write!(f, "xchg di, ax"),

            Self::Cbw => write!(f, "cbw"),
            Self::Cwd => write!(f, "cwd"),

            Self::CallFarProc(far_proc) => write!(f, "callf {}", far_proc),
            Self::CallNearProc(near_proc) => write!(f, "call {}", near_proc),
            Self::CallModRm16(mod_rm) => write!(f, "call {}", mod_rm),
            Self::CallMem16(mem_index) => write!(f, "callf word {}", mem_index),
            Self::Wait => write!(f, "wait"),
            Self::PushF => write!(f, "pushf"),
            Self::PopF => write!(f, "popf"),
            Self::SahF => write!(f, "sahf"),
            Self::LahF => write!(f, "lahf"),

            Self::MovS8(sr) => match sr {
                Some(sr) => write!(f, "{} movsb", sr),
                None => write!(f, "movsb"),
            },
            Self::MovS16(sr) => match sr {
                Some(sr) => write!(f, "{} movsw", sr),
                None => write!(f, "movsw"),
            },
            Self::CmpS8(sr) => match sr {
                Some(sr) => write!(f, "{} cmpsb", sr),
                None => write!(f, "cmpsb"),
            },
            Self::CmpS16(sr) => match sr {
                Some(sr) => write!(f, "{} cmpsw", sr),
                None => write!(f, "cmpsw"),
            },
            Self::StoS8(sr) => match sr {
                Some(sr) => write!(f, "{} stosb", sr),
                None => write!(f, "stosb"),
            },
            Self::StoS16(sr) => match sr {
                Some(sr) => write!(f, "{} stosw", sr),
                None => write!(f, "stosw"),
            },
            Self::LodS8(sr) => match sr {
                Some(sr) => write!(f, "{} lodsb", sr),
                None => write!(f, "lodsb"),
            },
            Self::LodS16(sr) => match sr {
                Some(sr) => write!(f, "{} lodsw", sr),
                None => write!(f, "lodsw"),
            },
            Self::ScaS8(sr) => match sr {
                Some(sr) => write!(f, "{} scasb", sr),
                None => write!(f, "scasb"),
            },
            Self::ScaS16(sr) => match sr {
                Some(sr) => write!(f, "{} scasw", sr),
                None => write!(f, "scasw"),
            },

            Self::RetIntraSegImmed16(immed) => write!(f, "retn {:X}", immed),
            Self::RetIntraSeg => write!(f, "retn"),
            Self::RetInterSeg => write!(f, "retf"),
            Self::RetInterSegImmed16(immed) => write!(f, "retf {:X}", immed),

            Self::LesToReg(reg, mem_index) => write!(f, "les {}, dword {}", reg, mem_index),
            Self::LdsToReg(reg, mem_index) => write!(f, "lds {}, dword {}", reg, mem_index),

            Self::Int3 => write!(f, "int3"),
            Self::IntFromImmed8(immed) => write!(f, "int {:X}", immed),

            Self::Into => write!(f, "into"),

            Self::Iret => write!(f, "iret"),

            Self::RolToModRm(mod_rm) => write!(f, "rol {}", mod_rm),
            Self::RorToModRm(mod_rm) => write!(f, "ror {}", mod_rm),
            Self::RclToModRm(mod_rm) => write!(f, "rcl {}", mod_rm),
            Self::RcrToModRm(mod_rm) => write!(f, "rcr {}", mod_rm),
            Self::ShlToModRm(mod_rm) => write!(f, "shl {}", mod_rm),
            Self::ShrToModRm(mod_rm) => write!(f, "shr {}", mod_rm),
            Self::SetmoToModRm(mod_rm) => write!(f, "setmo {}", mod_rm),
            Self::SarToModRm(mod_rm) => write!(f, "sar {}", mod_rm),

            Self::RolToModRmCL(mod_rm) => write!(f, "rol {}, cl", mod_rm),
            Self::RorToModRmCL(mod_rm) => write!(f, "ror {}, cl", mod_rm),
            Self::RclToModRmCL(mod_rm) => write!(f, "rcl {}, cl", mod_rm),
            Self::RcrToModRmCL(mod_rm) => write!(f, "rcr {}, cl", mod_rm),
            Self::ShlToModRmCL(mod_rm) => write!(f, "shl {}, cl", mod_rm),
            Self::ShrToModRmCL(mod_rm) => write!(f, "shr {}, cl", mod_rm),
            Self::SetmoToModRmCL(mod_rm) => write!(f, "setmoc {}, cl", mod_rm),
            Self::SarToModRmCL(mod_rm) => write!(f, "sar {}, cl", mod_rm),

            Self::Aam(immed) => write!(f, "aam {:X}", immed),
            Self::Aad(immed) => write!(f, "aad {:X}", immed),
            Self::Xlat => write!(f, "xlat"),

            Self::Esc(mod_rm) => write!(f, "esc {}", mod_rm),

            Self::Loopne(short_label) => write!(f, "loopne {:04X}h", short_label),
            Self::Loope(short_label) => write!(f, "loope {:04X}h", short_label),
            Self::Loop(short_label) => write!(f, "loop {:04X}h", short_label),

            Self::InToALFromImmed8(immed) => write!(f, "in al, {:X}", immed),
            Self::InToAXFromImmed8(immed) => write!(f, "in ax, {:X}", immed),
            Self::InToALFromDX => write!(f, "in al, dx"),
            Self::InToAXFromDX => write!(f, "in ax, dx"),

            Self::OutToPort8FromAL(immed) => write!(f, "out {:X}, al", immed),
            Self::OutToPort8FromAX(immed) => write!(f, "out {:X}, ax", immed),
            Self::OutToDXFromAL => write!(f, "out dx, al"),
            Self::OutToDXFromAX => write!(f, "out dx, ax"),

            Self::JmpNearLabel(near_label) => write!(f, "jmp {}", near_label),
            Self::JmpFarLabel(far_label) => write!(f, "jmpf {}", far_label),
            Self::JmpShortLabel(short_label) => write!(f, "jmp {:04X}h", short_label),
            Self::JmpModRm16(mod_rm) => write!(f, "jmp {}", mod_rm),
            Self::JmpMem16(mem_index) => write!(f, "jmpf word {}", mem_index),

            Self::Lock => write!(f, "lock"),

            Self::Hlt => write!(f, "hlt"),
            Self::Cmc => write!(f, "cmc"),

            Self::NotToModRm(mod_rm) => write!(f, "not {}", mod_rm),
            Self::NegToModRm(mod_rm) => write!(f, "neg {}", mod_rm),
            Self::MulToModRm(mod_rm) => write!(f, "mul {}", mod_rm),
            Self::ImulToModRm(mod_rm) => write!(f, "imul {}", mod_rm),
            Self::DivToModRm(mod_rm) => write!(f, "div {}", mod_rm),
            Self::IdivToModRm(mod_rm) => write!(f, "idiv {}", mod_rm),

            Self::Clc => write!(f, "clc"),
            Self::Stc => write!(f, "stc"),
            Self::Cli => write!(f, "cli"),
            Self::Sti => write!(f, "sti"),
            Self::Cld => write!(f, "cld"),
            Self::Std => write!(f, "std"),
            Self::Salc => write!(f, "salc"),
        }
    }
}
