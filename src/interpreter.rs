use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use environment::Environment;
use expression::Expr;
use statement::Stmt;
use token::{TokenType, Token};
use value::Value;

struct State {
    environment: Rc<RefCell<Environment>>,
}

impl State {
    fn new(environment: Rc<RefCell<Environment>>) -> State {
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
    pub fn new(location: &Token, message: String) -> Box<RuntimeError> {
        let description = format!("ERR:{}:{}", location.line, message);
        Box::new(RuntimeError {
            location: location.clone(),
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

pub fn interpret(environment: Rc<RefCell<Environment>>, statements: Vec<Stmt>) -> Result<(), Box<Error>> {
    let mut state = State::new(Rc::clone(&environment));
    let mut iter = statements.into_iter();
    loop {
        match iter.next() {
            Some(ref stmt) => execute_stmt(&mut state, stmt)?,
            None => break,
        }
    }
    Ok(())
}

fn execute_stmt(state: &mut State, stmt: &Stmt) -> Result<(), Box<Error>> {
    match stmt {
        &Stmt::Block { ref statements } => execute_block(state, statements),
        &Stmt::Expression { ref expression } => execute_expression_stmt(state, expression),
        &Stmt::For { ref initializer, ref condition, ref increment, ref body } => execute_for_stmt(state, initializer, condition, increment, body),
        &Stmt::If { ref expression, ref then_branch, ref else_branch } => execute_if_stmt(state, expression, then_branch, else_branch),
        &Stmt::Print { ref expression } => execute_print_stmt(state, expression),
        &Stmt::Var { ref name, ref initializer } => execute_var_stmt(state, name, initializer),
    }
}

fn execute_block(state: &mut State, statements: &Vec<Stmt>) -> Result<(), Box<Error>> {
    let block_environment = Environment::new_enclosing(Some(Rc::clone(&state.environment)));
    let mut block_state = State::new(Rc::new(RefCell::new(block_environment)));
    for statement in statements.iter() {
        match execute_stmt(&mut block_state, statement) {
            Ok(_) => (),
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

fn execute_expression_stmt(state: &mut State, expr: &Expr) -> Result<(), Box<Error>> {
    evaluate_expression(state, expr).map(|_| ())
}

fn execute_for_stmt(state: &mut State, initializer: &Option<Box<Stmt>>, condition: &Expr, increment: &Option<Box<Stmt>>, body: &Stmt) -> Result<(), Box<Error>> {
    match initializer {
        &Some(ref i) => execute_stmt(state, i),
        &None => Ok(()),
    }?;
    loop {
        let cond_value = evaluate_expression(state, &condition)?;
        if is_truthy(cond_value) {
            execute_stmt(state, &body)?;
            match increment {
                &Some(ref i) => execute_stmt(state, i),
                &None => Ok(()),
            }?;
        } else {
            break;
        }
    }
    Ok(())
}

fn execute_if_stmt(state: &mut State, expr: &Expr, then_branch: &Box<Stmt>, else_branch: &Option<Box<Stmt>>) -> Result<(), Box<Error>> {
    match evaluate_expression(state, expr) {
        Ok(value) => 
            if is_truthy(value) {
                execute_stmt(state, then_branch)
            } else {
                match else_branch {
                    &Some(ref eb) => execute_stmt(state, eb),
                    &None => Ok(()),
                }
            },
        Err(error) => Err(error), 
    }
}

fn execute_print_stmt(state: &mut State, expr: &Expr) -> Result<(), Box<Error>> {
    evaluate_expression(state, expr).map(|ref value| {
        println!("{}", value.to_string());
        ()
    })
}

fn execute_var_stmt(state: &mut State, name: &Token, initializer: &Option<Expr>) -> Result<(), Box<Error>> {
    match initializer {
        &Some(ref init) => evaluate_expression(state, init),
        &None => Ok(Rc::new(Value::Nil)),
    }.map(|init_value| {
        state.environment.borrow_mut().define(name.lexeme.clone(), init_value);
        ()
    })
}

fn evaluate_expression(state: &mut State, expr: &Expr) -> Result<Rc<Value>, Box<Error>> {
    match expr {
        &Expr::Assign { ref name, ref value } => evaluate_assign(state, name, &**value),
        &Expr::Binary { ref left, ref operator, ref right } => evaluate_binary(state, &**left, operator, &**right),
        &Expr::Grouping { ref expression } => evaluate_grouping(state, &**expression),
        &Expr::Literal { ref value } => evaluate_literal(state, Rc::clone(value)),
        &Expr::Logical { ref left, ref operator, ref right } => evaluate_logical(state, &**left, operator, &**right),
        &Expr::Unary { ref operator, ref right } => evaluate_unary(state, operator, &**right),
        &Expr::Variable { ref name } => match state.environment.borrow().get(name) {
            Some(ref value) => Ok(Rc::clone(value)),
            None => {
                let message = format!("Undefined variable {}", name.lexeme);
                Err(RuntimeError::new(name, message))
            }
        },
    }
}

fn evaluate_assign(state: &mut State, name: &Token, value: &Expr) -> Result<Rc<Value>, Box<Error>> {
    evaluate_expression(state, value).and_then(|ref expr_value| {
        if !state.environment.borrow_mut().assign(name.lexeme.clone(), Rc::clone(expr_value)) {
            let message = format!("Undefined variable {}", name.lexeme);
            Err(RuntimeError::new(name, message))
        } else {
            Ok(Rc::clone(expr_value))
        }
    })
}

fn evaluate_binary(state: &mut State, left: &Expr, operator: &Token, right: &Expr) -> Result<Rc<Value>, Box<Error>> {
    let left_value = evaluate_expression(state, left)?;
    let right_value = evaluate_expression(state, right)?;
    match operator.token_type {
        TokenType::Minus | TokenType::Plus | TokenType::Slash | TokenType::Star => arithmetic(&left_value, operator, &right_value).map(|v| Rc::new(v)),
        TokenType::Less | TokenType::LessEqual | TokenType::Greater | TokenType::GreaterEqual => compare(&left_value, operator, &right_value).map(|v| Rc::new(v)),
        TokenType::EqualEqual => Ok(Rc::new(Value::Boolean(is_equal(left_value, right_value)))),
        TokenType::BangEqual => Ok(Rc::new(Value::Boolean(!is_equal(left_value, right_value)))),
        _ => {
            let description = format!("Operator '{}' is not valid for a binary expression", operator.token_type);
            Err(RuntimeError::new(operator, description))
        },
    }
}

fn arithmetic(left: &Value, operator: &Token, right: &Value) -> Result<Value, Box<Error>> {
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

fn compare(left: &Value, operator: &Token, right: &Value) -> Result<Value, Box<Error>> {
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

fn evaluate_grouping(state: &mut State, expression: &Expr) -> Result<Rc<Value>, Box<Error>> {
    evaluate_expression(state, expression)
}

fn evaluate_literal(_state: &mut State, value: Rc<Value>) -> Result<Rc<Value>, Box<Error>> {
    Ok(value)
}

fn evaluate_logical(state: &mut State, left: &Expr, operator: &Token, right: &Expr) -> Result<Rc<Value>, Box<Error>> {
    match evaluate_expression(state, left) {
        Ok(ref left_value) => {
            let is_left_truthy = is_truthy(Rc::clone(left_value));
            match operator.token_type {
                TokenType::Or if is_left_truthy => Ok(Rc::clone(left_value)),
                TokenType::And if !is_left_truthy => Ok(Rc::clone(left_value)),
                _ => evaluate_expression(state, right),
            }
        },
        Err(error) => Err(error),
    }
}

fn evaluate_unary(state: &mut State, operator: &Token, right: &Expr) -> Result<Rc<Value>, Box<Error>> {
    let right_value = evaluate_expression(state, right)?;
    match operator.token_type {
        TokenType::Minus => match *right_value {
            Value::Number(n) => Ok(Rc::new(Value::Number(-n))),
            _ => Err(RuntimeError::new(operator, format!("Operator '-' cannot be applied to non-number value {}", right_value))),
        },
        TokenType::Bang => Ok(Rc::new(Value::Boolean(!is_truthy(right_value)))),
        _ => {
            let description = format!("Operator '{}' is not valid in a unary expression", operator.token_type);
            Err(RuntimeError::new(operator, description))
        },
    }
}

fn is_truthy(value: Rc<Value>) -> bool {
    match *value {
        Value::Nil => false,
        Value::Boolean(b) => b,
        _ => true,
    }
}

fn is_equal(left: Rc<Value>, right: Rc<Value>) -> bool {
    match *left {
        Value::Nil => match *right {
            Value::Nil => true,
            _ => false,
        },
        _ => *left == *right,
    }
}
