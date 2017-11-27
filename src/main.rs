extern crate rlox;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process;

use rlox::environment::Environment;

struct Arguments {
    source_filename: Option<String>,
}

impl Arguments {
    fn new(mut args: env::Args) -> Result<Arguments, &'static str> {
        args.next();
        let source_filename = args.next();
        Ok(Arguments { source_filename })
    }
}

fn read_source_file(source_filename: &String) -> Result<String, io::Error> {
    File::open(source_filename).and_then(|mut f| {
        let mut s = String::new();
        f.read_to_string(&mut s).map(|_| s)
    })
}

fn main() {
    let arguments = Arguments::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to parse arguments: {}", err);
        process::exit(1);
    });

    let mut environment = Environment::new();

    match arguments.source_filename {
        Some(source_filename) => {
            println!("Running Lox file {}", source_filename);

            let source = read_source_file(&source_filename).unwrap_or_else(|err| {
                eprintln!("Failed to read file '{}': {}", source_filename, err);
                process::exit(1);
            });
            println!("Running Lox source\n{}", source);

            match rlox::run(&mut environment, &source) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                },
            }
        },
        None => {
            let stdin = io::stdin();
            print!("> ");
            io::stdout().flush().unwrap();
            for line in stdin.lock().lines() {
                match line {
                    Ok(source) => match rlox::run(&mut environment, &source) {
                        Ok(_) => (),
                        Err(e) => eprintln!("{}", e),
                    },
                    Err(e) => {
                        eprintln!("Failed to read from stdin: {}", e);
                        process::exit(1);
                    },
                };
                print!("> ");
                io::stdout().flush().unwrap();
            }

        }
    }
}
