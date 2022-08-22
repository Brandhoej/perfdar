use std::fmt::Debug;
use std::fmt::Display;

use crate::language::node::Node;
use crate::language::node::Value;

use super::channel::*;
use super::location::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Guard {
    pub node: Node,
}

impl Guard {
    pub fn new(node: &Node) -> Self {
        Self { node: node.clone() }
    }

    pub fn new_true() -> Self {
        Self::new(&Node::Literal(Value::Bool(true)))
    }

    pub fn new_false() -> Self {
        Self::new(&Node::Literal(Value::Bool(false)))
    }
}

impl Display for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.node.to_string()))
    }
}

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

    pub fn empty() -> Self {
        Self { node: None }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Edge {
    pub source: Location,
    pub action: Channel,
    pub guard: Guard,
    pub update: Update,
    pub target: Location,
}

impl Edge {
    pub fn new(
        source: &Location,
        action: &Channel,
        guard: &Guard,
        update: &Update,
        target: &Location,
    ) -> Self {
        Self {
            source: source.clone(),
            action: action.clone(),
            guard: guard.clone(),
            update: update.clone(),
            target: target.clone(),
        }
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} -({}, {}, {})-> {}",
            self.source, self.action, self.guard, self.update, self.target
        ))
    }
}
