use crate::builtin::BuiltInFn;

pub enum OpCode {
    Push(u64),
    Execute(BuiltInFn),
    NotFound(String),
}