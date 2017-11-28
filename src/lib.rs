#[macro_use]
extern crate lazy_static;

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

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

pub fn run(environment: Rc<RefCell<Environment>>, source: &String) -> Result<(), Vec<Box<Error>>> {
    scan(source).map_err(|error| vec![error]).and_then(|tokens| {
        //println!("tokens: {:?}", tokens);
        parse(tokens)
    }).and_then(|expr| {
        //println!("expr: {}", expr);
        interpret(environment, expr).map_err(|error| vec![error])
    })
}
