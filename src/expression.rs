use std::fmt;

use token::Token;
use value::Value;

#[derive(Clone)]
pub enum Expr {
    Assign { name: Token, value: Box<Expr> },
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: Value },
    Logical { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Unary { operator: Token, right: Box<Expr> },
    Variable { name: Token },
}

impl Expr {
    pub fn assign(name: Token, value: Expr) -> Expr {
        Expr::Assign {
            name,
            value: Box::new(value),
        }
    }

    pub fn binary(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn grouping(expression: Expr) -> Expr {
        Expr::Grouping {
            expression: Box::new(expression),
        }
    }

    pub fn literal(value: Value) -> Expr {
        Expr::Literal {
            value,
        }
    }

    pub fn logical(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Logical {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn unary(operator: Token, right: Expr) -> Expr {
        Expr::Unary {
            operator,
            right: Box::new(right),
        }
    }

    pub fn variable(name: Token) -> Expr {
        Expr::Variable {
            name,
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expr::Assign { ref name, ref value } => write!(f, "{} = {}", name.lexeme, value),
            &Expr::Binary { ref left, ref operator, ref right } => write!(f, "{} {} {}", left, operator, right),
            &Expr::Grouping { ref expression } => write!(f, "({})", expression),
            &Expr::Literal { ref value } => write!(f, "{}", value),
            &Expr::Logical { ref left, ref operator, ref right } => write!(f, "{} {} {}", left, operator, right),
            &Expr::Unary { ref operator, ref right } => write!(f, "{} {}", operator, right),
            &Expr::Variable { ref name } => write!(f, "{}", name),
        }
    }
}
