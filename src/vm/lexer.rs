use crate::vm::word::Word;

pub struct Lexer;

impl Lexer {
    pub fn tokenize(input: &str) -> Result<Vec<Word>, String> {
        let bytes = input.as_bytes();
        let mut pos = 0;
        let mut words = Vec::<Word>::new();
        loop {
            match Self::next_token(bytes, &mut pos) {
                Some(word) => {
                    // if we come across the ":" word when we are tokenizing/parsing
                    // when need to convert the very next word into a TextWord
                    // this new TextWord is the name of the function being defined 
                    // and this is what it will be looked up by.
                    if word == ":" {
                        let word_name = Self::next_token(bytes, &mut pos);
                        if word_name.is_none() {
                            return Err("missing word name".to_string());
                        }

                        let word_name = word_name.unwrap();
                        words.push(Word::define(word_name));
                        continue;
                    }

                    words.push(Word::from(word))
                }
                None => break,
            }
        }
        Ok(words)
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
