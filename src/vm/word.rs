use crate::builtin::BuiltIn;
use crate::defined::UserDefinedWord;

pub enum Word {
    BuiltIn(BuiltIn), // Represents a built-in function or command
    Integer(u64), // Represents a numeric literal
    NotFound(String), // Represents a word that was not found in the built-in list
    Define(String), // Represents the start of a new word definition
    CallUserWord(String),
}

impl Word {

    pub fn define(word: String) -> Self {
        Word::Define(word)
    }

    pub fn from(word: String, dictionary: &Vec::<UserDefinedWord>) -> Self {

        if let Some(udw) = dictionary.iter().find(|w| w.read_name() == word) {
            return Word::CallUserWord(udw.read_name().to_string());
        }
        
        if let Some(func) = BuiltIn::get(&word) {
            return Word::BuiltIn(func);
        }
 
        if let Ok(value) = word.parse::<u64>() {
            return Word::Integer(value);
        }

        Word::NotFound(word)
    }

}
