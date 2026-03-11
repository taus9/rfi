use crate::{args::Args, repl::Repl};

mod builtin;
mod vm;
mod args;
mod repl;

const RUN_FILE: &str = "-f";

fn main() {

    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut args = Args::new(args);
    
    // start the repl
    if args.is_empty() {

        Repl::start();
    
    } else {

        while let Some(arg) =  args.next_arg() {
            match arg.as_str() {
                RUN_FILE => {
                    println!("run file!");
                }
                _ => {
                    println!("unknown command '{}'", arg);
                }
            }
        }

    }
   
}

