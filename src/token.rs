use std::fmt;
use value;
use value::Value;

pub trait Lexeme {
    fn lexeme(&self) -> &str;
}

pub trait Literal {
    fn literal(&self) -> Option<&Value>;
}

#[derive(Debug)]
pub enum Token {
    // single-char
    LeftParen(u32),
    RightParen(u32),
    LeftBrace(u32),
    RightBrace(u32),
    Comma(u32),
    Dot(u32),
    Minus(u32),
    Plus(u32),
    Semicolon(u32),

    // single-or-double
    Slash(u32),
    Star(u32),
    BangEqual(u32),
    Bang(u32),
    EqualEqual(u32),
    Equal(u32),
    GreaterEqual(u32),
    Greater(u32),
    LessEqual(u32),
    Less(u32),

    // keywords
    And(u32),
    Class(u32),
    Else(u32),
    Fun(u32),
    For(u32),
    If(u32),
    Or(u32),
    Print(u32),
    Return(u32),
    Super(u32),
    This(u32),
    Var(u32),
    While(u32),
    Continue(u32),
    Break(u32),

    // const-literal keywords
    False(u32),
    Nil(u32),
    True(u32),

    // var-length
    Identifier(String, u32),
    String(String, Value, u32),
    Number(String, Value, u32),
    Comment(String, u32),

    Eof(u32),
    Invalid(String, u32),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Token {
    pub fn lexeme(&self) -> &str {
        use self::Token::*;
        match *self {
            LeftParen(_) => "(",
            RightParen(_) => ")",
            LeftBrace(_) => "{",
            RightBrace(_) => "}",
            Comma(_) => ",",
            Dot(_) => ".",
            Minus(_) => "-",
            Plus(_) => "+",
            Semicolon(_) => ";",
            Slash(_) => "/",
            Star(_) => "*",
            BangEqual(_) => "!=",
            Bang(_) => "!",
            EqualEqual(_) => "==",
            Equal(_) => "=",
            GreaterEqual(_) => ">=",
            Greater(_) => ">",
            LessEqual(_) => "<=",
            Less(_) => "<",
            And(_) => "and",
            Class(_) => "class",
            Else(_) => "else",
            Fun(_) => "fun",
            For(_) => "for",
            If(_) => "if",
            Or(_) => "or",
            Print(_) => "print",
            Return(_) => "return",
            Super(_) => "super",
            This(_) => "this",
            Var(_) => "var",
            While(_) => "while",
            Continue(_) => "continue",
            Break(_) => "break",
            Eof(_) => "EOF",

            False(_) => "false",
            Nil(_) => "nil",
            True(_) => "true",

            Identifier(ref lexeme, _) => lexeme,
            String(ref lexeme, _, _) => lexeme,
            Number(ref lexeme, _, _) => lexeme,
            Comment(ref lexeme, _) => lexeme,
            Invalid(ref lexeme, _) => lexeme,
        }
    }

    pub fn literal(&self) -> Option<&Value> {
        use self::Token::*;
        match *self {
            LeftParen(_) => None,
            RightParen(_) => None,
            LeftBrace(_) => None,
            RightBrace(_) => None,
            Comma(_) => None,
            Dot(_) => None,
            Minus(_) => None,
            Plus(_) => None,
            Semicolon(_) => None,
            Slash(_) => None,
            Star(_) => None,
            BangEqual(_) => None,
            Bang(_) => None,
            EqualEqual(_) => None,
            Equal(_) => None,
            GreaterEqual(_) => None,
            Greater(_) => None,
            LessEqual(_) => None,
            Less(_) => None,
            And(_) => None,
            Class(_) => None,
            Else(_) => None,
            Fun(_) => None,
            For(_) => None,
            If(_) => None,
            Or(_) => None,
            Print(_) => None,
            Return(_) => None,
            Super(_) => None,
            This(_) => None,
            Var(_) => None,
            While(_) => None,
            Continue(_) => None,
            Break(_) => None,
            Eof(_) => None,

            False(_) => Some(&value::FalseValue),
            Nil(_) => Some(&Value::Nil),
            True(_) => Some(&value::TrueValue),

            Identifier(_, _) => None,
            String(_, ref literal, _) => Some(literal),
            Number(_, ref literal, _) => Some(literal),
            Comment(_, _) => None,
            Invalid(_, _) => None,
        }
    }

    pub fn line(&self) -> u32 {
        use self::Token::*;
        match *self {
            LeftParen(line) => line,
            RightParen(line) => line,
            LeftBrace(line) => line,
            RightBrace(line) => line,
            Comma(line) => line,
            Dot(line) => line,
            Minus(line) => line,
            Plus(line) => line,
            Semicolon(line) => line,
            Slash(line) => line,
            Star(line) => line,
            BangEqual(line) => line,
            Bang(line) => line,
            EqualEqual(line) => line,
            Equal(line) => line,
            GreaterEqual(line) => line,
            Greater(line) => line,
            LessEqual(line) => line,
            Less(line) => line,
            And(line) => line,
            Class(line) => line,
            Else(line) => line,
            Fun(line) => line,
            For(line) => line,
            If(line) => line,
            Or(line) => line,
            Print(line) => line,
            Return(line) => line,
            Super(line) => line,
            This(line) => line,
            Var(line) => line,
            While(line) => line,
            Continue(line) => line,
            Break(line) => line,
            Eof(line) => line,

            False(line) => line,
            Nil(line) => line,
            True(line) => line,

            Identifier(_, line) => line,
            String(_, _, line) => line,
            Number(_, _, line) => line,
            Comment(_, line) => line,
            Invalid(_, line) => line,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_lexeme_token_prints() {
        let t = ConstLexemeTokenType::And.token(42);
        println!("{}", t)
    }

    #[test]
    fn const_literal_token_prints() {
        let t = False.token(42);
        println!("{}", t)
    }

    #[test]
    fn variable_token_prints() {
        let literal = Value::Str(String::from("hello world"));
        let t = VariableTokenType::String.token(
            String::from("\"hello world\""),
            &literal,
            42
        );
        println!("{}", t)
    }
}
