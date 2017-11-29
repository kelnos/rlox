extern crate lazy_static;

use std::fmt;
use std::iter::Peekable;
use std::error::Error;
use std::vec::IntoIter;

use expression::Expr;
use statement::Stmt;
use token::TokenType::*;
use token::{TokenType, Token};

lazy_static! {
    static ref EXPECT_PRIMARY: Vec<TokenType> = {
        vec![Number, Str, True, False, Nil, LeftParen, Identifier]
    };
}

#[derive(Debug)]
pub struct ParseError {
    expected: Vec<TokenType>,
    found: Token,
    description: String,
}

impl ParseError {
    pub fn new(expected: &Vec<TokenType>, found: Option<Token>) -> Box<ParseError> {
        let expected_strings: Vec<&'static str> = expected.iter().map(|tt| tt.as_str()).collect();
        let token = found.unwrap_or(Token::simple(Eof, 0));
        let description = format!("ERR:{}:unexpected token {}; expected {}", token.line, token.token_type, expected_strings.join(", "));
        Box::new(ParseError {
            expected: expected.to_vec(),
            found: token,
            description,
        })
    }

    fn new_arr(expected: &[TokenType], found: Option<Token>) -> Box<ParseError> {
        let mut v = Vec::new();
        v.extend(expected.iter().cloned());
        ParseError::new(&v, found)
    }

