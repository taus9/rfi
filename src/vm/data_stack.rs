pub struct DataStack {
    stack: Vec<u64>
}

impl DataStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::<u64>::new(),
        }
    }

    pub fn push(&mut self, u: u64) -> Result<(), String> {
        self.stack.push(u);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<u64, String> {
        self.stack.pop().ok_or_else(|| "Stack underflow".to_string())
    }

}