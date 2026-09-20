#[macro_export]
macro_rules! impl_opcodes_display {
    ( $( ($prefix:ident, $name:expr) ),* ) => {
        impl fmt::Display for Opcode {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                paste! {
                    match self {
                        $(
                            Self::[<$prefix FromReg8>](mod_rm, reg) => write!(f, concat!($name, " {}, {}"), mod_rm, reg),
                            Self::[<$prefix FromReg16>](mod_rm, reg) => write!(f, concat!($name, " {}, {}"), mod_rm, reg),
                            Self::[<$prefix ToReg8>](reg, mod_rm) => write!(f, concat!($name, " {}, {}"), reg, mod_rm),
                            Self::[<$prefix ToReg16>](reg, mod_rm) => write!(f, concat!($name, " {}, {}"), reg, mod_rm),
                            Self::[<$prefix ToALFromImmed8>](byte) => write!(f, concat!($name, " al, {:X}h"), byte),
                            Self::[<$prefix ToAXFromImmed16>](word) => write!(f, concat!($name, " ax, {:X}h"), word),
                            Self::[<$prefix ToModRmFromImmed8>](mod_rm, immed) => write!(f, concat!($name, " {}, {:X}h"), mod_rm, immed),
                            Self::[<$prefix ToModRmFromImmed16>](mod_rm, immed) => write!(f, concat!($name, " {}, {:X}h"), mod_rm, immed),
                        )*

                        Self::SROverride(sr) => write!(f, "{}", sr),
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
                        },
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
                        },

                        Self::PushSR(sr) => write!(f, "push {}", sr),
                        Self::PushGR16(reg) => write!(f, "push {}", reg),
                        // Self::PushMem16(mem_index) => write!(f, "push word {}", mem_index),
                        Self::PushModRm16(mod_rm) => write!(f, "push {}", mod_rm),

                        Self::PopSR(sr) => write!(f, "pop {}", sr),
                        Self::PopGR16(reg) => write!(f, "pop {}", reg),
                        Self::PopModRm(mod_rm) => write!(f, "pop {}", mod_rm),

                        Self::Daa => write!(f, "daa"),
                        Self::Das => write!(f, "das"),
                        Self::Aaa => write!(f, "aaa"),
                        Self::Aas => write!(f, "aas"),

                        Self::IncGR16(reg) => write!(f, "inc {}", reg),
                        Self::IncModRm8(mod_rm) => write!(f, "inc {}", mod_rm),
                        Self::IncModRm16(mod_rm) => write!(f, "inc {}", mod_rm),
                        Self::DecGR16(reg) => write!(f, "dec {}", reg),
                        Self::DecModRm8(mod_rm) => write!(f, "dec {}", mod_rm),
                        Self::DecModRm16(mod_rm) => write!(f, "dec {}", mod_rm),

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

                        Self::MovFromSR(mod_rm, reg) => write!(f, "mov {}, {}", mod_rm, reg),
                        Self::MovToSR(reg, mod_rm) => write!(f, "mov {}, {}", reg, mod_rm),
                        Self::MovToALFromMem8(mem_index) => write!(f, "mov al, byte {}", mem_index),
                        Self::MovToAXFromMem16(mem_index) => write!(f, "mov ax, word {}", mem_index),
                        Self::MovToMem8FromAL(mem_index) => write!(f, "mov byte {}, al", mem_index),
                        Self::MovToMem16FromAL(mem_index) => write!(f, "mov word {}, ax", mem_index),
                        Self::MovToGR8FromImmed8(reg, immed) => write!(f, "mov {}, {:X}h", reg, immed),
                        Self::MovToGR16FromImmed16(reg, immed) => write!(f, "mov {}, {:X}h", reg, immed),
                        Self::MovToMem8FromImmed8(mod_rm, immed) => write!(f, "mov {}, {:X}h", mod_rm, immed),
                        Self::MovToMem16FromImmed16(mod_rm, immed) => write!(f, "mov {}, {:X}h", mod_rm, immed),

                        Self::LeaToGR16(reg, mem_index) => write!(f, "lea {}, {}", reg, mem_index),

                        Self::Nop => write!(f, "nop"),

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

                        Self::MovS8(sr) => {
                            match sr {
                                Some(sr) => write!(f, "{} movsb", sr),
                                None => write!(f, "movsb"),
                            }
                        },
                        Self::MovS16(sr) => {
                            match sr {
                                Some(sr) => write!(f, "{} movsw", sr),
                                None => write!(f, "movsw"),
                            }
                        },
                        Self::CmpS8(sr) => {
                            match sr {
                                Some(sr) => write!(f, "{} cmpsb", sr),
                                None => write!(f, "cmpsb"),
                            }
                        },
                        Self::CmpS16(sr) => {
                            match sr {
                                Some(sr) => write!(f, "{} cmpsw", sr),
                                None => write!(f, "cmpsw"),
                            }
                        },
                        Self::StoS8(sr) => {
                            match sr {
                                Some(sr) => write!(f, "{} stosb", sr),
                                None => write!(f, "stosb"),
                            }
                        },
                        Self::StoS16(sr) => {
                            match sr {
                                Some(sr) => write!(f, "{} stosw", sr),
                                None => write!(f, "stosw"),
                            }
                        },
                        Self::LodS8(sr) => {
                            match sr {
                                Some(sr) => write!(f, "{} lodsb", sr),
                                None => write!(f, "lodsb"),
                            }
                        },
                        Self::LodS16(sr) => {
                            match sr {
                                Some(sr) => write!(f, "{} lodsw", sr),
                                None => write!(f, "lodsw"),
                            }
                        },
                        Self::ScaS8(sr) => {
                            match sr {
                                Some(sr) => write!(f, "{} scasb", sr),
                                None => write!(f, "scasb"),
                            }
                        },
                        Self::ScaS16(sr) => {
                            match sr {
                                Some(sr) => write!(f, "{} scasw", sr),
                                None => write!(f, "scasw"),
                            }
                        },

                        Self::RetIntraSegImmed16(immed) => write!(f, "retn {:X}h", immed),
                        Self::RetIntraSeg => write!(f, "retn"),
                        Self::RetInterSeg => write!(f, "retf"),
                        Self::RetInterSegImmed16(immed) => write!(f, "retf {:X}h", immed),

                        Self::LesToReg(reg, mem_index) => write!(f, "les {}, dword {}", reg, mem_index),
                        Self::LdsToReg(reg, mem_index) => write!(f, "lds {}, dword {}", reg, mem_index),

                        Self::Int3 => write!(f, "int3"),
                        Self::IntFromImmed8(immed) => write!(f, "int {:X}h", immed),

                        Self::Into => write!(f, "into"),

                        Self::Iret => write!(f, "iret"),

                        Self::RolToModRm8(mod_rm) => write!(f, "rol {}", mod_rm),
                        Self::RorToModRm8(mod_rm) => write!(f, "ror {}", mod_rm),
                        Self::RclToModRm8(mod_rm) => write!(f, "rcl {}", mod_rm),
                        Self::RcrToModRm8(mod_rm) => write!(f, "rcr {}", mod_rm),
                        Self::ShlToModRm8(mod_rm) => write!(f, "shl {}", mod_rm),
                        Self::ShrToModRm8(mod_rm) => write!(f, "shr {}", mod_rm),
                        Self::SetmoToModRm8(mod_rm) => write!(f, "setmo {}", mod_rm),
                        Self::SarToModRm8(mod_rm) => write!(f, "sar {}", mod_rm),
                        Self::RolToModRm16(mod_rm) => write!(f, "rol {}", mod_rm),
                        Self::RorToModRm16(mod_rm) => write!(f, "ror {}", mod_rm),
                        Self::RclToModRm16(mod_rm) => write!(f, "rcl {}", mod_rm),
                        Self::RcrToModRm16(mod_rm) => write!(f, "rcr {}", mod_rm),
                        Self::ShlToModRm16(mod_rm) => write!(f, "shl {}", mod_rm),
                        Self::ShrToModRm16(mod_rm) => write!(f, "shr {}", mod_rm),
                        Self::SetmoToModRm16(mod_rm) => write!(f, "setmo {}", mod_rm),
                        Self::SarToModRm16(mod_rm) => write!(f, "sar {}", mod_rm),
                        Self::RolToModRm8CL(mod_rm) => write!(f, "rol {}, cl", mod_rm),
                        Self::RorToModRm8CL(mod_rm) => write!(f, "ror {}, cl", mod_rm),
                        Self::RclToModRm8CL(mod_rm) => write!(f, "rcl {}, cl", mod_rm),
                        Self::RcrToModRm8CL(mod_rm) => write!(f, "rcr {}, cl", mod_rm),
                        Self::ShlToModRm8CL(mod_rm) => write!(f, "shl {}, cl", mod_rm),
                        Self::ShrToModRm8CL(mod_rm) => write!(f, "shr {}, cl", mod_rm),
                        Self::SetmoToModRm8CL(mod_rm) => write!(f, "setmoc {}, cl", mod_rm),
                        Self::SarToModRm8CL(mod_rm) => write!(f, "sar {}, cl", mod_rm),
                        Self::RolToModRm16CL(mod_rm) => write!(f, "rol {}, cl", mod_rm),
                        Self::RorToModRm16CL(mod_rm) => write!(f, "ror {}, cl", mod_rm),
                        Self::RclToModRm16CL(mod_rm) => write!(f, "rcl {}, cl", mod_rm),
                        Self::RcrToModRm16CL(mod_rm) => write!(f, "rcr {}, cl", mod_rm),
                        Self::ShlToModRm16CL(mod_rm) => write!(f, "shl {}, cl", mod_rm),
                        Self::ShrToModRm16CL(mod_rm) => write!(f, "shr {}, cl", mod_rm),
                        Self::SetmoToModRm16CL(mod_rm) => write!(f, "setmoc {}, cl", mod_rm),
                        Self::SarToModRm16CL(mod_rm) => write!(f, "sar {}, cl", mod_rm),

                        Self::Aam(immed) => write!(f, "aam {:X}h", immed),
                        Self::Aad(immed) => write!(f, "aad {:X}h", immed),
                        Self::Xlat => write!(f, "xlat"),

                        Self::Esc(mod_rm) => write!(f, "esc {}", mod_rm),

                        Self::Loopne(short_label) => write!(f, "loopne {:04X}h", short_label),
                        Self::Loope(short_label) => write!(f, "loope {:04X}h", short_label),
                        Self::Loop(short_label) => write!(f, "loop {:04X}h", short_label),

                        Self::InToALFromImmed8(immed) => write!(f, "in al, {:X}h", immed),
                        Self::InToAXFromImmed8(immed) => write!(f, "in ax, {:X}h", immed),
                        Self::InToALFromDX => write!(f, "in al, dx"),
                        Self::InToAXFromDX => write!(f, "in ax, dx"),

                        Self::OutToPort8FromAL(immed) => write!(f, "out {:X}h, al", immed),
                        Self::OutToPort8FromAX(immed) => write!(f, "out {:X}h, ax", immed),
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

                        Self::NotToModRm8(mod_rm) => write!(f, "not {}", mod_rm),
                        Self::NegToModRm8(mod_rm) => write!(f, "neg {}", mod_rm),
                        Self::MulToModRm8(mod_rm) => write!(f, "mul {}", mod_rm),
                        Self::ImulToModRm8(mod_rm) => write!(f, "imul {}", mod_rm),
                        Self::DivToModRm8(mod_rm) => write!(f, "div {}", mod_rm),
                        Self::IdivToModRm8(mod_rm) => write!(f, "idiv {}", mod_rm),
                        Self::NotToModRm16(mod_rm) => write!(f, "not {}", mod_rm),
                        Self::NegToModRm16(mod_rm) => write!(f, "neg {}", mod_rm),
                        Self::MulToModRm16(mod_rm) => write!(f, "mul {}", mod_rm),
                        Self::ImulToModRm16(mod_rm) => write!(f, "imul {}", mod_rm),
                        Self::DivToModRm16(mod_rm) => write!(f, "div {}", mod_rm),
                        Self::IdivToModRm16(mod_rm) => write!(f, "idiv {}", mod_rm),

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
        }
    }
}
