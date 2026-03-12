pub mod core;

use crate::vm::VM;

pub type BuiltInFn = fn(&mut VM) -> Result<(), String>;

pub struct BuiltIn;

impl BuiltIn {

    pub fn get_func(word: &str) -> Option<BuiltInFn> {
        core::get_func(word)
        // .or_else(|| core_ext::get_func(word))
    }
}

