use std::collections::{HashMap, VecDeque};

use super::node::{Node, Value};

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Environment {
    map: HashMap<String, Value>,
}

impl Environment {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn contains(&self, identifier: &str) -> bool {
        self.map.contains_key(identifier)
    }

    pub fn get(&self, identifier: &str) -> Option<&Value> {
        self.map.get(identifier)
    }

    pub fn add(&mut self, identifier: &str, value: &Value) -> bool {
        if self.contains(identifier) {
            return false;
        }
        self.map.insert(String::from(identifier), value.clone());
        return true;
    }

    pub fn concat(&mut self, other: &Environment) -> bool {
        if !self.is_disjoint(other) {
            return false;
        }

        self.map.extend(other.map.clone());
        true
    }

    pub fn set(&mut self, identifier: &str, value: &Value) -> bool {
        if !self.contains(identifier) {
            return false;
        }
        self.map.insert(String::from(identifier), value.clone());
        true
    }

    pub fn count(&self) -> usize {
        self.map.keys().count()
    }

    pub fn contains_identifiers_in_node(&self, node: &Node) -> bool {
        match node {
            Node::Literal(literal) => {
                return self.contains_identifiers_in_value(literal);
            }
            Node::Assigment { identifier, value } => {
                return self.contains_identifiers_in_value(identifier)
                    && self.contains_identifiers_in_value(value);
            }
        }
    }

    pub fn contains_identifiers_in_value(&self, value: &Value) -> bool {
        if let Value::Identifier(ident) = value {
            return self.contains(ident);
        }
        true
    }

    pub fn missing_identifiers_in_node(&self, node: &Node) -> Vec<String> {
        let mut missing = Vec::new();
        let mut worklist: VecDeque<&Node> = VecDeque::new();
        worklist.push_back(node);

        let mut contains_value_or_add = |value: &Value| {
            if let Value::Identifier(identifier) = value {
                if !self.contains(identifier) {
                    missing.push(identifier.clone());
                }
            }
        };

        while !worklist.is_empty() {
            let current = worklist.pop_back().unwrap();

            match current {
                Node::Literal(literal) => contains_value_or_add(literal),
                Node::Assigment { identifier, value } => {
                    contains_value_or_add(identifier);
                    contains_value_or_add(value);
                }
            }
        }

        missing
    }

