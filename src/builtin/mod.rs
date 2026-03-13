pub mod core;

use crate::vm::Vm;

pub type BuiltInFn = fn(&mut Vm) -> Result<(), String>;

#[derive(Debug)]
pub struct BuiltInFlags(u32);

impl BuiltInFlags {
    pub const NONE:      Self = Self(0);
    pub const IMMEDIATE: Self = Self(1 << 0);
    //pub const HIDDEN:    Self = Self(1 << 1);
    pub fn has(&self, flag: BuiltInFlags) -> bool {
        self.0 & flag.0 != 0
    }
}

#[derive(Debug)]
pub struct BuiltIn {
    pub flags: BuiltInFlags,
    pub func:  BuiltInFn,
}

impl BuiltIn {
    pub fn get(word: &str) -> Option<BuiltIn> {
        core::get(word)
        // .or_else(|| core_ext::get(word))
    }
}