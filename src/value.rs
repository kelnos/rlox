use std::fmt;
use callable::LoxCallable;

#[derive(PartialEq, Debug)]
pub enum Value {
    Nil,
    Str(String),
    Number(f64),
    Boolean(bool),
    Identifier(String),
    Comment(String),
    Callable(LoxCallable),
}

#[allow(non_upper_case_globals)]
pub const TrueValue: Value = Value::Boolean(true);
#[allow(non_upper_case_globals)]
pub const FalseValue: Value = Value::Boolean(false);

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Nil => f.write_str("nil"),
            Value::Str(ref s) => f.write_str(s),
            Value::Number(n) => f.write_str(&n.to_string()),
            Value::Boolean(b) => f.write_str(&b.to_string()),
            Value::Identifier(ref s) => f.write_str(s),
            Value::Comment(ref s) => f.write_str(s),
            Value::Callable(ref c) => f.write_str(c.name()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    fn write_value(value: &Value) -> String {
        let mut s = String::new();
        write!(&mut s, "{}", value).unwrap();
        s
    }

    #[test]
    fn nil_prints() {
        let s = write_value(&Value::Nil);
        assert_eq!(
            "nil",
            &s
        )
    }

    #[test]
    fn identifier_prints() {
        let s = write_value(&Value::Identifier(String::from("foo")));
        assert_eq!(
            "foo",
            &s
        )
    }
}
