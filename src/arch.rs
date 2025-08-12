use crate::compiler::Compiler;

pub const MEM_SIZE: u8 = 255;

include!(concat!(env!("OUT_DIR"), "/inst_set.rs"));

pub const COMPILER: Compiler = Compiler::new(INST_NAMES, OP_CODES);
