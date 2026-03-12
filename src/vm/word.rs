use crate::builtin::BuiltIn;
use crate::vm::VM;

pub enum Word {
    BuiltIn(fn(&mut VM) -> Result<(), String>),
    Integer(u64),
    Unknown(),
}

impl Word {
    pub fn from(word: String) -> Self {
        if let Some(func) = BuiltIn::get_func(&word) {
            return Word::BuiltIn(func);
        }

        if let Ok(value) = word.parse::<u64>() {
            return Word::Integer(value);
        }

        Word::Unknown()
    }
}