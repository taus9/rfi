pub mod lexer;
pub mod data_stack;
pub mod word;
pub mod emitter;
pub mod opcode;

use crate::vm::data_stack::DataStack;
use crate::vm::opcode::OpCode;

pub struct Vm {
    pub data_stack: DataStack,
    pub output: String,
}


impl Vm {
    pub fn new() -> Self {
        Self {
            data_stack: DataStack::new(),
            output: String::new(),
        }
    }

    pub fn run(&mut self, codes: Vec<OpCode>) -> Result<(), String> {

        if !self.output.is_empty() {
            self.output = String::new();
        }

        for code in codes.iter(){
            match code {
                OpCode::Push(u) => self.data_stack.push(*u)?,
                OpCode::Execute(func) =>  func(self)?,
                OpCode::NotFound(s) => return Err(format!("{} not found", s)),
            }
        }

        Ok(())
    }  
}