use crate::builtin::BuiltIn;

#[derive(Debug)]
pub enum OpCode {
    Push(u64),
    ExecuteBuiltIn(BuiltIn),
    NotFound(String),
    BeginDefine(),
    EndDefine(),
}
