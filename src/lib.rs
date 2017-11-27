#[macro_use]
extern crate lazy_static;

use std::error::Error;

pub mod callable;
pub mod environment;
pub mod expression;
pub mod interpreter;
pub mod function;
pub mod parser;
pub mod scanner;
pub mod statement;
pub mod token;
pub mod value;

use environment::Environment;
use interpreter::interpret;
use parser::parse;
use scanner::scan;

pub fn run(environment: &mut Environment, source: &String) -> Result<(), Box<Error>> {
    scan(source).and_then(|tokens| {
        //println!("tokens: {:?}", tokens);
        parse(tokens)
    }).and_then(|expr| {
        //println!("expr: {}", expr);
        interpret(environment, expr)
    })
}
