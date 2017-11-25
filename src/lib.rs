#[macro_use]
extern crate lazy_static;

use std::error::Error;

pub mod callable;
pub mod expression;
pub mod interpreter;
pub mod function;
pub mod parser;
pub mod scanner;
pub mod token;
pub mod value;

use interpreter::interpret;
use parser::parse;
use scanner::scan;
use value::Value;

pub fn run(source: &String) -> Result<Value, Box<Error>> {
    scan(source).and_then(|tokens| {
        println!("tokens: {:?}", tokens);
        parse(tokens)
    }).and_then(|expr| {
        println!("expr: {}", expr);
        interpret(expr)
    })
}
