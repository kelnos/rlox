use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use token::Token;
use value::Value;

pub struct Environment {
    values: HashMap<String, Rc<Value>>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment::new_enclosing(None)
    }

    pub fn new_enclosing(enclosing: Option<Rc<RefCell<Environment>>>) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: String, value: Rc<Value>) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: Rc<Value>) -> bool {
        if !self.values.contains_key(&name) {
            match self.enclosing {
                Some(ref mut enc) => enc.borrow_mut().assign(name, value),
                None => false,
            }
        } else {
            self.values.insert(name, value);
            true
        }
    }

    pub fn get(&self, name: &Token) -> Option<Rc<Value>> {
        match self.values.get(&name.lexeme) {
            Some(v) => Some(v.clone()),
            None => match self.enclosing {
                Some(ref enc) => enc.borrow().get(name),
                None => None,
            }
        }
    }
}
