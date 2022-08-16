use std::collections::VecDeque;

pub enum NodeType {
    Logical,
    Void,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Node {
    Boolean(bool),
    Identifier(String),
    Assigment {
        identifier: Box<Node>,
        value: Box<Node>,
    },
}

impl Node {
    pub fn identifiers(&self) -> Vec<String> {
        let mut identifiers: Vec<String> = Vec::new();

        let mut worklist: VecDeque<Node> = VecDeque::new();
        worklist.push_back(self.clone());

        while worklist.len() > 0 {
            let current = worklist.pop_back().unwrap();

            match current {
                Node::Boolean(_) => todo!(),
                Node::Identifier(identifier) => {
                    identifiers.push(identifier.clone());
                }
                Node::Assigment { identifier, value } => {
                    worklist.push_back(*identifier);
                    worklist.push_back(*value);
                }
            }
        }

        identifiers
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        return match self {
            Node::Boolean(value) => value.to_string(),
            Node::Identifier(identifier) => identifier.clone(),
            Node::Assigment { identifier, value } => {
                identifier.to_string() + " = " + &value.to_string()
            }
        };
    }
}
