use token::Token;
use value::Value;

pub enum Expr<'a> {
    Binary { left: Box<Expr<'a>>, operator: &'a Token, right: Box<Expr<'a>> },
    Grouping { expression: Box<Expr<'a>> },
    Literal { value: &'a Value },
    Unary { operator: &'a Token, right: Box<Expr<'a>> },
}

impl<'a> Expr<'a> {
    pub fn binary(left: Expr<'a>, operator: &'a Token, right: Expr<'a>) -> Expr<'a> {
        Expr::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn grouping(expression: Expr<'a>) -> Expr<'a> {
        Expr::Grouping {
            expression: Box::new(expression),
        }
    }

    pub fn literal(value: &'a Value) -> Expr<'a> {
        Expr::Literal {
            value,
        }
    }

    pub fn unary(operator: &'a Token, right: Expr<'a>) -> Expr<'a> {
        Expr::Unary {
            operator,
            right: Box::new(right),
        }
    }
}
