use std::collections::HashMap;
use std::error::Error;
use std::iter::Peekable;
use std::str::Chars;
use token::{TokenType, Token};
use value::Value;

// unfortunately we can't store closure in a HashMap that's defined as
// lazy_static!, so we have to create a bunch of one-line functions and
// store pointers to those instead.
macro_rules! token_fn {
    ($name:ident, $token_type:ident) => (
        fn $name(line: u32) -> Token {
            Token::simple(TokenType::$token_type, line)
        }
    )
}

token_fn!(create_and, And);
token_fn!(create_break, Break);
token_fn!(create_class, Class);
token_fn!(create_continue, Continue);
token_fn!(create_else, Else);
token_fn!(create_false, False);
token_fn!(create_for, For);
token_fn!(create_fun, Fun);
token_fn!(create_if, If);
token_fn!(create_nil, Nil);
token_fn!(create_or, Or);
token_fn!(create_print, Print);
token_fn!(create_return, Return);
token_fn!(create_super, Super);
token_fn!(create_this, This);
token_fn!(create_true, True);
token_fn!(create_var, Var);
token_fn!(create_while, While);

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, fn(u32) -> Token> = {
        let mut m = HashMap::new();
        m.insert("and", create_and as fn(u32) -> Token);
        m.insert("break", create_break as fn(u32) -> Token);
        m.insert("class", create_class as fn(u32) -> Token);
        m.insert("continue", create_continue as fn(u32) -> Token);
        m.insert("else", create_else as fn(u32) -> Token);
        m.insert("false", create_false as fn(u32) -> Token);
        m.insert("for", create_for as fn(u32) -> Token);
        m.insert("fun", create_fun as fn(u32) -> Token);
        m.insert("if", create_if as fn(u32) -> Token);
        m.insert("nil", create_nil as fn(u32) -> Token);
        m.insert("or", create_or as fn(u32) -> Token);
        m.insert("print", create_print as fn(u32) -> Token);
        m.insert("return", create_return as fn(u32) -> Token);
        m.insert("super", create_super as fn(u32) -> Token);
        m.insert("this", create_this as fn(u32) -> Token);
        m.insert("true", create_true as fn(u32) -> Token);
        m.insert("var", create_var as fn(u32) -> Token);
        m.insert("while", create_while as fn(u32) -> Token);
        m
    };
}

fn consume_next_if<'a>(iter: &mut Peekable<Chars>, line: u32, next_is: char, success: TokenType, failure: TokenType) -> Token {
    if iter.peek() == Some(&next_is) {
        iter.next();
        Token::simple(success, line)
    } else {
        Token::simple(failure, line)
    }
}

fn consume_slash_or_comment(iter: &mut Peekable<Chars>, line: u32) -> (Token, u32) {
    match iter.peek() {
        Some(&'/') => {
            let mut new_line = line;
            let mut comment = String::from("/");
            while let Some(c) = iter.next() {
                if c == '\n' {
                    new_line += 1;
                    break;
                }
                comment.push(c);
            }
            (Token::with_lexeme(TokenType::Comment, comment, line), new_line)
        },
        Some(&'*') => {
            iter.next();
            let mut comment = String::from("/*");
            let new_line = consume_block_comment(&mut comment, iter, line);
            (Token::with_lexeme(TokenType::Comment, comment, line), new_line)
        },
        _ => (Token::simple(TokenType::Slash, line), line)
    }
}

fn consume_block_comment(comment: &mut String, iter: &mut Peekable<Chars>, line: u32) -> u32 {
    let mut new_line = line;
    while let Some(c) = iter.next() {
        match c {
            '/' => if let Some(&'*') = iter.peek() {
                iter.next();
                comment.push_str("/*");
                let new_new_line = consume_block_comment(comment, iter, new_line);
                new_line = new_new_line;
            },
            '*' => {
                comment.push('*');
                if let Some(&'/') = iter.peek() {
                    comment.push('/');
                    break;
                }
            },
            _ => {
                comment.push(c);
                if c == '\n' {
                    new_line += 1;
                }
            },
        }
    }
    new_line
}

