pub mod data_stack;
pub mod emitter;
pub mod lexer;
pub mod opcode;
pub mod word;

use crate::builtin::BuiltInFlags;
use crate::vm::data_stack::DataStack;
use crate::vm::opcode::OpCode;
use crate::defined::UserDefinedWord;

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
    pub dictionary: Vec<UserDefinedWord>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            compile_buffer: Vec::new(),
            data_stack: DataStack::new(),
            output: String::new(),
            mode: VmMode::Interpret,
            compiling_word: String::new(),
            dictionary: Vec::new(),
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
                    OpCode::Define(s, bi) => {
                        self.compiling_word = s;
                        (bi.func)(self)?
                    },
                    OpCode::EndDefine() => (),
                    OpCode::NotFound(s) => {
                        
                        return Err(format!("{} not found", s));
                    },
                    
                    OpCode::CallUserWord(s) => {
                        let udw = self.get_user_word(&s);
                        self.run(udw.read_codes());
                    },
                },
            }
        }

        Ok(())
    }

    pub fn register_user_word(&mut self, name: String, codes: Vec<OpCode>) {
        let word = UserDefinedWord::new(name, codes);
        self.dictionary.push(word);
    }

    pub fn get_user_word(&self, name: &str) -> &UserDefinedWord {
        self.dictionary.iter().find(|w| w.read_name() == name).unwrap()      
    }
    
}
