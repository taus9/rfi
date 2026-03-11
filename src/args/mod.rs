pub struct Args {
    args: Vec<String>,
    pos: usize,
}


impl Args {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            args,
            pos: 0,
        }
    }

    pub fn next_arg(&mut self) -> Option<String> {
        if (self.pos + 1) > self.args.len() {
            return None;
        }

        self.pos += 1;

        return Some(self.args[self.pos - 1].to_string());
    }
}