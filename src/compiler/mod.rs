pub mod compile;
pub mod asm;

#[derive(strum::Display, Clone, Copy)]
pub enum Inst {
    LDC,
    LDM,
    ADC,
    ADM,
    STR,
    CLR,
    BRA,
    BZ,
}

pub struct Compiler<'a> {
    inst_set: &'a [&'a str],
    op_codes: &'a [&'a str],
}

#[allow(dead_code)] // why do I get deacode warning, even though values used when I call .unwrap()
#[derive(Debug)]
pub enum CompilerError<'a> {
    NewLine,
    UnknownInst(&'a str),
    ShortHex(&'a str),
}

impl<'a> Compiler<'a> {
    pub const fn new(inst_set: &'a [&str], op_codes: &'a [&str]) -> Self {
        assert!(inst_set.len() == op_codes.len());
        Self { inst_set, op_codes }
    }
}
