use std::fmt::Display;

use crate::language::{expression::Expression, value::Value};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Guard {
    pub node: Expression,
}

impl Guard {
    pub fn new(node: &Expression) -> Self {
        Self { node: node.clone() }
    }

    pub fn new_true() -> Self {
        Self::new(&Expression::Literal(Value::Bool(true)))
    }

    pub fn new_false() -> Self {
        Self::new(&Expression::Literal(Value::Bool(false)))
    }
}

impl Display for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.node.to_string()))
    }
}
