pub mod lexer;
pub mod data_stack;
pub mod word;
pub mod emitter;
pub mod opcode;

use std::io::Write;

use crate::vm::data_stack::DataStack;
use crate::vm::opcode::OpCode;

pub struct VM {
    pub data_stack: DataStack,
    pub output: Option<String>,
    writer: Box<dyn Write>,
}

impl VM
 {
    pub fn new(writer: Box<dyn Write>) -> Self {
        Self {
            data_stack: DataStack::new(),
            output: None,
            writer,
        }
    }

    pub fn run(&mut self, codes: Vec<OpCode>) {
        for code in codes.iter() {
            let result = match code {
                OpCode::NoOp => Ok(()),
                OpCode::Push(u) => self.data_stack.push(*u),
                OpCode::Execute(func) =>  func(self),
            };

            
            match result {
                Ok(_) => (),
                Err(msg) => println!("rfi error: {msg}"),
            }
        }
    }  
}