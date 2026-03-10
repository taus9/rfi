use crate::vm::VM;

pub enum OpCode {
    NoOp,
    Push(u64),
    Execute(fn(&mut VM) -> Result<(), String>),
}