fn consume_string(iter: &mut Peekable<Chars>, line: u32) -> (Option<Token>, u32) {
    let mut new_line = line;
    let mut s = String::from("\"");
    while let Some(c) = iter.next() {
        s.push(c);
        if c == '\n' {
            new_line += 1;
        }
        if c == '"' && (!s.ends_with("\\") || s.ends_with("\\\\")) {
            break;
        }
    }
    if iter.peek() != None {
        let literal = Value::Str(s[1..s.len()-1].to_string());
        (Some(Token::with_literal(TokenType::Str, s, literal, line)), new_line)
    } else {
        (None, new_line)
    }
}

fn consume_number(iter: &mut Peekable<Chars>, first_char: char, line: u32) -> Token {
    let mut n = first_char.to_string();
    while let Some(_) = iter.peek().and_then(|c| {
        if c.is_numeric() || *c == '.' {
            n.push(*c);
            Some(c)
        } else {
            None
        }
    }) {
        iter.next();
    }
    let literal = Value::Number(n.parse().unwrap());
    Token::with_literal(TokenType::Number, n, literal, line)
}

fn consume_identifier_or_keyword(iter: &mut Peekable<Chars>, first_char: char, line: u32) -> Token {
    let mut s = first_char.to_string();
    while let Some(_) = iter.peek().and_then(|c| {
        if c.is_alphanumeric() {
            s.push(*c);
            Some(c)
        } else {
            None
        }
    }) {
        iter.next();
    }
    match KEYWORDS.get(s.as_str()) {
        Some(f) => f(line),
        None => Token::with_lexeme(TokenType::Identifier, s, line),
    }
}

pub fn scan(source: &String) -> Result<Vec<Token>, Box<Error>> {
    let mut tokens = vec![];
    let mut line = 1;
    let mut iter = source.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            '(' => tokens.push(Token::simple(TokenType::LeftParen, line)),
            ')' => tokens.push(Token::simple(TokenType::RightParen, line)),
            '{' => tokens.push(Token::simple(TokenType::LeftBrace, line)),
            '}' => tokens.push(Token::simple(TokenType::RightBrace, line)),
            ',' => tokens.push(Token::simple(TokenType::Comma, line)),
            '.' => tokens.push(Token::simple(TokenType::Dot, line)),
            '-' => tokens.push(Token::simple(TokenType::Minus, line)),
            '+' => tokens.push(Token::simple(TokenType::Plus, line)),
            ';' => tokens.push(Token::simple(TokenType::Semicolon, line)),
            '/' => {
                let (token, new_line) = consume_slash_or_comment(&mut iter, line);
                line = new_line;
                tokens.push(token)
            },
            '*' => tokens.push(Token::simple(TokenType::Star, line)),
            '!' => tokens.push(consume_next_if(&mut iter, line, '=', TokenType::BangEqual, TokenType::Bang)),
            '=' => tokens.push(consume_next_if(&mut iter, line, '=', TokenType::EqualEqual, TokenType::Equal)),
            '>' => tokens.push(consume_next_if(&mut iter, line, '=', TokenType::GreaterEqual, TokenType::Greater)),
            '<' => tokens.push(consume_next_if(&mut iter, line, '=', TokenType::LessEqual, TokenType::Less)),
            '"' => {
                let (token, new_line) = consume_string(&mut iter, line);
                line = new_line;
                match token {
                    Some(t) => tokens.push(t),
                    _ => (),
                }
            },
            c if c.is_numeric() => tokens.push(consume_number(&mut iter, c, line)),
            c if c.is_alphabetic() => tokens.push(consume_identifier_or_keyword(&mut iter, c, line)),
            '\n' => line += 1,
            c if c.is_whitespace() => (),
            _ => (),  // not sure what we should do here... just skip it?  print a warning?
        }
    }
    tokens.push(Token::simple(TokenType::Eof, line));
    Ok(tokens)
}
