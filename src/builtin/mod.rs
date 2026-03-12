pub mod core;

use crate::vm::VM;
pub struct BuiltIn;

impl BuiltIn {

    pub fn get_func(word: &str) -> Option<fn(&mut VM) -> Result<(), String>> {
        core::get_func(word)
        // .or_else(|| core_ext::get_func(word))
    }
}

