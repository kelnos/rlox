#[macro_use]
extern crate lazy_static;

use std::error::Error;

pub mod callable;
pub mod expression;
pub mod function;
pub mod parser;
pub mod scanner;
pub mod token;
pub mod value;

use expression::Expr;
use parser::parse;
use scanner::scan;

pub fn run(source: &String) -> Result<Expr, Box<Error>> {
    scan(source).and_then(|tokens| {
        println!("tokens: {:?}", tokens);
        parse(tokens)
    })
}
