use crate::builtin::BuiltIn;

pub enum Word {
    BuiltIn(String),
    Integer(u64),
    Unknown(String),
}

impl Word {
    pub fn from(word: String) -> Self {
        if BuiltIn::is_builtin(&word) {
            return Word::BuiltIn(word);
        }

        if let Ok(value) = word.parse::<u64>() {
            return Word::Integer(value);
        }

        Word::Unknown(word)
    }
}