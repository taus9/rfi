use crate::builtin::BuiltIn;

pub enum OpCode {
    Push(u64),
    ExecuteBuiltIn(BuiltIn),
    NotFound(String),
}