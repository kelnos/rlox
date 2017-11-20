use callable::Callable;
use value::Value;

#[derive(PartialEq, Debug)]
pub struct LoxFunction {
    name: String,
    arity: u32,
}

impl Callable for LoxFunction {
    fn name(&self) -> &String {
        &self.name
    }

    fn call(arguments: &Vec<&Value>) -> Value {
        unimplemented!()
    }
}