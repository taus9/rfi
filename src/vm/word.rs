use crate::builtin::{BuiltIn, BuiltInFn};

pub enum Word {
    BuiltIn(BuiltInFn),
    Integer(u64),
    NotFound(String),
}

impl Word {
    pub fn from(word: String) -> Self {
        if let Some(func) = BuiltIn::get_func(&word) {
            return Word::BuiltIn(func);
        }

        if let Ok(value) = word.parse::<u64>() {
            return Word::Integer(value);
        }

        Word::NotFound(word)
    }
}