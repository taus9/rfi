use crate::builtin::BuiltIn;

pub enum Word {
    BuiltIn(BuiltIn), // Represents a built-in function or command
    Integer(u64), // Represents a numeric literal
    NotFound(String), // Represents a word that was not found in the built-in list
    Define(String), // Represents the start of a new word definition
}

impl Word {

    pub fn define(word: String) -> Self {
        Word::Define(word)
    }

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
