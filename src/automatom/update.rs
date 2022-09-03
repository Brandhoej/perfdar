use std::fmt::Display;

use crate::language::node::Node;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Update {
    pub node: Option<Node>,
}

impl Default for Update {
    fn default() -> Self {
        Self::empty()
    }
}

impl Display for Update {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(node) = &self.node {
            return f.write_fmt(format_args!("{}", node.to_string()));
        }
        return f.write_str("void");
    }
}

impl Update {
    pub fn new(node: &Node) -> Self {
        Self {
            node: Some(node.clone()),
        }
    }

    pub fn new_void() -> Self {
        Self::empty()
    }

    pub fn empty() -> Self {
        Self { node: None }
    }
}
