use crate::builtin::BuiltInFn;

pub enum OpCode {
    NoOp,
    Push(u64),
    Execute(BuiltInFn),
}