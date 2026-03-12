use std::io;
use std::io::Write;

use crate::vm::lexer::Lexer;
use crate::vm::emitter::Emitter;
use crate::vm::VM;

const MSG_ERROR: &str = "rfi error: ";
const MSG_OK: &str = "ok";

pub struct Runner<'a> {
    vm: &'a mut VM,
}

impl<'a> Runner<'a> {
    pub fn new(vm: &'a mut VM) -> Self {
        Self { 
            vm,
         }    
    }

    pub fn run(&mut self, input: &str) {
        // get words from lexer
        let words = Lexer::tokenize(input);
        if words.is_empty() {
            return;
        }

        // emit opcodes from words
        let codes = Emitter::emit(words);

        // run opcodes in vm
        match self.vm.run(codes) {
            Ok(()) => {
                print!(" {}\n", MSG_OK);
                io::stdout().flush().unwrap();
            }
            Err(msg) => println!("\n{}{}", MSG_ERROR, msg),
        }
    }
}



