use std::io::{self, Write};

mod builtin;
mod vm;

fn main() {

    let rfi_intro_message = String::from("-> rusty forth interpreter <-");
    let rfi_prompt = String::from("-> ");

    println!("{}", &rfi_intro_message);


    let mut vm = vm::VM::new();

    
    loop {
        print!("{}", &rfi_prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("rfi - system error");
        
        vm.run(input.trim().to_string());

    }
}
