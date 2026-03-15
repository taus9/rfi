use crate::{builtin::BuiltIn};

#[derive(Debug, Clone)]
pub enum OpCode {
    Push(u64),
    ExecuteBuiltIn(BuiltIn),
    NotFound(String),
    Define(String, BuiltIn),
    CallUserWord(String),
}
