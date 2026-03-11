pub mod core;

use crate::vm::VM;
pub struct BuiltIn;

impl BuiltIn {

    pub fn is_builtin(word: &str) -> bool {
        match word {
            "+" => true, "-" => true,
            "." => true,
            _ => false,
        }
    }

    pub fn get_func(word: &String) -> fn(&mut VM) -> Result<(), String> {
        core::get_func(word).unwrap_or_else(|| {
            fn noop(_ds: &mut VM) -> Result<(), String> { Ok(()) }
            noop
        })
    }
}

