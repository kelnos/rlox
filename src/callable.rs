use value::Value;
use function::LoxFunction;

#[derive(PartialEq, Debug)]
pub enum LoxCallable {
    Function(LoxFunction),
}

impl LoxCallable {
    pub fn name(&self) -> &String {
        match *self {
            LoxCallable::Function(ref f) => f.name()
        }
    }
}

pub trait Callable {
    fn name(&self) -> &String;
    fn call(arguments: &Vec<&Value>) -> Value;
}

impl Callable for LoxCallable {
    fn name(&self) -> &String {
        unimplemented!()
    }
    fn call(arguments: &Vec<&Value>) -> Value {
        unimplemented!()
    }
}