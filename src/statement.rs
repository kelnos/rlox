use std::fmt;

use expression::Expr;
use token::Token;

pub enum Stmt {
    Block { statements: Vec<Stmt> },
    Expression { expression: Box<Expr> },
    If { expression: Box<Expr>, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    Print { expression: Box<Expr> },
    Var { name: Token, initializer: Option<Expr> },
}

impl Stmt {
    pub fn block(statements: Vec<Stmt>) -> Stmt {
        Stmt::Block {
            statements,
        }
    }

    pub fn expression(expr: Expr) -> Stmt {
        Stmt::Expression {
            expression: Box::new(expr),
        }
    }

    pub fn if_(expr: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Stmt {
        Stmt::If {
            expression: Box::new(expr),
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(|eb| Box::new(eb)),
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
            Block { .. } => write!(f, "[block]"),
            Expression { .. } => write!(f, "[expression]"),
            If { .. } => write!(f, "[if-then-else]"),
            Print { .. } => write!(f, "[print]"),
            Var { ref name, .. } => write!(f, "[decl {}]", name.lexeme),
        }
    }
}
