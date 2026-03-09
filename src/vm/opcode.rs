use crate::vm::data_stack::DataStack;

pub enum OpCode {
    NoOp,
    Push(u64),
    Execute(fn(&mut DataStack)),
}