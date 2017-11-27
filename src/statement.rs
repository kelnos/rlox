use std::fmt;

use expression::Expr;
use token::Token;

pub enum Stmt {
    Expression { expression: Box<Expr> },
    Print { expression: Box<Expr> },
    Var { name: Token, initializer: Option<Expr> },
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

    pub fn var(name: Token, initializer: Option<Expr>) -> Stmt {
        Stmt::Var {
            name,
            initializer,
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Stmt::*;
        match *self {
            Expression { .. } => write!(f, "[expression]"),
            Print { .. } => write!(f, "[print]"),
            Var { ref name, .. } => write!(f, "[decl {}]", name.lexeme),
        }
    }
}
