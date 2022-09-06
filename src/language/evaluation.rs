use std::fmt::Display;

use super::value::Value;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Evaluation {
    Bool(bool),
    Void,
}

impl Evaluation {
    pub fn new_boolean(value: bool) -> Self {
        Self::Bool(value)
    }

    pub fn new_void() -> Self {
        Self::Void
    }

    pub fn new_true() -> Self {
        Evaluation::Bool(true)
    }

    pub fn new_false() -> Self {
        Evaluation::Bool(false)
    }

    pub fn is_false(&self) -> bool {
        match self {
            Evaluation::Bool(value) => return *value == false,
            _ => false,
        }
    }

    pub fn is_true(&self) -> bool {
        match self {
            Evaluation::Bool(value) => return *value == true,
            _ => false,
        }
    }
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Evaluation::Bool(value) => f.write_fmt(format_args!("{}", value)),
            Evaluation::Void => f.write_str("void"),
        }
    }
}

impl From<Value> for Evaluation {
    fn from(value: Value) -> Self {
        match value {
            Value::Bool(value) => return Evaluation::Bool(value),
            Value::Identifier(_) => panic!("Identifier cannot be converted to an evaluation"),
        }
    }
}

impl Into<Value> for Evaluation {
    fn into(self) -> Value {
        match self {
            Evaluation::Bool(boolean) => Value::new_boolean(boolean),
            Evaluation::Void => panic!("Void evaluation cannot be converted to a value"),
        }
    }
}

impl Into<bool> for Evaluation {
    fn into(self) -> bool {
        match self {
            Evaluation::Bool(value) => value,
            _ => panic!("Evaluation is not boolean"),
        }
    }
}

impl From<bool> for Evaluation {
    fn from(value: bool) -> Self {
        Evaluation::new_boolean(value)
    }
}
