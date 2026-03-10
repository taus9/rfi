use std::io::{self, Write};

use crate::vm::lexer::Lexer;
use crate::vm::emitter::Emitter;
use crate::vm::VM;

mod builtin;
mod vm;

const INTRO: &str = "-> rusty forth interpreter <-";
const PROMPT: &str = "-> ";

fn main() {

    println!("{}", &INTRO);
    
    let mut vm = VM::new(Box::new(io::stdout()));

    loop {
        // print prompt '-> '
        print!("{}", &PROMPT);
        io::stdout().flush().unwrap();
        
        // get input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("rfi - system error");
        
        
        // get words from lexer
        let words = Lexer::tokenize(input);
        if words.is_empty() {
            continue;
        }

        // emit opcodes from words
        let codes = Emitter::emit(words);

        // run opcodes in vm
        vm.run(codes);
        
    }
}
