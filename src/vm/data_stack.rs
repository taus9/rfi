pub struct DataStack {
    stack: Vec<u64>
}

impl DataStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::<u64>::new(),
        }
    }

    pub fn push(&mut self, u: u64) {
        self.stack.push(u);
    }

    pub fn pop(&mut self) -> Option<u64> {
        self.stack.pop()
    }

    pub fn to_string(&self) -> String {
        self.stack.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}