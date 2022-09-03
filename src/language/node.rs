use std::collections::VecDeque;

use super::value::Value;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Node {
    Literal(Value),
    Assigment { identifier: Value, value: Value },
}

impl Node {
    pub fn new_literal(value: &Value) -> Node {
        Node::Literal(value.clone())
    }

    pub fn new_boolean(value: &bool) -> Node {
        Node::Literal(Value::Bool(*value))
    }

    pub fn new_identifier(ident: &str) -> Node {
        Self::new_literal(&Value::new_identifier(ident))
    }

    pub fn new_assignment(identifier: &str, value: &Value) -> Node {
        Node::Assigment {
            identifier: Value::new_identifier(identifier),
            value: value.clone(),
        }
    }

    pub fn identifiers(&self) -> Vec<String> {
        let mut identifiers: Vec<String> = Vec::new();

        let mut worklist: VecDeque<Node> = VecDeque::new();
        worklist.push_back(self.clone());

        let mut visit_value = |value: Value| {
            if let Value::Identifier(ident) = value {
                identifiers.push(ident.clone());
            }
        };

        while worklist.len() > 0 {
            let current = worklist.pop_back().unwrap();
            match current {
                Node::Literal(literal) => {
                    visit_value(literal);
                }
                Node::Assigment { identifier, value } => {
                    visit_value(identifier);
                    visit_value(value);
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

#[cfg(test)]
mod tests {
    use super::{Node, Value};

    #[test]
    fn node_new_literal_boolean_true() {
        let value = Value::new_true();
        let node = Node::new_literal(&value);
        assert!(matches!(node, Node::Literal(node_value) if node_value == value));
    }

    #[test]
    fn node_new_literal_boolean_false() {
        let value = Value::new_false();
        let node = Node::new_literal(&value);
        assert!(matches!(node, Node::Literal(node_value) if node_value == value));
    }

    #[test]
    fn node_new_literal_identifier() {
        let ident = "ident";
        let value = Value::new_identifier(ident);
        let node = Node::new_literal(&value);
        assert!(matches!(node, Node::Literal(node_value) if node_value == value));
    }

    #[test]
    fn node_new_assignment_literal_boolean_true() {
        let ident = "ident";
        let identifier = Value::new_identifier(ident);
        let value = Value::new_true();
        let node = Node::new_assignment(ident, &value);
        assert!(
            matches!(node, Node::Assigment { identifier: node_identifier, value: node_value } 
                if node_identifier == identifier && node_value == value)
        );
    }

    #[test]
    fn node_new_assignment_literal_boolean_false() {
        let ident = "ident";
        let identifier = Value::new_identifier(ident);
        let value = Value::new_false();
        let node = Node::new_assignment(ident, &value);
        assert!(
            matches!(node, Node::Assigment { identifier: node_identifier, value: node_value } 
                if node_identifier == identifier && node_value == value)
        );
    }

    #[test]
    fn node_new_assignment_literal_identifier() {
        let ident = "ident";
        let identifier = Value::new_identifier(ident);
        let value = Value::new_identifier("other ident");
        let node = Node::new_assignment(ident, &value);
        assert!(
            matches!(node, Node::Assigment { identifier: node_identifier, value: node_value } 
                if node_identifier == identifier && node_value == value)
        );
    }

    #[test]
    fn node_new_literal_boolean_true_identifiers() {
        let value = Value::new_true();
        let node = Node::new_literal(&value);
        let identifiers = node.identifiers();
        assert!(identifiers.is_empty());
    }

    #[test]
    fn node_new_literal_boolean_false_identifiers() {
        let value = Value::new_false();
        let node = Node::new_literal(&value);
        let identifiers = node.identifiers();
        assert!(identifiers.is_empty());
    }

    #[test]
    fn node_new_literal_identifier_identifiers() {
        let ident = "ident";
        let value = Value::new_identifier(ident);
        let node = Node::new_literal(&value);
        let identifiers = node.identifiers();
        assert_eq!(identifiers, vec![String::from(ident)])
    }

    #[test]
    fn node_new_assignment_literal_boolean_true_identifiers() {
        let ident = "ident";
        let value = Value::new_true();
        let node = Node::new_assignment(ident, &value);
        let identifiers = node.identifiers();
        assert_eq!(identifiers, vec![String::from(ident)])
    }

    #[test]
    fn node_new_assignment_literal_boolean_false_identifiers() {
        let ident = "ident";
        let value = Value::new_false();
        let node = Node::new_assignment(ident, &value);
        let identifiers = node.identifiers();
        assert_eq!(identifiers, vec![String::from(ident)])
    }

    #[test]
    fn node_new_assignment_literal_identifier_identifiers() {
        let ident = "ident";
        let rhs_ident = "other ident";
        let value = Value::new_identifier(rhs_ident);
        let node = Node::new_assignment(ident, &value);
        let identifiers = node.identifiers();
        assert_eq!(
            identifiers,
            vec![String::from(ident), String::from(rhs_ident)]
        )
    }
}
