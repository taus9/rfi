use std::io::{self, Write};


fn main() {

    let rfi_intro_message = String::from("-> rusty forth interpreter <-");
    println!("{}", &rfi_intro_message);
    
    loop {
        print!("-> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("rfi - system error");
        
    }
}
