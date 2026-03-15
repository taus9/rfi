pub mod data_stack;
pub mod emitter;
pub mod lexer;
pub mod opcode;
pub mod word;

use crate::builtin::BuiltInFlags;
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
    pub compile_buffer: Vec<OpCode>,
    pub compiling_word: String,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            compile_buffer: Vec::new(),
            data_stack: DataStack::new(),
            output: String::new(),
            mode: VmMode::Interpret,
            compiling_word: String::new(),
        }
    }

    pub fn run(&mut self, codes: Vec<OpCode>) -> Result<(), String> {
        if !self.output.is_empty() {
            self.output = String::new();
        }

        for code in codes {
            match self.mode {
                VmMode::Compile => {

                    if let OpCode::ExecuteBuiltIn(bi) = &code {
                        if bi.flags.has(BuiltInFlags::DEFINING) {
                            return Err("nested definitions are not supported".to_string());
                        } else if bi.flags.has(BuiltInFlags::IMMEDIATE) {
                            (bi.func)(self)?;
                            continue;
                        }
                    }

                    self.compile_buffer.push(code);
                    continue;
                }

                VmMode::Interpret => match code {
                    OpCode::Push(u) => self.data_stack.push(u)?,
                    OpCode::ExecuteBuiltIn(bi) => (bi.func)(self)?,
                    OpCode::NotFound(s) => return Err(format!("{} not found", s)),
                    OpCode::Define(s) => self.compiling_word = s,
                    OpCode::EndDefine() => (),
                },
            }
        }

        Ok(())
    }
}
