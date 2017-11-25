use std::fmt;

use expression::Expr;

pub enum Stmt {
    Expression { expression: Box<Expr> },
    Print { expression: Box<Expr> },
}

impl Stmt {
    pub fn expression(expr: Expr) -> Stmt {
        Stmt::Expression {
            expression: Box::new(expr),
        }
    }

    pub fn print(expr: Expr) -> Stmt {
        Stmt::Print {
            expression: Box::new(expr),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Stmt::*;
        match *self {
            Expression { .. } => write!(f, "[expression]"),
            Print { .. } => write!(f, "[print]"),
        }
    }
}
