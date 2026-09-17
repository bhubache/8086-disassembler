#[macro_export]
macro_rules! parse_mod_reg_rm_8_from_reg {
    ( $self:expr, $mnemonic:expr, $sr_override:ident ) => {{
        let (mod_rm, reg) = $self.parse_mod_reg_rm($sr_override)?;

        $mnemonic(mod_rm, reg)
    }};
}

#[macro_export]
macro_rules! parse_mod_reg_rm_16_from_reg {
    ( $self:expr, $mnemonic:expr, $sr_override:ident ) => {{
        let (mod_rm, reg) = $self.parse_mod_reg_rm($sr_override)?;

        $mnemonic(mod_rm, reg)
    }};
}

#[macro_export]
macro_rules! parse_mod_reg_rm_8_to_reg {
    ( $self:expr, $mnemonic:expr, $sr_override:ident ) => {{
        let (mod_rm, reg) = $self.parse_mod_reg_rm($sr_override)?;

        $mnemonic(reg, mod_rm)
    }};
}

#[macro_export]
macro_rules! parse_mod_reg_rm_16_to_reg {
    ( $self:expr, $mnemonic:expr, $sr_override:ident ) => {{
        let (mod_rm, reg) = $self.parse_mod_reg_rm($sr_override)?;

        $mnemonic(reg, mod_rm)
    }};
}
