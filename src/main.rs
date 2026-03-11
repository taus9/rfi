use crate::{args::Args, repl::Repl, file::File};

mod builtin;
mod vm;
mod args;
mod repl;
mod file;

const RFI_VERSION: &str = "rusty forth interpreter 0.1.0";

const DASH_F: &str = "-f";
const DASH_H: &str = "-h";
const DASH_V: &str = "-v";

const COMPLETE: &str = "-> program complete <-";
const USAGE: &str = "Usage: rfi -f [filename]";
const UNKNOWN: &str = "Unknown command:";
const HELP: &str = "Use -h to get help";

const HELP_TEXT: &str = "\
Usage: rfi [options]

  -f <file>  Run a file
  -h         Show this help
  -v         Show version

  If no arguments are provided, the interactive REPL will start.";

fn main() {

    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut args = Args::new(args);
    
    // start the repl
    if args.is_empty() {

        Repl::start();
    
    } else {

        while let Some(arg) =  args.next_arg() {
            match arg.as_str() {
                DASH_F => {
                    if let Some(file) = args.next_arg() {
                        
                        match File::run(&file) {
                            Ok(_) => println!("{}", COMPLETE),
                            Err(e) => eprintln!("{}", e),
                        }

                    } else {
                        eprintln!("{}", USAGE);
                    }
                }

                DASH_H => {
                    println!("{}", HELP_TEXT);
                }

                DASH_V => {
                    println!("{}", RFI_VERSION);
                }

                _ => {
                    println!("{} {}\n{}", UNKNOWN, arg, HELP);
                }
            }
        }

    }
   
}

