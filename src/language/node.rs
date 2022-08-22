use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum NodeType {
    Logical,
    Void,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Logical => f.write_str("logical"),
            NodeType::Void => f.write_str("void"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Value {
    Bool(bool),
    Identifier(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(value) => return f.write_str(&value.to_string()),
            Value::Identifier(identifier) => return f.write_str(identifier),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Node {
    Literal(Value),
    Assigment { identifier: Value, value: Value },
}

impl Node {
    pub fn identifiers(&self) -> Vec<String> {
        let mut identifiers: Vec<String> = Vec::new();

        let mut worklist: VecDeque<Node> = VecDeque::new();
        worklist.push_back(self.clone());

        while worklist.len() > 0 {
            let current = worklist.pop_back().unwrap();
            if let Node::Literal(literal) = current {
                if let Value::Identifier(ident) = literal {
                    identifiers.push(ident.clone())
                }
            }
        }

        identifiers
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        return match self {
            Node::Literal(value) => value.to_string(),
            Node::Assigment { identifier, value } => {
                identifier.to_string() + " = " + &value.to_string()
            }
        };
    }
}
