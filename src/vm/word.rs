use crate::builtin::{BuiltIn};

pub enum Word {
    BuiltIn(BuiltIn),
    Integer(u64),
    NotFound(String),
}

impl Word {
    pub fn from(word: String) -> Self {
        if let Some(func) = BuiltIn::get(&word) {
            return Word::BuiltIn(func);
        }

        if let Ok(value) = word.parse::<u64>() {
            return Word::Integer(value);
        }

        Word::NotFound(word)
    }
}