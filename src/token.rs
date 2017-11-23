use std::fmt;
use value;
use value::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // single-char
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,

    // single-or-double
    Slash,
    Star,
    BangEqual,
    Bang,
    EqualEqual,
    Equal,
    GreaterEqual,
    Greater,
    LessEqual,
    Less,

    // keywords
    And,
    Class,
    Else,
    Fun,
    For,
    If,
    Or,
    Print,
    Return,
    Super,
    This,
    Var,
    While,
    Continue,
    Break,

    // const-literal keywords
    False,
    Nil,
    True,

    // var-length
    Identifier,
    Str,
    Number,
    Comment,

    Eof,
    Invalid,
}

impl TokenType {
    pub fn const_lexeme(&self) -> Option<&'static str> {
        use self::TokenType::*;
        match *self {
            LeftParen => Some("("),
            RightParen => Some(")"),
            LeftBrace => Some("{"),
            RightBrace => Some("}"),
            Comma => Some(","),
            Dot => Some("."),
            Minus => Some("-"),
            Plus => Some("+"),
            Semicolon => Some(";"),
            Slash => Some("/"),
            Star => Some("*"),
            BangEqual => Some("!="),
            Bang => Some("!"),
            EqualEqual => Some("=="),
            Equal => Some("="),
            GreaterEqual => Some(">="),
            Greater => Some(">"),
            LessEqual => Some("<="),
            Less => Some("<"),
            And => Some("and"),
            Class => Some("class"),
            Else => Some("else"),
            Fun => Some("fun"),
            For => Some("for"),
            If => Some("if"),
            Or => Some("or"),
            Print => Some("print"),
            Return => Some("return"),
            Super => Some("super"),
            This => Some("this"),
            Var => Some("var"),
            While => Some("while"),
            Continue => Some("continue"),
            Break => Some("break"),
            Eof => Some("EOF"),

            False => Some("false"),
            Nil => Some("nil"),
            True => Some("true"),

            _ => None
        }
    }

    pub fn const_literal(&self) -> Option<Value> {
        match *self {
            TokenType::False => Some(value::FalseValue),
            TokenType::Nil => Some(Value::Nil),
            TokenType::True => Some(value::TrueValue),
            _ => None
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self.const_lexeme() {
            Some(s) => s,
            None => match *self {
                TokenType::Identifier => "[identifier]",
                TokenType::Str => "[string]",
                TokenType::Number => "[number]",
                TokenType::Comment => "[comment]",
                TokenType::Eof => "[EOF]",
                TokenType::Invalid => "[invalid]",
                _ => "[unknown]",
            },
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Value>,
    pub line: u32,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let literal = self.literal.as_ref().map(|l| l.to_string()).unwrap_or(String::from("(none)"));
        write!(f, "<{}@{} ({}, {})>", self.token_type, self.line, self.lexeme, literal)
    }
}

impl Token {
    pub fn simple(token_type: TokenType, line: u32) -> Token {
        let lexeme = match token_type.const_lexeme() {
            Some(s) => s,
            None => panic!("Cannot use Token::simple() for token type {}", token_type),
        };
        let literal = token_type.const_literal();
        Token {
            token_type,
            lexeme: String::from(lexeme),
            literal,
            line,
        }
    }

    pub fn with_lexeme(token_type: TokenType, lexeme: String, line: u32) -> Token {
        match token_type {
            TokenType::Identifier => (),
            TokenType::Comment => (),
            TokenType::Invalid => (),
            _ => panic!("Cannot use Token::with_lexeme() for token type {}", token_type),
        }
        let literal = token_type.const_literal();
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn with_literal(token_type: TokenType, lexeme: String, literal: Value, line: u32) -> Token {
        match token_type {
            TokenType::Str => (),
            TokenType::Number => (),
            _ => panic!("Cannot use Token::with_literal() for token type {}", token_type),
        }
        Token {
            token_type,
            lexeme,
            literal: Some(literal),
            line,
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
