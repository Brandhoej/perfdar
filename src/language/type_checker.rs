use super::{
    environment::Environment,
    node::{Node, NodeType, Value},
};

#[derive(Default, Clone, Eq, PartialEq)]
pub struct TypeChecker {
    environment: Environment,
}

impl TypeChecker {
    pub fn new(environment: &Environment) -> Self {
        Self {
            environment: environment.clone(),
        }
    }

    pub fn empty() -> Self {
        TypeChecker::default()
    }

    pub fn check_node(&self, node: &Node) -> NodeType {
        match node {
            Node::Literal(literal) => self.check_value(literal),
            Node::Assigment {
                identifier: _,
                value,
            } => {
                return match value {
                    Value::Bool(_) => NodeType::Logical,
                    Value::Identifier(_) => panic!("value cannot be an identifier"),
                };
            }
        }
    }

    pub fn check_value(&self, value: &Value) -> NodeType {
        match value {
            Value::Bool(_) => NodeType::Logical,
            Value::Identifier(identifier) => {
                if let Some(value) = self.environment.get(identifier) {
                    return self.check_value(value);
                }
                panic!("Unknown identifier");
            }
        }
    }
}
