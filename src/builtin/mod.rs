pub mod core;

use crate::vm::data_stack::DataStack;
pub struct BuiltIn;

impl BuiltIn {

    pub fn is_builtin(word: &str) -> bool {
        match word {
            "+" => true, "-" => true,
            "." => true,
            _ => false,
        }
    }

    pub fn get_func(word: &String) -> fn(&mut DataStack) -> Result<(), String> {
        core::get_func(word).unwrap_or_else(|| {
            fn noop(_ds: &mut DataStack) -> Result<(), String> { Ok(()) }
            noop
        })
    }
}