    pub fn line(&self) -> u32 {
        self.found.line
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.description
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, Vec<Box<Error>>> {
    let mut stmts = Vec::new();
    let mut errors = Vec::new();

    let mut iter = tokens.into_iter().peekable();
    loop {
        if next_is(&mut iter, &[TokenType::Eof]) {
            iter.next();
            break;
        } else if iter.peek().is_none() {
            break;
        } else {
            match declaration(&mut iter) {
                Ok(stmt) => stmts.push(stmt),
                Err(e) => {
                    errors.push(e);
                    synchronize(&mut iter)
                },
            }
        }
    }

    if errors.is_empty() {
        Ok(stmts)
    } else {
        Err(errors)
    }
}

fn synchronize(iter: &mut Peekable<IntoIter<Token>>) {
    use token::TokenType::*;

    while let Some(token) = iter.next() {
        if token.token_type == Semicolon || token.token_type == Eof {
            break;
        }

        if next_is(iter, &[Class, Fun, Var, For, If, While, Print, Return, Eof]) {
            break;
        }
    }
}

fn declaration(iter: &mut Peekable<IntoIter<Token>>) -> Result<Stmt, Box<Error>> {
    if next_is(iter, &[TokenType::Var]) {
        var_declaration(iter)
    } else {
        statement(iter)
    }
}

fn var_declaration(iter: &mut Peekable<IntoIter<Token>>) -> Result<Stmt, Box<Error>> {
    consume(iter, &[TokenType::Var])?;
    let name = consume(iter, &[TokenType::Identifier])?;
    let initializer = match maybe_consume(iter, &[TokenType::Equal]) {
        Some(_) => parse_expression(iter).map(|initializer| Some(initializer)),
        None => Ok(None),
    }?;
    consume(iter, &[TokenType::Semicolon])?;
    Ok(Stmt::var(name, initializer))
}

fn statement(iter: &mut Peekable<IntoIter<Token>>) -> Result<Stmt, Box<Error>> {
    if next_is(iter, &[TokenType::If]) {
        if_statement(iter)
    } else if next_is(iter, &[TokenType::Print]) {
        print_statement(iter)
    } else if next_is(iter, &[TokenType::LeftBrace]) {
        block_statement(iter).map(|stmts| Stmt::block(stmts))
    } else {
        expression_statement(iter)
    }
}

fn if_statement(iter: &mut Peekable<IntoIter<Token>>) -> Result<Stmt, Box<Error>> {
    iter.next();
    consume(iter, &[TokenType::LeftParen])?;
    let expr = parse_expression(iter)?;
    consume(iter, &[TokenType::RightParen])?;
    let then_branch = statement(iter)?;
    let else_branch = match maybe_consume(iter, &[TokenType::Else]) {
        Some(_) => statement(iter).map(|eb| Some(eb)),
        None => Ok(None),
    }?;
    Ok(Stmt::if_(expr, then_branch, else_branch))
}

fn print_statement(iter: &mut Peekable<IntoIter<Token>>) -> Result<Stmt, Box<Error>> {
    iter.next();
    let expr = parse_expression(iter)?;
    consume(iter, &[TokenType::Semicolon])?;
    Ok(Stmt::print(expr))
}

fn block_statement(iter: &mut Peekable<IntoIter<Token>>) -> Result<Vec<Stmt>, Box<Error>> {
    iter.next();
    let mut statements = Vec::new();
    while !next_is(iter, &[TokenType::RightBrace]) {
        match declaration(iter) {
            Ok(stmt) => statements.push(stmt),
            Err(error) => return Err(error),
        }
    }
    consume(iter, &[TokenType::RightBrace]).expect("BUG: expected RightBrace");
    Ok(statements)
}

fn expression_statement(iter: &mut Peekable<IntoIter<Token>>) -> Result<Stmt, Box<Error>> {
    let expr = parse_expression(iter)?;
    consume(iter, &[TokenType::Semicolon])?;
    Ok(Stmt::expression(expr))
}


fn parse_expression(iter: &mut Peekable<IntoIter<Token>>) -> Result<Expr, Box<Error>> {
    parse_assignment(iter)
}

fn parse_assignment(iter: &mut Peekable<IntoIter<Token>>) -> Result<Expr, Box<Error>> {
    parse_equality(iter).and_then(|expr| {
        match maybe_consume(iter, &[TokenType::Equal]) {
            Some(equal) => {
                parse_assignment(iter).and_then(|value| {
                    match expr {
                        Expr::Variable { ref name } => Ok(Expr::assign((*name).clone(), value)),
                        _ => Err(ParseError::new_arr(&[TokenType::Identifier], Some(equal))),
                    }
                })
            }
            None => Ok(expr),
        }
    })
}

fn parse_binary(iter: &mut Peekable<IntoIter<Token>>, matches: &[TokenType], parse_operand: fn(&mut Peekable<IntoIter<Token>>) -> Result<Expr, Box<Error>>) -> Result<Expr, Box<Error>> {
    let mut expr = parse_operand(iter)?;
    while let Some(operator) = maybe_consume(iter, matches) {
        expr = parse_operand(iter).map(|right| Expr::binary(expr, operator, right))?;
    }
    Ok(expr)
}

fn parse_equality(iter: &mut Peekable<IntoIter<Token>>) -> Result<Expr, Box<Error>> {
    parse_binary(iter, &[BangEqual, EqualEqual], parse_comparison)
}

fn parse_comparison(iter: &mut Peekable<IntoIter<Token>>) -> Result<Expr, Box<Error>> {
    parse_binary(iter, &[Greater, GreaterEqual, Less, LessEqual], parse_addition)
}

fn parse_addition(iter: &mut Peekable<IntoIter<Token>>) -> Result<Expr, Box<Error>> {
    parse_binary(iter, &[Minus, Plus], parse_multiplication)
}

fn parse_multiplication(iter: &mut Peekable<IntoIter<Token>>) -> Result<Expr, Box<Error>> {
    parse_binary(iter, &[Slash, Star], parse_unary)
}

fn parse_unary(iter: &mut Peekable<IntoIter<Token>>) -> Result<Expr, Box<Error>> {
    match maybe_consume(iter, &[Bang, Minus]) {
        Some(operator) => parse_unary(iter).map(|right| Expr::unary(operator, right)),
        None => parse_primary(iter),
    }
}

fn parse_primary(iter: &mut Peekable<IntoIter<Token>>) -> Result<Expr, Box<Error>> {
    iter.next().ok_or(ParseError::new(&*EXPECT_PRIMARY, None) as Box<Error>).and_then(|token| {
        if EXPECT_PRIMARY.contains(&token.token_type) {
            match token.token_type {
                LeftParen => parse_expression(iter).and_then(|expr| {
                    consume(iter, &[RightParen]).map(|_| Expr::grouping(expr))
                }),
                Identifier => Ok(Expr::variable(token)),
                _ => match token.literal {
                    Some(value) => Ok(Expr::literal(value)),
                    None => Err(ParseError::new(&*EXPECT_PRIMARY, Some(token))),
                },
            }
        } else {
            Err(ParseError::new(&*EXPECT_PRIMARY, Some(token)))
        }
    })
}


fn next_is(iter: &mut Peekable<IntoIter<Token>>, matches: &[TokenType]) -> bool {
    if let Some(next) = iter.peek() {
        for tt in matches.iter() {
            if *tt == next.token_type {
                return true
            }
        }
    }
    false
}

fn maybe_consume(iter: &mut Peekable<IntoIter<Token>>, matches: &[TokenType]) -> Option<Token> {
    if next_is(iter, matches) {
        iter.next()
    } else {
        None
    }
}

fn is_one_of(token: &Token, matches: &[TokenType]) -> bool {
    for tt in matches.iter() {
        if *tt == token.token_type {
            return true
        }
    }
    false
}

fn consume(iter: &mut Peekable<IntoIter<Token>>, matches: &[TokenType]) -> Result<Token, Box<Error>> {
    iter.next().map(|token| {
        if is_one_of(&token, matches) {
            Ok(token)
        } else {
            Err(ParseError::new_arr(matches, Some(token)) as Box<Error>)
        }
    }).unwrap_or(Err(ParseError::new_arr(matches, None) as Box<Error>))
}
