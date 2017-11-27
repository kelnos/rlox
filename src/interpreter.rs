use std::error::Error;
use std::fmt;

use environment::Environment;
use expression::Expr;
use statement::Stmt;
use token::{TokenType, Token};
use value::Value;

struct State<'a> {
    environment: &'a mut Environment,
}

impl<'a> State<'a> {
    fn new(environment: &'a mut Environment) -> State {
        State {
            environment,
        }
    }
}

#[derive(Debug)]
struct RuntimeError {
    location: Token,
    description: String,
}

impl RuntimeError {
    pub fn new(location: Token, message: String) -> Box<RuntimeError> {
        let description = format!("ERR:{}:{}", location.line, message);
        Box::new(RuntimeError {
            location,
            description,
        })
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for RuntimeError {
    fn description(&self) -> &str {
        &self.description
    }
}

pub fn interpret(environment: &mut Environment, statements: Vec<Stmt>) -> Result<(), Box<Error>> {
    let mut state = State::new(environment);
    let mut iter = statements.into_iter();
    loop {
        match iter.next() {
            Some(stmt) => execute_stmt(&mut state, stmt)?,
            None => break,
        }
    }
    Ok(())
}

fn execute_stmt(state: &mut State, stmt: Stmt) -> Result<(), Box<Error>> {
    match stmt {
        Stmt::Print { expression } => execute_print_stmt(state, *expression),
        Stmt::Expression { expression } => execute_expression_stmt(state, *expression),
        Stmt::Var { name, initializer } => execute_var_stmt(state, name, initializer),
    }
}

fn execute_print_stmt(state: &mut State, expr: Expr) -> Result<(), Box<Error>> {
    evaluate_expression(state, expr).map(|value| {
        println!("{}", value.to_string());
        ()
    })
}

fn execute_expression_stmt(state: &mut State, expr: Expr) -> Result<(), Box<Error>> {
    evaluate_expression(state, expr).map(|_| ())
}

fn execute_var_stmt(state: &mut State, name: Token, initializer: Option<Expr>) -> Result<(), Box<Error>> {
    match initializer {
        Some(init) => evaluate_expression(state, init),
        None => Ok(Value::Nil),
    }.map(|init_value| {
        state.environment.define(name.lexeme.clone(), init_value);
        ()
    })
}

fn evaluate_expression(state: &mut State, expr: Expr) -> Result<Value, Box<Error>> {
    match expr {
        Expr::Binary { left, operator, right } => evaluate_binary(state, *left, operator, *right),
        Expr::Grouping { expression } => evaluate_grouping(state, *expression),
        Expr::Literal { value } => evaluate_literal(state, value),
        Expr::Unary { operator, right } => evaluate_unary(state, operator, *right),
        Expr::Variable { name } => match state.environment.get(&name) {
            Some(value) => Ok((*value).clone()),
            None => {
                let message = format!("Undefined variable {}", name.lexeme);
                Err(RuntimeError::new(name, message))
            }
        },
    }
}

fn evaluate_binary(state: &mut State, left: Expr, operator: Token, right: Expr) -> Result<Value, Box<Error>> {
    let left_value = evaluate_expression(state, left)?;
    let right_value = evaluate_expression(state, right)?;
    match operator.token_type {
        TokenType::Minus | TokenType::Plus | TokenType::Slash | TokenType::Star => arithmetic(&left_value, operator, &right_value),
        TokenType::Less | TokenType::LessEqual | TokenType::Greater | TokenType::GreaterEqual => compare(&left_value, operator, &right_value),
        TokenType::EqualEqual => Ok(Value::Boolean(is_equal(&left_value, &right_value))),
        TokenType::BangEqual => Ok(Value::Boolean(!is_equal(&left_value, &right_value))),
        _ => {
            let description = format!("Operator '{}' is not valid for a binary expression", operator.token_type);
            Err(RuntimeError::new(operator, description))
        },
    }
}

fn arithmetic(left: &Value, operator: Token, right: &Value) -> Result<Value, Box<Error>> {
    match (left, right) {
        (&Value::Number(l), &Value::Number(r)) => match operator.token_type {
            TokenType::Minus => Ok(Value::Number(l - r)),
            TokenType::Plus => Ok(Value::Number(l + r)),
            TokenType::Slash if r == 0.0 => Err(RuntimeError::new(operator, String::from("Can't divide by zero"))),
            TokenType::Slash => Ok(Value::Number(l / r)),
            TokenType::Star => Ok(Value::Number(l * r)),
            _ => {
                let description = format!("Operator '{}' is not valid for arithmetic", operator.token_type);
                Err(RuntimeError::new(operator, description))
            },
        },
        (&Value::Str(ref l), _) => match operator.token_type {
            TokenType::Plus => Ok(Value::Str(format!("{}{}", l, right.to_string()))),
            _ => {
                let description = format!("Operator '{}' is not valid for string concatenation", operator.token_type);
                Err(RuntimeError::new(operator, description))
            },
        },
        (_, &Value::Str(ref r)) => match operator.token_type {
            TokenType::Plus => Ok(Value::Str(format!("{}{}", left.to_string(), r))),
            _ => {
                let description = format!("Operator '{}' is not valid for string concatenation", operator.token_type);
                Err(RuntimeError::new(operator, description))
            },
        },
        _ => Err(RuntimeError::new(operator, String::from("Cannot perform arithmetic on non-numeric values"))),
    }
}

fn compare(left: &Value, operator: Token, right: &Value) -> Result<Value, Box<Error>> {
    match (left, right) {
        (&Value::Number(l), &Value::Number(r)) => match operator.token_type {
            TokenType::Less => Ok(Value::Boolean(l < r)),
            TokenType::LessEqual => Ok(Value::Boolean(l <= r)),
            TokenType::Greater => Ok(Value::Boolean(l > r)),
            TokenType::GreaterEqual => Ok(Value::Boolean(l >= r)),
            _ => {
                let description = format!("Operator '{}' is not valid for comparison", operator.token_type);
                Err(RuntimeError::new(operator, description))
            },
        },
        _ => Err(RuntimeError::new(operator, String::from("Cannot perform comparison on non-numeric values"))),
    }
}

fn evaluate_grouping(state: &mut State, expression: Expr) -> Result<Value, Box<Error>> {
    evaluate_expression(state, expression)
}

fn evaluate_literal(_state: &mut State, value: Value) -> Result<Value, Box<Error>> {
    Ok(value)
}

fn evaluate_unary(state: &mut State, operator: Token, right: Expr) -> Result<Value, Box<Error>> {
    let right_value = evaluate_expression(state, right)?;
    match operator.token_type {
        TokenType::Minus => match right_value {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(RuntimeError::new(operator, format!("Operator '-' cannot be applied to non-number value {}", right_value))),
        },
        TokenType::Bang => Ok(Value::Boolean(!is_truthy(&right_value))),
        _ => {
            let description = format!("Operator '{}' is not valid in a unary expression", operator.token_type);
            Err(RuntimeError::new(operator, description))
        },
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        &Value::Nil => false,
        &Value::Boolean(b) => b,
        _ => true,
    }
}

fn is_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (&Value::Nil, &Value::Nil) => true,
        (&Value::Nil, _) => false,
        (_, _) => left == right,
    }
}
