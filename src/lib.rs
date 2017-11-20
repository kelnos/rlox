#[macro_use]
extern crate lazy_static;

use std::error::Error;

pub mod callable;
pub mod function;
pub mod scanner;
pub mod token;
pub mod value;

use scanner::scan;

pub fn run(source: &String) -> Result<(), Box<Error>> {
    scan(source).map(|tokens| {
        println!("tokens: {:?}", tokens);
        ()
    })
}
