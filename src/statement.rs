use std::fmt;

use expression::Expr;
use token::Token;

#[derive(Clone)]
pub enum Stmt {
    Block { statements: Vec<Stmt> },
    Expression { expression: Expr },
    For { initializer: Option<Box<Stmt>>, condition: Expr, increment: Option<Box<Stmt>>, body: Box<Stmt> },
    If { expression: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    Print { expression: Expr },
    Var { name: Token, initializer: Option<Expr> },
}

impl Stmt {
    pub fn block(statements: Vec<Stmt>) -> Stmt {
        Stmt::Block {
            statements,
        }
    }

    pub fn expression(expression: Expr) -> Stmt {
        Stmt::Expression {
            expression,
        }
    }

    pub fn if_(expression: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Stmt {
        Stmt::If {
            expression,
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(|eb| Box::new(eb)),
        }
    }

    pub fn print(expression: Expr) -> Stmt {
        Stmt::Print {
            expression,
        }
    }

    pub fn var(name: Token, initializer: Option<Expr>) -> Stmt {
        Stmt::Var {
            name,
            initializer,
        }
    }

    pub fn for_(initializer: Option<Stmt>, condition: Expr, increment: Option<Stmt>, body: Stmt) -> Stmt {
        Stmt::For {
            initializer: initializer.map(|i| Box::new(i)),
            condition,
            increment: increment.map(|i| Box::new(i)),
            body: Box::new(body),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Stmt::*;
        match *self {
            Block { .. } => write!(f, "[block]"),
            Expression { .. } => write!(f, "[expression]"),
            For { .. } => write!(f, "[for/while-loop]"),
            If { .. } => write!(f, "[if-then-else]"),
            Print { .. } => write!(f, "[print]"),
            Var { ref name, .. } => write!(f, "[decl {}]", name.lexeme),
        }
    }
}