    pub fn is_disjoint(&self, other: &Environment) -> bool {
        for key in self.map.keys() {
            if other.contains(key) {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::language::node::{Node, Value};

    use super::Environment;

    #[test]
    fn environment_empty_has_no_identifiers() {
        let environment = Environment::empty();

        let count = environment.count();

        assert_eq!(count, 0);
    }

    #[test]
    fn environment_empty_has_with_two_identifiers() {
        let mut environment = Environment::empty();
        environment.add("a", &Value::Bool(false));
        environment.add("b", &Value::Bool(false));

        let count = environment.count();

        assert_eq!(count, 2);
    }

    #[test]
    fn environment_contains_identifier_has_identifier() {
        let mut environment = Environment::empty();
        environment.add("a", &Value::Bool(true));

        let contains = environment.contains("a");

        assert!(contains);
    }

    #[test]
    fn environment_contains_identifier_does_not_have_identifier() {
        let mut environment = Environment::empty();
        environment.add("a", &Value::Bool(true));

        let contains = environment.contains("b");

        assert!(!contains);
    }

    #[test]
    fn environment_contains_identifiers_in_value_has_identifier() {
        let mut environment = Environment::empty();
        environment.add("a", &Value::Bool(true));
        let identifier = Value::Identifier(String::from("a"));

        let contains = environment.contains_identifiers_in_value(&identifier);

        assert!(contains);
    }

    #[test]
    fn environment_contains_identifiers_in_value_does_not_have_identifier() {
        let mut environment = Environment::empty();
        environment.add("a", &Value::Bool(true));
        let identifier = Value::Identifier(String::from("b"));

        let contains = environment.contains_identifiers_in_value(&identifier);

        assert!(!contains);
    }

    #[test]
    fn environment_contains_identifiers_in_node_literal_has_identifier() {
        let mut environment = Environment::empty();
        environment.add("a", &Value::Bool(true));
        let identifier = Node::Literal(Value::Identifier(String::from("a")));

        let contains = environment.contains_identifiers_in_node(&identifier);

        assert!(contains);
    }

    #[test]
    fn environment_contains_identifiers_in_node_literal_does_not_have_identifier() {
        let mut environment = Environment::empty();
        environment.add("a", &Value::Bool(true));
        let identifier = Node::Literal(Value::Identifier(String::from("b")));

        let contains = environment.contains_identifiers_in_node(&identifier);

        assert!(!contains);
    }

    #[test]
    fn environment_contains_identifiers_in_node_assignment_has_identifier() {
        let mut environment = Environment::empty();
        environment.add("a", &Value::Bool(true));
        let assignment = Node::Assigment {
            identifier: Value::Identifier(String::from("a")),
            value: Value::Bool(false),
        };

        let contains = environment.contains_identifiers_in_node(&assignment);

        assert!(contains);
    }

    #[test]
    fn environment_contains_identifiers_in_node_assignment_does_not_have_identifier() {
        let mut environment = Environment::empty();
        environment.add("a", &Value::Bool(true));
        let assignment = Node::Assigment {
            identifier: Value::Identifier(String::from("b")),
            value: Value::Bool(false),
        };

        let contains = environment.contains_identifiers_in_node(&assignment);

        assert!(!contains);
    }

    #[test]
    fn environment_get_identifier_does_not_have_identifier() {
        let mut environment = Environment::empty();
        environment.add("a", &Value::Bool(true));

        let value = environment.get("b");

        assert_eq!(value, None);
    }

    #[test]
    fn environment_set_identifier_has_identifier() {
        let mut environment = Environment::empty();
        environment.add("a", &Value::Bool(true));

        let was_set = environment.set("a", &Value::Bool(false));

        assert!(was_set);
    }

    #[test]
    fn environment_set_identifier_has_identifier_changes_value() {
        let mut environment = Environment::empty();
        environment.add("a", &Value::Bool(true));
        environment.set("a", &Value::Bool(false));

        let value = environment.get("a").unwrap();

        assert_eq!(*value, Value::Bool(false));
    }

    #[test]
    fn environment_set_identifier_does_not_have_identifier() {
        let mut environment = Environment::empty();
        environment.add("a", &Value::Bool(true));

        let was_set = environment.set("b", &Value::Bool(false));

        assert!(!was_set);
    }

    #[test]
    fn environment_concat_adds_all_identifiers_in_other() {
        let mut left = Environment::empty();
        left.add("a", &Value::Bool(false));
        let mut right = Environment::empty();
        right.add("b", &Value::Bool(true));

        let concat = left.concat(&right);
        let count = left.count();
        let a = left.get("a").unwrap();
        let b = left.get("b").unwrap();

        assert!(concat);
        assert_eq!(count, 2);
        assert_eq!(a, &Value::Bool(false));
        assert_eq!(b, &Value::Bool(true));
    }

    #[test]
    fn environment_concat_is_not_disjoint_fails() {
        let mut left = Environment::empty();
        left.add("a", &Value::Bool(false));
        let mut right = Environment::empty();
        right.add("a", &Value::Bool(true));

        let concat = left.concat(&right);
        let count = left.count();
        let a = left.get("a").unwrap();

        assert!(!concat);
        assert_eq!(count, 1);
        assert_eq!(a, &Value::Bool(false));
    }

    #[test]
    fn environment_is_disjoint_true() {
        let mut left = Environment::empty();
        left.add("a", &Value::Bool(false));
        let mut right = Environment::empty();
        right.add("b", &Value::Bool(true));

        let disjoint = left.is_disjoint(&right);

        assert!(disjoint);
    }

    #[test]
    fn environment_is_disjoint_false() {
        let mut left = Environment::empty();
        left.add("a", &Value::Bool(false));
        let mut right = Environment::empty();
        right.add("a", &Value::Bool(true));

        let disjoint = left.is_disjoint(&right);

        assert!(!disjoint);
    }

    #[test]
    fn environment_missing_identifiers_in_node() {
        let mut environment = Environment::empty();
        environment.add("a", &Value::Bool(false));
        let assignment = Node::Assigment {
            identifier: Value::Identifier(String::from("a")),
            value: Value::Identifier(String::from("b")),
        };

        let missing = environment.missing_identifiers_in_node(&assignment);

        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0], "b");
    }
}
