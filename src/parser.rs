extern crate lazy_static;

use std::fmt;
use std::iter::Peekable;
use std::error::Error;
use std::slice::Iter;

use expression::Expr;
use token::TokenType::*;
use token::{TokenType, Token};

lazy_static! {
    static ref INVALID_EOF: Token = {
        Token::simple(Eof, 0)
    };
}

#[derive(Debug)]
pub struct ParseError<'a> {
    expected: Vec<TokenType>,
    found: Option<&'a Token>,
    description: String,
}

impl<'a> ParseError<'a> {
    pub fn new(expected: &Vec<TokenType>, found: Option<&'a Token>) -> ParseError<'a> {
        let expected_strings: Vec<&'static str> = expected.iter().map(|tt| tt.as_str()).collect();
        let line = found.map_or(0, |token| token.line);
        let description = format!("ERR:{}:unexpected token {}; expected {}", line, found.unwrap_or(&INVALID_EOF), expected_strings.join(", "));
        ParseError {
            expected: expected.to_vec(),
            found,
            description,
        }
    }

    pub fn line(&self) -> u32 {
        self.found.map_or(0, |token| token.line)
    }
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<'a> Error for ParseError<'a> {
    fn description(&self) -> &str {
        &self.description
    }
}

pub fn parse<'a>(tokens: &Vec<&'a Token>) -> Result<Expr<'a>, ParseError<'a>> {
    let mut iter = tokens.iter().peekable();
    parse_expression(&mut iter)
}

fn consume<'a>(iter: &mut Peekable<Iter<&'a Token>>, matches: &[TokenType]) -> Option<&'a Token> {
    if let Some(next) = iter.peek().map(|x| *x) {
        for tt in matches.iter() {
            if *tt == next.token_type {
                return iter.next().map(|x| *x);
            }
        }
    }
    None
}

fn parse_expression<'a>(iter: &mut Peekable<Iter<&'a Token>>) -> Result<Expr<'a>, ParseError<'a>> {
    parse_equality(iter)
}

fn parse_binary<'a>(iter: &mut Peekable<Iter<&'a Token>>, matches: &[TokenType], parse_operand: fn(&mut Peekable<Iter<&'a Token>>) -> Result<Expr<'a>, ParseError<'a>>) -> Result<Expr<'a>, ParseError<'a>> {
    let mut expr = parse_operand(iter)?;
    while let Some(operator) = consume(iter, matches) {
        expr = parse_operand(iter).map(|right| Expr::binary(expr, &operator, right))?;
    }
    Ok(expr)
}

fn parse_equality<'a>(iter: &mut Peekable<Iter<&'a Token>>) -> Result<Expr<'a>, ParseError<'a>> {
    parse_binary(iter, &[BangEqual, EqualEqual], parse_comparison)
}

fn parse_comparison<'a>(iter: &mut Peekable<Iter<&'a Token>>) -> Result<Expr<'a>, ParseError<'a>> {
    parse_binary(iter, &[Greater, GreaterEqual, Less, LessEqual], parse_addition)
}

fn parse_addition<'a>(iter: &mut Peekable<Iter<&'a Token>>) -> Result<Expr<'a>, ParseError<'a>> {
    parse_binary(iter, &[Minus, Plus], parse_multiplication)
}

fn parse_multiplication<'a>(iter: &mut Peekable<Iter<&'a Token>>) -> Result<Expr<'a>, ParseError<'a>> {
    parse_binary(iter, &[Slash, Star], parse_unary)
}

fn parse_unary<'a>(iter: &mut Peekable<Iter<&'a Token>>) -> Result<Expr<'a>, ParseError<'a>> {
    match consume(iter, &[Bang, Minus]) {
        Some(operator) => parse_unary(iter).map(|right| Expr::unary(&operator, right)),
        None => parse_primary(iter),
    }
}

lazy_static! {
    static ref EXPECT_PRIMARY: Vec<TokenType> = {
        vec![Number, Str, True, False, Nil]
    };
}

fn parse_primary<'a>(iter: &mut Peekable<Iter<&'a Token>>) -> Result<Expr<'a>, ParseError<'a>> {
    iter.next().map(|token| {
        match (&token.token_type, &token.literal) {
            (tt, &Some(ref value)) if EXPECT_PRIMARY.contains(tt) => Ok(Expr::literal(value)),
            _ => Err(ParseError::new(&*EXPECT_PRIMARY, Some(&token)))
        }
    }).unwrap_or(Err(ParseError::new(&*EXPECT_PRIMARY, None)))
}
