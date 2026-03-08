use std::io::{self, Write};

struct Lexer {
    input: String,
    pos: usize
}


fn main() {

    let rfi_intro_message = String::from("-> rusty forth interpreter <-");
    let rfi_prompt = String::from("-> ");
    
    println!("{}", &rfi_intro_message);
    
    loop {
        print!("{}", &rfi_prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("rfi - system error");
        
    }
}
