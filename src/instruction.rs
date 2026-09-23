use std::fmt;

use crate::impl_opcodes_display;
use crate::operand::MemoryIndex;
use crate::operand::ModRm;
use crate::register::GeneralRegister16;
use crate::register::SegmentRegister;
use crate::width::OpWidth;
use crate::width::Width8;
use crate::width::Width16;

use paste::paste;

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
    // Segment Register
    SROverride(SegmentRegister),

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
    // PushMem16(MemoryIndex),

    // TODO: The manual says this is MEM16, but there's at least one hardware generated test that has a
    // register as input
    PushModRm16(ModRm<Width16>),

    // // pop
    PopSR(SegmentRegister),
    PopGR16(GeneralRegister16),
    PopModRm(ModRm<Width16>),

    // or
    OrFromReg(ModRm<W>, W::Register),
    OrToReg(W::Register, ModRm<W>),
    OrToALFromImmed8(W::Immediate),
    OrToAXFromImmed16(W::Immediate),
    // TODO: If we associated u<8/16> width OpWidth, we should be able to consolidate these two
    // opcodes
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

    // FIXME: These may not actually exist, temporarily including them to satisfy the
    // `impl_opcodes_display` macro
    TestToReg(W::Register, ModRm<W>),

    TestToALFromImmed8(W::Immediate),
    TestToAXFromImmed16(W::Immediate),
    TestToModRmFromImmed(ModRm<W>, W::Immediate),

    // xchg
    XchgToReg(W::Register, ModRm<W>),

    // FIXME: These may not actually exist, temporarily including them to satisfy the
    // `impl_opcodes_display` macro
    XchgFromReg(ModRm<W>, W::Register),
    XchgToALFromImmed8(W::Immediate),
    XchgToAXFromImmed16(W::Immediate),
    XchgToModRmFromImmed(ModRm<W>, W::Immediate),

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
    MovToALFromImmed8(W::Immediate),
    MovToAXFromImmed16(W::Immediate),

    // FIXME: These may not actually exist, temporarily including them to satisfy the
    // `impl_opcodes_display` macro
    MovToModRmFromImmed(ModRm<W>, W::Immediate),

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

    // // repnz
    // Repnz,

    // // repz
    // Repz,

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

impl_opcodes_display!(
    (Add, "add"),
    (Or, "or"),
    (Adc, "adc"),
    (Sbb, "sbb"),
    (And, "and"),
    (Sub, "sub"),
    (Xor, "xor"),
    (Cmp, "cmp"),
    (Test, "test"),
    (Xchg, "xchg"),
    (Mov, "mov")
);

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
