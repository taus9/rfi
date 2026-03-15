use crate::vm::word::Word;

pub struct Lexer;

impl Lexer {
    pub fn tokenize(input: &str) -> Vec<Word> {
        let bytes = input.as_bytes();
        let mut pos = 0;
        let mut words = Vec::<Word>::new();
        loop {
            match Self::next_token(bytes, &mut pos) {
                Some(word) => words.push(Word::from(word)),
                None => break,
            }
        }
        words
    }

    fn next_token(bytes: &[u8], pos: &mut usize) -> Option<String> {
        while *pos < bytes.len() && bytes[*pos].is_ascii_whitespace() {
            *pos += 1;
        }

        if *pos >= bytes.len() {
            return None; // End of input
        }

        let start = *pos;

        // Find end of word (until whitespace or end)
        while *pos < bytes.len() && !bytes[*pos].is_ascii_whitespace() {
            *pos += 1;
        }

        // Extract the word as a String (assuming valid UTF-8)
        let word_str = String::from_utf8_lossy(&bytes[start..*pos]).to_string();
        Some(word_str)
    }
}
