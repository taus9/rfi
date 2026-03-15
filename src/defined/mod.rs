use crate::vm::opcode::OpCode;

#[derive(Debug)]
pub struct UserDefinedWord {
    name: String,
    codes: Vec<OpCode>,
}

impl UserDefinedWord {
    pub fn new(name: String, codes: Vec<OpCode>) -> Self {
        Self {
            name,
            codes,
        }
    }

    pub fn read_name(&self) -> &str {
        &self.name
    }

    pub fn read_codes(&self) -> &Vec<OpCode> {
        &self.codes
    }
}
