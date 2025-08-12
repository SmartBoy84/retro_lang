// note; address types fixed -> 128 instructions + 128 addresses

mod compile;
mod inst;

use std::{collections::HashMap, fmt::Display};

use crate::arch::MEM_SIZE;

// these instructions are for my lang - architecture independent, needed to work
#[derive(strum::Display, Clone, Copy)]
pub enum OpCode {
    LDC,
    LDM,
    ADC,
    ADM,
    STR,
    CLR,
    BRA,
    BRQ,
}

pub struct Inst {
    op: OpCode,
    val: u8,
}

struct UnresolvedJump<'a> {
    from: usize,
    to: &'a str,
    __debug_line: &'a str,
}

#[derive(Default)]
pub struct Program<'a> {
    stack: HashMap<&'a str, u8>,
    stack_i: u8,
    instructions: Vec<Inst>,
    jumps: HashMap<&'a str, u8>,
    unresolved_jumps: Vec<UnresolvedJump<'a>>,
}

#[derive(Debug)]
pub enum ProgramError {
    VarAlreadyDefined,
    UnsupportedOperator,
    VarUndefined,
    BadSyntax,
    NoLabelLen,
    BadVarName,
    ReservedName,
    LabelNotFound,
    LabelNotSpecified,
    EmptyLineLabelled
}

#[derive(Debug)]
pub struct CompileError<'a>(ProgramError, &'a str, usize); // error, line, line number

impl<'a> Program<'a> {
    pub fn new(stack_size: Option<u8>) -> Self {
        let stack_i = stack_size.map(|s| MEM_SIZE - s).unwrap_or(MEM_SIZE / 2);

        Self {
            stack_i,
            ..Default::default()
        }
    }

    pub fn get_inner(&self) -> &Vec<Inst> {
        &self.instructions
    }
}

impl Display for Program<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for Inst { op: inst, val: i } in &self.instructions {
            writeln!(f, "{inst} {i:02x}")?;
        }
        Ok(())
    }
}
