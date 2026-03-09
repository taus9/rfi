pub struct Lexer {
    input: Option<String>,
    bytes: Option<Vec<u8>>,
    pos: usize
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            input: None,
            bytes: None,
            pos: 0
        }
    }

    pub fn set_input(&mut self, input: String) {
        let bytes = input.as_bytes().to_vec();
        self.input = Some(input);
        self.bytes = Some(bytes);
    }

    pub fn reset_pos(&mut self) {
        self.pos = 0;
    }

    pub fn next_word(&mut self) -> Option<String> {
        if let Some(ref bytes) = self.bytes {
            
            while self.pos < bytes.len() && bytes[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }
            
            if self.pos >= bytes.len() {
                return None;  // End of input
            }
            
            let start = self.pos;
            
            // Find end of word (until whitespace or end)
            while self.pos < bytes.len() && !bytes[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }
            
            // Extract the word as a String (assuming valid UTF-8)
            let word_str = String::from_utf8_lossy(&bytes[start..self.pos]).to_string();
            Some(word_str)
        } else {
            None
        }        
    
    }

}