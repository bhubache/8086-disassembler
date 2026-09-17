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

                            write!(f, "{}rep {}", sr_str, rep_str_inst)
                        },
                        Self::Repne(sr, rep_str_inst) => {
                            let sr_str = match sr {
                                Some(sr) => format!("{} ", sr),
                                None => String::new(),
                            };

                            write!(f, "{}repne {}", sr_str, rep_str_inst)
                        },

                        Self::PushSR(sr) => write!(f, "push {}", sr),
                        Self::PushGR16(reg) => write!(f, "push {}", reg),

                        Self::PopSR(sr) => write!(f, "pop {}", sr),
                        Self::PopGR16(reg) => write!(f, "pop {}", reg),
                        Self::PopModRm(mod_rm) => write!(f, "pop {}", mod_rm),

                        Self::Daa => write!(f, "daa"),
                        Self::Das => write!(f, "das"),
                        Self::Aaa => write!(f, "aaa"),
                        Self::Aas => write!(f, "aas"),

                        Self::IncGR16(reg) => write!(f, "inc {}", reg),
                        Self::DecGR16(reg) => write!(f, "dec {}", reg),

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

                        Self::RetImmed16(immed) => write!(f, "retn {:X}h", immed),
                        Self::RetIntraSeg => write!(f, "retn"),
                        Self::RetInterSeg(immed) => write!(f, "retf {:X}h", immed),

                        Self::LesToReg(reg, mem_index) => write!(f, "les {}, dword {}", reg, mem_index),
                        Self::LdsToReg(reg, mem_index) => write!(f, "lds {}, dword {}", reg, mem_index),
                    }
                }
            }
        }
    }
}
