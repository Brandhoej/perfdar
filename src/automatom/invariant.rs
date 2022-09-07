use std::{collections::HashSet, fmt::Display};

use crate::language::{
    expression::{BinaryOperator, Expression},
    value::Value,
};

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

    pub fn new_conjunction(invariants: HashSet<Invariant>) -> Invariant {
        if invariants.len() < 2 {
            panic!("Cannto conjoin less than two invariants")
        }

        let mut invariant_iter = invariants.iter().map(|invariant| invariant.node.clone());
        let mut lhs = invariant_iter.next().unwrap();
        for rhs in invariant_iter {
            lhs = Expression::new_binary_expression(&lhs, &BinaryOperator::LogicalAnd, &rhs);
        }

        Invariant::new(&lhs)
    }
}

impl Display for Invariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.node.to_string())
    }
}
