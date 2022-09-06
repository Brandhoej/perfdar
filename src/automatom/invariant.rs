use std::fmt::Display;

use crate::language::{expression::Expression, value::Value};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Invariant {
    pub node: Expression,
}

impl Invariant {
    pub fn new(node: &Expression) -> Invariant {
        Invariant { node: node.clone() }
    }

    pub fn new_true() -> Invariant {
        Self::new(&Expression::Literal(Value::Bool(true)))
    }

    pub fn new_false() -> Invariant {
        Self::new(&Expression::Literal(Value::Bool(false)))
    }
}

impl Display for Invariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.node.to_string())
    }
}
