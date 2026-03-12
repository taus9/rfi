use std::path::Path;

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

const EXIT_FILE_NOT_FOUND: i32 = 1;
const EXIT_MISSING_ARG: i32    = 4;
const EXIT_NOT_A_FILE: i32     = 2;
const EXIT_RUNTIME_ERROR: i32  = 3;
const EXIT_UNKNOWN_ARG: i32    = 5;

const ERR_FILE_NOT_FOUND: &str = "file not found:";
const ERR_MISSING_ARG: &str = "missing filename arg:";
const ERR_NOT_A_FILE: &str = "not a valid file:";
const ERR_RUNTIME_ERROR: &str = "rfi error:";
const ERR_UNKNOWN_ARG: &str = "unknown command:";

const TXT_COMPLETE: &str = "-> program complete <-";
const TXT_HELP: &str = "\
Usage: rfi [options]

  -f <file>  Run a file
  -h         Show this help
  -v         Show version

  If no arguments are provided, the interactive REPL will start.";

struct RfiError(String, i32);

fn main() {

    // get command line arguments
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

                        validate_file(&file).unwrap_or_else(|err| {
                            eprintln!("{} {}", err.0, file);
                            std::process::exit(err.1);
                        });

                        match File::run(file) {
                            Ok(_) => println!("\n{}", TXT_COMPLETE),
                            Err(message) => {
                                eprintln!("{} {}", ERR_RUNTIME_ERROR, message);
                                std::process::exit(EXIT_RUNTIME_ERROR);
                            }
                        }

                    } else {
                        eprintln!("{}", ERR_MISSING_ARG);
                        std::process::exit(EXIT_MISSING_ARG);
                    }
                }

                DASH_H => {
                    println!("{}", TXT_HELP);
                }

                DASH_V => {
                    println!("{}", RFI_VERSION);
                }

                _ => {
                    eprintln!("{} {}", ERR_UNKNOWN_ARG, arg);
                    std::process::exit(EXIT_UNKNOWN_ARG);
                }
            }
        }

    }
   
}


fn validate_file(file: &str) -> Result<(), RfiError> {
    let path = Path::new(&file);

    if !path.exists() {
        let error = RfiError(ERR_FILE_NOT_FOUND.to_string(), EXIT_FILE_NOT_FOUND);
        return Err(error);
    }

    if !path.is_file() {
        let error = RfiError(ERR_NOT_A_FILE.to_string(), EXIT_NOT_A_FILE);
        return Err(error);
    }

    Ok(())
}