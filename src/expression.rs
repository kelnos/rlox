use std::fmt;

use token::Token;
use value::Value;

pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: Value },
    Unary { operator: Token, right: Box<Expr> },
}

impl Expr {
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

    pub fn unary(operator: Token, right: Expr) -> Expr {
        Expr::Unary {
            operator,
            right: Box::new(right),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expr::Binary { ref left, ref operator, ref right } => write!(f, "{} {} {}", left, operator, right),
            &Expr::Grouping { ref expression } => write!(f, "({})", expression),
            &Expr::Literal { ref value } => write!(f, "{}", value),
            &Expr::Unary { ref operator, ref right } => write!(f, "{} {}", operator, right),
        }
    }
}
