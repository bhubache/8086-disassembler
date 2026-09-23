#[macro_export]
macro_rules! parse_mod_reg_rm_8_from_reg {
    ( $self:expr, $opcode:expr, $prefixes:ident ) => {{
        let (mod_rm, reg) = $self.parse_mod_reg_rm($prefixes)?;

        Operation::Width8($opcode(mod_rm, reg))
    }};
}

#[macro_export]
macro_rules! parse_mod_reg_rm_16_from_reg {
    ( $self:expr, $opcode:expr, $prefixes:ident ) => {{
        let (mod_rm, reg) = $self.parse_mod_reg_rm($prefixes)?;

        Operation::Width16($opcode(mod_rm, reg))
    }};
}

#[macro_export]
macro_rules! parse_mod_reg_rm_8_to_reg {
    ( $self:expr, $opcode:expr, $prefixes:ident ) => {{
        let (mod_rm, reg) = $self.parse_mod_reg_rm($prefixes)?;

        Operation::Width8($opcode(reg, mod_rm))
    }};
}

#[macro_export]
macro_rules! parse_mod_reg_rm_16_to_reg {
    ( $self:expr, $opcode:expr, $prefixes:ident ) => {{
        let (mod_rm, reg) = $self.parse_mod_reg_rm($prefixes)?;

        Operation::Width16($opcode(reg, mod_rm))
    }};
}

#[macro_export]
macro_rules! op_8 {
    ( $opcode:expr ) => {{ Operation::Width8($opcode) }};
}

#[macro_export]
macro_rules! op_16 {
    ( $opcode:expr ) => {{ Operation::Width16($opcode) }};
}
