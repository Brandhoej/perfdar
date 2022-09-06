use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use super::{expression::Expression, statement::Statement, value::Value};

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Environment {
    map: HashMap<String, Value>,
}

impl Environment {
    pub fn new_empty() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn contains(&self, identifier: &str) -> bool {
        self.map.contains_key(identifier)
    }

    pub fn contains_with_value(&self, identifier: &str, value: &Value) -> bool {
        if let Some(identified_value) = self.get_value(identifier) {
            identified_value == value
        } else {
            false
        }
    }

    pub fn get_value(&self, identifier: &str) -> Option<&Value> {
        self.map.get(identifier)
    }

    pub fn insert(&mut self, identifier: &str, value: &Value) -> bool {
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

    pub fn contains_identifiers_in_expression(&self, expression: &Expression) -> bool {
        match expression {
            Expression::Literal(literal) => {
                return self.contains_identifiers_in_value(literal);
            }
            Expression::Parenthesized(expr) => self.contains_identifiers_in_expression(expr),
            Expression::Binary(lhs, _, rhs) => {
                self.contains_identifiers_in_expression(lhs)
                    && self.contains_identifiers_in_expression(rhs)
            }
            Expression::Unary(_, expr) => self.contains_identifiers_in_expression(expr),
        }
    }

    pub fn contains_identifiers_in_statement(&self, statement: &Statement) -> bool {
        match statement {
            Statement::Assigment { identifier, value } => {
                return self.contains_identifiers_in_expression(identifier)
                    && self.contains_identifiers_in_expression(value);
            }
        }
    }

    pub fn contains_identifiers_in_value(&self, value: &Value) -> bool {
        if let Value::Identifier(ident) = value {
            return self.contains(ident);
        }
        true
    }

    pub fn missing_identifiers_in_expression(&self, expression: &Expression) -> Vec<String> {
        let mut missing = Vec::new();
        let mut worklist: VecDeque<&Expression> = VecDeque::new();
        worklist.push_back(expression);

        let mut contains_value_or_add = |value: &Value| {
            if let Value::Identifier(identifier) = value {
                if !self.contains(identifier) {
                    missing.push(identifier.clone());
                }
            }
        };

        while !worklist.is_empty() {
            match worklist.pop_front().unwrap() {
                Expression::Literal(literal) => contains_value_or_add(literal),
                Expression::Parenthesized(expr) => worklist.push_back(expr),
                Expression::Binary(lhs, _, rhs) => {
                    worklist.push_back(lhs);
                    worklist.push_back(rhs);
                }
                Expression::Unary(_, expr) => worklist.push_back(expr),
            }
        }

        missing
    }

    pub fn missing_identifiers_in_statement(&self, statement: &Statement) -> Vec<String> {
        let mut missing = Vec::new();
        let mut worklist: VecDeque<&Statement> = VecDeque::new();
        worklist.push_back(statement);

        while !worklist.is_empty() {
            match worklist.pop_front().unwrap() {
                Statement::Assigment { identifier, value } => {
                    missing.extend(self.missing_identifiers_in_expression(identifier));
                    missing.extend(self.missing_identifiers_in_expression(value));
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

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displays: Vec<&str> = self
            .map
            .iter()
            .map(|(key, value)| format_args!("{} := {}", key, value).as_str().unwrap())
            .collect();
        f.write_fmt(format_args!("{:#?}", displays))
    }
}

#[cfg(test)]
mod tests {
    use crate::language::{expression::Expression, statement::Statement, value::Value};

    use super::Environment;

    #[test]
    fn environment_empty_has_no_identifiers() {
        let environment = Environment::new_empty();

        let count = environment.count();

        assert_eq!(count, 0);
    }

    #[test]
    fn environment_empty_has_with_two_identifiers() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(false));
        environment.insert("b", &Value::Bool(false));

        let count = environment.count();

        assert_eq!(count, 2);
    }

    #[test]
    fn environment_contains_identifier_has_identifier() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));

        let contains = environment.contains("a");

