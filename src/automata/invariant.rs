use std::fmt::Display;

use super::node::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Invariant {
    pub node: Node,
}

impl Display for Invariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.node.to_string())
    }
}
