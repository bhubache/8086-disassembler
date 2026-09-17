use std::fs;

use crate::instruction::Instruction;

#[derive(Debug)]
pub enum DisassemblerError {
    InvalidOpcode(u8),
}

impl std::fmt::Display for DisassemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidOpcode(opcode) => write!(f, "invalid opcode `{:08b}`", opcode),
        }
    }
}

pub struct Disassembler {
    bytes: Vec<u8>,
    index: usize,
    instructions: Vec<Instruction>,
}

impl Disassembler {
    pub fn from_file(path: &str) -> Result<Disassembler, std::io::Error> {
        let bytes: Vec<u8> = fs::read(path)?;

        Ok(Disassembler::from_bytes(bytes))
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Disassembler {
        Disassembler {
            bytes,
            index: 0,
            instructions: Vec::new(),
        }
    }

    pub fn dump(&self) -> String {
        self.instructions
            .iter()
            .map(|inst| inst.opcode.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn disassemble(&mut self) -> Result<(), DisassemblerError> {
        Err(DisassemblerError::InvalidOpcode(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::io;
    use std::io::Write;

    use flate2::bufread::GzDecoder;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct TestSpec {
        name: String,
        bytes: Vec<u8>,
    }

    fn run_tests_in_file(opcode: &str) {
        dbg!(opcode);
        let filename = format!("/tmp/test_files/{}", opcode);
        let bytes = match fs::read(&filename) {
            Ok(bytes) => bytes,
            Err(err) => {
                match err.kind() {
                    io::ErrorKind::NotFound => {
                        let response = reqwest::blocking::get(format!("https://github.com/SingleStepTests/8088/raw/refs/heads/main/v2/{}.json.gz", opcode)).unwrap();
                        let bytes = response.bytes().unwrap();

                        let decoder = GzDecoder::new(&bytes[..]);
                        let test_list: Vec<TestSpec> = serde_json::from_reader(decoder).unwrap();

                        let parent_dir = std::path::Path::new(&filename).parent().unwrap();
                        fs::create_dir_all(parent_dir).unwrap();

                        let file = fs::File::create(filename)
                            .expect("Should be able to create the file as it won't exist");
                        let mut buf_writer = io::BufWriter::new(file);

                        let _ = serde_json::to_writer_pretty(&mut buf_writer, &test_list);

                        buf_writer.flush().unwrap();

                        serde_json::to_vec(&test_list).unwrap()
                    }
                    _ => panic!("unexpected error when reading file {}: {}", filename, err),
                }
            }
        };

        let test_list: Vec<TestSpec> = serde_json::from_slice(&bytes).unwrap();

        let mut num_wrong = 0;
        for test_spec in test_list {
            let mut disassembler = Disassembler::from_bytes(test_spec.bytes.clone());
            disassembler.disassemble().unwrap();
            let observed_name = disassembler.dump();

            if observed_name != test_spec.name {
                num_wrong += 1;

                println!(
                    "observed: \"{}\"\nexpected: {:#?}\n",
                    observed_name, test_spec
                );
            }
        }

        dbg!(opcode);
        assert_eq!(num_wrong, 0);
    }

    #[test]
    fn test_a_lot() {
        for opcode in 0x00..0x01 {
            run_tests_in_file(&format!("{:02X}", opcode));
        }
    }
}
