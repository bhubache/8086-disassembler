use std::env;

use crate::disassembler::Disassembler;

mod disassembler;
mod instruction;
mod mode;
mod register;
mod rm;

// TODO: Provide polished CLI via clap
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut disassembler = Disassembler::from_file(file_path).unwrap();
    disassembler.disassemble().unwrap();

    println!("{}", disassembler.dump());
}
