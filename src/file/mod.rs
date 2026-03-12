use std::fs;
use std::io;
use std::io::{BufRead, BufReader};

use crate::vm::VM;
use crate::runner::Runner;

pub struct File;

impl File {

    pub fn run(file: String) -> Result<(), String> {
        let lines = Self::read_file(&file)?;

        let mut vm = VM::new(Box::new(io::stdout()));
        let mut runner = Runner::new(&mut vm);

        for line in &lines {
            runner.run(line);
        }   

        Ok(())
    }

    fn read_file(file: &str) -> Result<Vec<String>, String> {
        let file = match fs::File::open(file) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };

        let lines = BufReader::new(file)
            .lines()
            .flatten()
            .collect::<Vec<String>>();
        
        Ok(lines)
    }

}