use std::io::{self, Write};

use crate::vm::lexer::Lexer;
use crate::vm::emitter::Emitter;
use crate::vm::VM;

mod builtin;
mod vm;

fn main() {

    let rfi_intro_message = String::from("-> rusty forth interpreter <-");
    let rfi_prompt = String::from("-> ");

    println!("{}", &rfi_intro_message);
    
    let mut vm = VM::new();

    loop {
        // print prompt '-> '
        print!("{}", &rfi_prompt);
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