        assert!(contains);
    }

    #[test]
    fn environment_contains_identifier_does_not_have_identifier() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));

        let contains = environment.contains("b");

        assert!(!contains);
    }

    #[test]
    fn environment_contains_identifiers_in_value_has_identifier() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));
        let identifier = Value::Identifier(String::from("a"));

        let contains = environment.contains_identifiers_in_value(&identifier);

        assert!(contains);
    }

    #[test]
    fn environment_contains_identifiers_in_value_does_not_have_identifier() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));
        let identifier = Value::Identifier(String::from("b"));

        let contains = environment.contains_identifiers_in_value(&identifier);

        assert!(!contains);
    }

    #[test]
    fn environment_contains_identifiers_in_node_literal_has_identifier() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));
        let identifier = Expression::Literal(Value::Identifier(String::from("a")));

        let contains = environment.contains_identifiers_in_expression(&identifier);

        assert!(contains);
    }

    #[test]
    fn environment_contains_identifiers_in_node_literal_does_not_have_identifier() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));
        let identifier = Expression::Literal(Value::Identifier(String::from("b")));

        let contains = environment.contains_identifiers_in_expression(&identifier);

        assert!(!contains);
    }

    #[test]
    fn environment_contains_identifiers_in_node_assignment_has_identifier() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));
        let assignment = Statement::new_simple_assignment("a", &Value::new_false());

        let contains = environment.contains_identifiers_in_statement(&assignment);

        assert!(contains);
    }

    #[test]
    fn environment_contains_identifiers_in_node_assignment_does_not_have_identifier() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));
        let assignment = Statement::new_simple_assignment("b", &Value::new_false());

        let contains = environment.contains_identifiers_in_statement(&assignment);

        assert!(!contains);
    }

    #[test]
    fn environment_get_identifier_does_not_have_identifier() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));

        let value = environment.get_value("b");

        assert_eq!(value, None);
    }

    #[test]
    fn environment_set_identifier_has_identifier() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));

        let was_set = environment.set("a", &Value::Bool(false));

        assert!(was_set);
    }

    #[test]
    fn environment_set_identifier_has_identifier_changes_value() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));
        environment.set("a", &Value::Bool(false));

        let value = environment.get_value("a").unwrap();

        assert_eq!(*value, Value::Bool(false));
    }

    #[test]
    fn environment_set_identifier_does_not_have_identifier() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));

        let was_set = environment.set("b", &Value::Bool(false));

        assert!(!was_set);
    }

    #[test]
    fn environment_concat_adds_all_identifiers_in_other() {
        let mut left = Environment::new_empty();
        left.insert("a", &Value::Bool(false));
        let mut right = Environment::new_empty();
        right.insert("b", &Value::Bool(true));

        let concat = left.concat(&right);
        let count = left.count();
        let a = left.get_value("a").unwrap();
        let b = left.get_value("b").unwrap();

        assert!(concat);
        assert_eq!(count, 2);
        assert_eq!(a, &Value::Bool(false));
        assert_eq!(b, &Value::Bool(true));
    }

    #[test]
    fn environment_concat_is_not_disjoint_fails() {
        let mut left = Environment::new_empty();
        left.insert("a", &Value::Bool(false));
        let mut right = Environment::new_empty();
        right.insert("a", &Value::Bool(true));

        let concat = left.concat(&right);
        let count = left.count();
        let a = left.get_value("a").unwrap();

        assert!(!concat);
        assert_eq!(count, 1);
        assert_eq!(a, &Value::Bool(false));
    }

    #[test]
    fn environment_is_disjoint_true() {
        let mut left = Environment::new_empty();
        left.insert("a", &Value::Bool(false));
        let mut right = Environment::new_empty();
        right.insert("b", &Value::Bool(true));

        let disjoint = left.is_disjoint(&right);

        assert!(disjoint);
    }

    #[test]
    fn environment_is_disjoint_false() {
        let mut left = Environment::new_empty();
        left.insert("a", &Value::Bool(false));
        let mut right = Environment::new_empty();
        right.insert("a", &Value::Bool(true));

        let disjoint = left.is_disjoint(&right);

        assert!(!disjoint);
    }

    #[test]
    fn environment_missing_identifiers_in_node() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(false));
        let assignment = Statement::new_simple_assignment("a", &Value::new_identifier("b"));

        let missing = environment.missing_identifiers_in_statement(&assignment);

        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0], "b");
    }
}
