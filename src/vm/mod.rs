pub mod lexer;
pub mod data_stack;
pub mod word;
pub mod emitter;
pub mod opcode;

use crate::vm::lexer::Lexer;
use crate::vm::data_stack::DataStack;
use crate::vm::word::Word;
use crate::vm::emitter::Emitter;
use crate::vm::opcode::OpCode;

pub struct VM {
    lexer: Lexer,
    data_stack: DataStack,
    emitter: Emitter,
}

impl VM {
    pub fn new() -> Self {
        Self {
            lexer: Lexer::new(),
            data_stack: DataStack::new(),
            emitter: Emitter::new(),
        }
    }

    pub fn run(&mut self, input: String) {
        self.lexer.set_input(input);
        self.lexer.reset_pos();

        loop {

            if let Some(word) = self.lexer.next_word() {
                
                let word = Word::from(word);
                let opcode = self.emitter.emit(word);

                //TODO: check if in compile mode
                match opcode {
                    OpCode::NoOp => (),
                    OpCode::Push(u) => self.data_stack.push(u),
                    OpCode::Execute(func) => func(&mut self.data_stack),
                }


            } else {
                break;
            }
        }
    }    
}
