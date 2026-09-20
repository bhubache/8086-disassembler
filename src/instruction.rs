use std::fmt;

use crate::impl_opcodes_display;
use crate::operand::MemoryIndex;
use crate::operand::ModRm8;
use crate::operand::ModRm16;
use crate::register::GeneralRegister8;
use crate::register::GeneralRegister16;
use crate::register::SegmentRegister;

use paste::paste;

pub struct Instruction {
    pub address: usize,
    pub opcode: Opcode,
}

impl Instruction {
    pub fn new(address: usize, opcode: Opcode) -> Self {
        Instruction { address, opcode }
    }
}

// TODO: Use more appropriate name like `Opcode`
#[derive(Debug)]
pub enum Opcode {
    // Segment Register
    SROverride(SegmentRegister),

    // rep
    Rep(Option<SegmentRegister>, RepeatableStringInstruction),
    Repne(Option<SegmentRegister>, RepeatableStringInstruction),

    // Include <Opcode>ToModRmFromImmed8 and <Opcode>ToModRmFromImmed16 in macro as well?
    // add
    AddFromReg8(ModRm8, GeneralRegister8),
    AddFromReg16(ModRm16, GeneralRegister16),
    AddToReg8(GeneralRegister8, ModRm8),
    AddToReg16(GeneralRegister16, ModRm16),
    AddToALFromImmed8(u8),
    AddToAXFromImmed16(u16),
    AddToModRmFromImmed8(ModRm8, u8),
    AddToModRmFromImmed16(ModRm16, u16),

    // push
    PushSR(SegmentRegister),
    PushGR16(GeneralRegister16),
    // PushMem16(MemoryIndex),

    // TODO: The manual says this is MEM16, but there's at least one hardware generated test that has a
    // register as input
    PushModRm16(ModRm16),

    // // pop
    PopSR(SegmentRegister),
    PopGR16(GeneralRegister16),
    PopModRm(ModRm16),

    // or
    OrFromReg8(ModRm8, GeneralRegister8),
    OrFromReg16(ModRm16, GeneralRegister16),
    OrToReg8(GeneralRegister8, ModRm8),
    OrToReg16(GeneralRegister16, ModRm16),
    OrToALFromImmed8(u8),
    OrToAXFromImmed16(u16),
    OrToModRmFromImmed8(ModRm8, u8),
    OrToModRmFromImmed16(ModRm16, u16),

    // adc
    AdcFromReg8(ModRm8, GeneralRegister8),
    AdcFromReg16(ModRm16, GeneralRegister16),
    AdcToReg8(GeneralRegister8, ModRm8),
    AdcToReg16(GeneralRegister16, ModRm16),
    AdcToALFromImmed8(u8),
    AdcToAXFromImmed16(u16),
    AdcToModRmFromImmed8(ModRm8, u8),
    AdcToModRmFromImmed16(ModRm16, u16),

    // sbb
    SbbFromReg8(ModRm8, GeneralRegister8),
    SbbFromReg16(ModRm16, GeneralRegister16),
    SbbToReg8(GeneralRegister8, ModRm8),
    SbbToReg16(GeneralRegister16, ModRm16),
    SbbToALFromImmed8(u8),
    SbbToAXFromImmed16(u16),
    SbbToModRmFromImmed8(ModRm8, u8),
    SbbToModRmFromImmed16(ModRm16, u16),

    // and
    AndFromReg8(ModRm8, GeneralRegister8),
    AndFromReg16(ModRm16, GeneralRegister16),
    AndToReg8(GeneralRegister8, ModRm8),
    AndToReg16(GeneralRegister16, ModRm16),
    AndToALFromImmed8(u8),
    AndToAXFromImmed16(u16),
    AndToModRmFromImmed8(ModRm8, u8),
    AndToModRmFromImmed16(ModRm16, u16),

    // daa
    Daa,

    // sub
    SubFromReg8(ModRm8, GeneralRegister8),
    SubFromReg16(ModRm16, GeneralRegister16),
    SubToReg8(GeneralRegister8, ModRm8),
    SubToReg16(GeneralRegister16, ModRm16),
    SubToALFromImmed8(u8),
    SubToAXFromImmed16(u16),
    SubToModRmFromImmed8(ModRm8, u8),
    SubToModRmFromImmed16(ModRm16, u16),

    // das
    Das,

    // xor
    XorFromReg8(ModRm8, GeneralRegister8),
    XorFromReg16(ModRm16, GeneralRegister16),
    XorToReg8(GeneralRegister8, ModRm8),
    XorToReg16(GeneralRegister16, ModRm16),
    XorToALFromImmed8(u8),
    XorToAXFromImmed16(u16),
    XorToModRmFromImmed8(ModRm8, u8),
    XorToModRmFromImmed16(ModRm16, u16),

    // aaa
    Aaa,

    // aas
    Aas,

    // cmp
    CmpFromReg8(ModRm8, GeneralRegister8),
    CmpFromReg16(ModRm16, GeneralRegister16),
    CmpToReg8(GeneralRegister8, ModRm8),
    CmpToReg16(GeneralRegister16, ModRm16),
    CmpToALFromImmed8(u8),
    CmpToAXFromImmed16(u16),
    CmpToModRmFromImmed8(ModRm8, u8),
    CmpToModRmFromImmed16(ModRm16, u16),

    // inc
    IncGR16(GeneralRegister16),
    IncModRm8(ModRm8),
    IncModRm16(ModRm16),

    // dec
    DecGR16(GeneralRegister16),
    DecModRm8(ModRm8),
    DecModRm16(ModRm16),

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
    TestFromReg8(ModRm8, GeneralRegister8),
    TestFromReg16(ModRm16, GeneralRegister16),

