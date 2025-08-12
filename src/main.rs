mod arch;
mod compiler;
mod lang;

use crate::{arch::COMPILER, lang::Program};
use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);

    let retro_file = fs::read_to_string(args.next().expect("specify .retro file")).unwrap();

    let mut program = Program::new(None);
    program.append_instructions(&retro_file).unwrap();

    let output_file = args
        .skip_while(|s| s != "-o")
        .skip(1)
        .next()
        .expect("specify output file");

    println!("{program}");

    COMPILER
        .compile(&program.to_string())
        .unwrap()
        .to_file(&output_file)
        .unwrap();
}
