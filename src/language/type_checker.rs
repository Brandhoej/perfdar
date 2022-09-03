use super::{
    environment::Environment, error::Error, node::Node, node_type::NodeType, value::Value,
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

    pub fn check_node(&self, node: &Node) -> Result<NodeType, Error> {
        match node {
            Node::Literal(literal) => self.check_value(literal),
            Node::Assigment { .. } => Ok(NodeType::Void),
        }
    }

    pub fn check_value(&self, value: &Value) -> Result<NodeType, Error> {
        match value {
            Value::Bool(_) => Ok(NodeType::Logical),
            Value::Identifier(identifier) => {
                if let Some(value) = self.environment.get(identifier) {
                    return self.check_value(value);
                }
                Err(Error::TypeCheckingError {
                    message: String::from(format!("unknown identifier: {identifier}")),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::language::{
        environment::Environment, node::Node, node_type::NodeType, value::Value,
    };

    use super::TypeChecker;

    #[test]
    fn type_checker_default_is_empty_environment() {
        let type_checker = TypeChecker::default();

        assert_eq!(type_checker.environment, Environment::new_empty());
    }

    #[test]
    fn type_checker_empty_is_empty_environment() {
        let type_checker = TypeChecker::empty();

        assert_eq!(type_checker.environment, Environment::new_empty());
    }

    #[test]
    fn type_checker_check_node() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::new_false());
        let type_checker = TypeChecker::new(&environment);
        let inputs = vec![
            (Node::Literal(Value::new_false()), NodeType::Logical),
            (Node::Literal(Value::new_true()), NodeType::Logical),
            (Node::Literal(Value::new_identifier("a")), NodeType::Logical),
            (
                Node::new_assignment("a", &Value::new_false()),
                NodeType::Void,
            ),
            (
                Node::new_assignment("a", &Value::new_true()),
                NodeType::Void,
            ),
            (
                Node::new_assignment("a", &Value::new_identifier("a")),
                NodeType::Void,
            ),
        ];

        for (node, node_type) in inputs {
            let result = type_checker.check_node(&node);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), node_type);
        }
    }

    #[test]
    fn type_checker_check_value() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::new_false());
        let type_checker = TypeChecker::new(&environment);
        let inputs = vec![
            (Value::new_false(), NodeType::Logical),
            (Value::new_true(), NodeType::Logical),
            (Value::new_identifier("a"), NodeType::Logical),
        ];

        for (node, node_type) in inputs {
            let result = type_checker.check_value(&node);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), node_type);
        }
    }
}
