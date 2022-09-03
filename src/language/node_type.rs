use std::fmt::Display;

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
