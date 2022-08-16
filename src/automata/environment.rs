use std::collections::HashMap;

use super::node::{Node, NodeType};

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Environment {
    map: HashMap<String, Node>,
}

impl Environment {
    pub fn contains_identifiers_in(&self, node: Node) -> bool {
        match node {
            Node::Boolean(_) => todo!(),
            Node::Identifier(identifier) => {
                if !self.contains_identifier(&identifier) {
                    return false;
                }
            }
            Node::Assigment { identifier, value } => {
                return self.contains_identifiers_in(*identifier)
                    && self.contains_identifiers_in(*value);
            }
        }
        true
    }

    pub fn lookup(&self, identifier: &str) -> Option<Node> {
        if let Some(node) = self.map.get(identifier) {
            return Some(node.clone());
        }
        None
    }

    pub fn set(&mut self, identifier: &str, value: &Node) -> bool {
        if !self.contains_identifier(identifier) {
            return false;
        }
        let entry = self.map.entry(String::from(identifier));
        entry.or_insert(value.clone());
        true
    }

    pub fn is_disjoint(&self, other: &Environment) -> bool {
        for key in self.map.keys() {
            if other.contains_identifier(key) {
                return false;
            }
        }

        return true;
    }

    pub fn contains_identifier(&self, identifier: &str) -> bool {
        self.map.contains_key(identifier)
    }

    pub fn add_all(&mut self, other: &Environment) -> bool {
        if !self.is_disjoint(other) {
            return false;
        }

        self.map.extend(other.map.clone());
        true
    }
}

impl Environment {
    pub fn node_type(&self, node: &Node) -> NodeType {
        return match node {
            Node::Boolean(_) => NodeType::Logical,
            Node::Identifier(identifier) => match self.map.get(identifier) {
                Some(identifier_node) => self.node_type(identifier_node),
                None => panic!("identifier was not present in the environment"),
            },
            Node::Assigment {
                identifier: _,
                value: _,
            } => NodeType::Void,
        };
    }
}
