pub mod lexer;
pub mod data_stack;
pub mod word;
pub mod emitter;
pub mod opcode;

use crate::vm::data_stack::DataStack;
use crate::vm::opcode::OpCode;

pub enum VmMode {
    Compile,
    Interpret,
}

pub struct Vm {
    pub data_stack: DataStack,
    pub output: String,
    pub mode: VmMode,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            data_stack: DataStack::new(),
            output: String::new(),
            mode: VmMode::Interpret,
        }
    }

    pub fn run(&mut self, codes: Vec<OpCode>) -> Result<(), String> {

        if !self.output.is_empty() {
            self.output = String::new();
        }

        for code in codes.iter(){
            match code {
                OpCode::Push(u) => self.data_stack.push(*u)?,
                OpCode::ExecuteBuiltIn(bi) => (bi.func)(self)?,
                OpCode::NotFound(s) => return Err(format!("{} not found", s)),
            }
        }

        Ok(())
    }  
}