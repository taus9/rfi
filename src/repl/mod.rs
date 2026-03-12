use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal;

use std::io::{self, Write};

use crate::vm::VM;
use crate::runner::Runner;



pub struct Repl;

const INTRO: &str = "->   rusty forth interpreter 0.1.0   <-\n-> type quit or press ctrl+c to exit <-";
const PROMPT: &str = "-> ";
const QUIT: &str = "quit";

impl Repl {

    pub fn start() {
        println!("{}", INTRO);

        println!("{}", PROMPT);

        let mut vm = VM::new(Box::new(io::stdout()));
        let mut runner = Runner::new(&mut vm);

        loop {
            // print prompt '-> '
            print!("{}", PROMPT);
            io::stdout().flush().unwrap();

            // get input
            let input = Self::read_input();

            if input.trim() == "" {
                println!();
            }

            // TODO: make quit into an OpCode
            if input == QUIT {
                break;
            }

            runner.run(&input);

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
                        println!("\x08\x08\x08");
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

}