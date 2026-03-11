use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal;

use std::io::{self, Write};

use crate::vm::lexer::Lexer;
use crate::vm::emitter::Emitter;
use crate::vm::VM;

mod builtin;
mod vm;

const INTRO: &str = "->   rusty forth interpreter 0.1.0   <-\n-> type quit or press ctrl+c to exit <-";
const PROMPT: &str = "-> ";
const QUIT: &str = "quit";
const ERROR: &str = "rfi error: ";


fn main() {

    println!("{}", &INTRO);
    
    let mut vm = VM::new(Box::new(io::stdout()));

    loop {
        // print prompt '-> '
        print!("{}", &PROMPT);
        io::stdout().flush().unwrap();
        
        // get input
        let input = read_input();

        // TODO: make quit into an OpCode
        if input == QUIT {
            break;
        }
        
        // get words from lexer
        let words = Lexer::tokenize(input);
        if words.is_empty() {
            continue;
        }

        // emit opcodes from words
        let codes = Emitter::emit(words);

        // run opcodes in vm
        match vm.run(codes) {
            Ok(()) => {
                print!(" ok\n");
                io::stdout().flush().unwrap();
            }
            Err(msg) => println!("\n{}{}", ERROR, msg),
        }
    }
    println!();
}

fn read_input() -> String {
    let mut input = String::new();
    terminal::enable_raw_mode().unwrap();

    loop {
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read().unwrap() {
            match code {
                KeyCode::Enter => break,
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                    terminal::disable_raw_mode().unwrap();
                    std::process::exit(0);
                },
                KeyCode::Char(c) => {
                    input.push(c);
                    print!("{}", c);
                    io::stdout().flush().unwrap();
                },
                KeyCode::Backspace => {
                    if !input.is_empty() {
                        input.pop();
                        print!("\x08 \x08"); // move back, erase, move back
                        io::stdout().flush().unwrap();
                    }
                },
                _ => {},
            }
        }
    }
    terminal::disable_raw_mode().unwrap();

    input
}