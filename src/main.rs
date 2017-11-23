extern crate rlox;

use std::env;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;
use std::process;

struct Arguments {
    source_filename: String,
}

impl Arguments {
    fn new(mut args: env::Args) -> Result<Arguments, &'static str> {
        args.next();
        let source_filename = match args.next() {
            Some(arg) => arg,
            None => return Err("First argument must be a file name"),
        };
        Ok(Arguments { source_filename })
    }
}

fn read_source_file(source_filename: &String) -> Result<String, Error> {
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
    println!("Running Lox file {}", arguments.source_filename);

    let source = read_source_file(&arguments.source_filename).unwrap_or_else(|err| {
        eprintln!("Failed to read file '{}': {}", arguments.source_filename, err);
        process::exit(1);
    });
    println!("Running Lox source\n{}", source);

    match rlox::run(&source) {
        Ok(result) => println!("Ran; result: {}", result),
        Err(e) => {
            eprintln!("Failed to run: {}", e);
            process::exit(1);
        },
    }
}