    // FIXME: These may not actually exist, temporarily including them to satisfy the
    // `impl_opcodes_display` macro
    TestToReg8(GeneralRegister8, ModRm8),
    TestToReg16(GeneralRegister16, ModRm16),

    TestToALFromImmed8(u8),
    TestToAXFromImmed16(u16),
    TestToModRmFromImmed8(ModRm8, u8),
    TestToModRmFromImmed16(ModRm16, u16),

    // xchg
    XchgToReg8(GeneralRegister8, ModRm8),
    XchgToReg16(GeneralRegister16, ModRm16),

    // FIXME: These may not actually exist, temporarily including them to satisfy the
    // `impl_opcodes_display` macro
    XchgFromReg8(ModRm8, GeneralRegister8),
    XchgFromReg16(ModRm16, GeneralRegister16),
    XchgToALFromImmed8(u8),
    XchgToAXFromImmed16(u16),
    XchgToModRmFromImmed8(ModRm8, u8),
    XchgToModRmFromImmed16(ModRm16, u16),

    XchgToAXFromCX,
    XchgToAXFromDX,
    XchgToAXFromBX,
    XchgToAXFromSP,
    XchgToAXFromBP,
    XchgToAXFromSI,
    XchgToAXFromDI,

    // mov
    MovFromReg8(ModRm8, GeneralRegister8),
    MovFromReg16(ModRm16, GeneralRegister16),
    MovToReg8(GeneralRegister8, ModRm8),
    MovToReg16(GeneralRegister16, ModRm16),
    MovToALFromImmed8(u8),
    MovToAXFromImmed16(u16),

    // FIXME: These may not actually exist, temporarily including them to satisfy the
    // `impl_opcodes_display` macro
    MovToModRmFromImmed8(ModRm8, u8),
    MovToModRmFromImmed16(ModRm16, u16),

    MovFromSR(ModRm16, SegmentRegister),
    MovToSR(SegmentRegister, ModRm16),
    MovToALFromMem8(MemoryIndex),
    MovToAXFromMem16(MemoryIndex),
    MovToMem8FromAL(MemoryIndex),
    MovToMem16FromAL(MemoryIndex),
    MovToGR8FromImmed8(GeneralRegister8, u8),
    MovToGR16FromImmed16(GeneralRegister16, u16),
    MovToMem8FromImmed8(ModRm8, u8),
    MovToMem16FromImmed16(ModRm16, u16),

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
    CallModRm16(ModRm16),
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
    RetIntraSegImmed16(u16),
    RetIntraSeg,
    RetInterSegImmed16(u16),
    RetInterSeg,

    // les
    LesToReg(GeneralRegister16, MemoryIndex),

    // lds
    LdsToReg(GeneralRegister16, MemoryIndex),

    // int
    Int3,
    IntFromImmed8(u8),

    // into
    Into,

    // iret
    Iret,

    // rol
    RolToModRm8(ModRm8),
    RolToModRm16(ModRm16),
    RolToModRm8CL(ModRm8),
    RolToModRm16CL(ModRm16),

    // ror
    RorToModRm8(ModRm8),
    RorToModRm16(ModRm16),
    RorToModRm8CL(ModRm8),
    RorToModRm16CL(ModRm16),

    // rcl
    RclToModRm8(ModRm8),
    RclToModRm16(ModRm16),
    RclToModRm8CL(ModRm8),
    RclToModRm16CL(ModRm16),

    // rcr
    RcrToModRm8(ModRm8),
    RcrToModRm16(ModRm16),
    RcrToModRm8CL(ModRm8),
    RcrToModRm16CL(ModRm16),

    // shl
    ShlToModRm8(ModRm8),
    ShlToModRm16(ModRm16),
    ShlToModRm8CL(ModRm8),
    ShlToModRm16CL(ModRm16),

    // shr
    ShrToModRm8(ModRm8),
    ShrToModRm16(ModRm16),
    ShrToModRm8CL(ModRm8),
    ShrToModRm16CL(ModRm16),

    // sar
    SetmoToModRm8(ModRm8),
    SarToModRm8(ModRm8),
    SetmoToModRm16(ModRm16),
    SarToModRm16(ModRm16),
    SetmoToModRm8CL(ModRm8),
    SarToModRm8CL(ModRm8),
    SetmoToModRm16CL(ModRm16),
    SarToModRm16CL(ModRm16),

    // aam
    Aam(u8),

    // aad
    Aad(u8),

    // xlat
    Xlat,

    // esc
    Esc(ModRm16),

    // loopnz/loopne
    Loopne(i16),

    // loopz/loope
    Loope(i16),

    // loop
    Loop(i16),

    // in
    InToALFromImmed8(u8),
    InToAXFromImmed8(u8),
    InToALFromDX,
    InToAXFromDX,

    // out
    OutToPort8FromAL(u8),
    OutToPort8FromAX(u8),
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
    NotToModRm8(ModRm8),
    NotToModRm16(ModRm16),

    // neg
    NegToModRm8(ModRm8),
    NegToModRm16(ModRm16),

    // mul
    MulToModRm8(ModRm8),
    MulToModRm16(ModRm16),

    // imul
    ImulToModRm8(ModRm8),
    ImulToModRm16(ModRm16),

    // div
    DivToModRm8(ModRm8),
    DivToModRm16(ModRm16),

    // idiv
    IdivToModRm8(ModRm8),
    IdivToModRm16(ModRm16),

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
    JmpModRm16(ModRm16),
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
