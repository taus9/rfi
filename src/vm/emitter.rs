use crate::vm::word::Word;
use crate::vm::opcode::OpCode;

pub struct Emitter;

impl Emitter {

    pub fn emit(words: Vec<Word>) -> Vec<OpCode> {
        words.into_iter()
            .map(|w| Self::transform(w))
            .collect()
    }

    fn transform(word: Word) -> OpCode {
        match word {
            Word::Integer(u) => OpCode::Push(u),
            Word::BuiltIn(f) => OpCode::Execute(f),
            _ => OpCode::NoOp,

        }
    }
}

