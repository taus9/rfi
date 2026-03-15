use std::fs;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};

use crate::vm::Vm;
use crate::vm::emitter::Emitter;
use crate::vm::lexer::Lexer;

pub struct File;

impl File {
    pub fn run(file: String) -> Result<(), String> {
        let lines = Self::read_file(&file)?;

        let mut vm = Vm::new();

        for line in &lines {
            // get words from lexer
            // the tokenizers need to be passed the vm's dictionary
            // in order to be able to look for user defined words
            let words = Lexer::tokenize(line, &vm.dictionary)?;

            if words.is_empty() {
                continue;
            }

            // emit opcodes from words
            let codes = Emitter::emit(words);

            // run opcodes in vm
            match vm.run(codes) {
                Ok(()) => {
                    if !vm.output.is_empty() {
                        print!("{}", vm.output);
                        io::stdout().flush().unwrap();
                    }
                }

                Err(msg) => return Err(msg),
            }
        }

        Ok(())
    }

    fn read_file(file: &str) -> Result<Vec<String>, String> {
        let file = match fs::File::open(file) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };

        let lines = BufReader::new(file)
            .lines()
            .flatten()
            .collect::<Vec<String>>();

        Ok(lines)
    }
}
