use super::value::Value;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Evaluation {
    Bool(bool),
    Void,
}

impl Evaluation {
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

impl From<Value> for Evaluation {
    fn from(value: Value) -> Self {
        match value {
            Value::Bool(value) => return Evaluation::Bool(value),
            Value::Identifier(_) => panic!("Identifier cannot be converted to an evaluation"),
        }
    }
}
