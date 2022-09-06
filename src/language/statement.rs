use super::{expression::Expression, value::Value};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Statement {
    Assigment {
        identifier: Expression,
        value: Expression,
    },
}

impl Statement {
    pub fn new_assignment(identifier: &Expression, value: &Expression) -> Statement {
        Statement::Assigment {
            identifier: identifier.clone(),
            value: value.clone(),
        }
    }

    pub fn new_simple_assignment(identifier: &str, value: &Value) -> Statement {
        Statement::Assigment {
            identifier: Expression::new_identifier(identifier),
            value: Expression::Literal(value.clone()),
        }
    }

    pub fn identifiers(&self) -> Vec<String> {
        let mut identifiers: Vec<String> = Vec::new();

        match self {
            Statement::Assigment { identifier, value } => {
                identifiers.extend(identifier.identifiers());
                identifiers.extend(value.identifiers());
            }
        }

        identifiers
    }
}

impl ToString for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::Assigment { identifier, value } => {
                identifier.to_string() + " = " + &value.to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::language::expression::Expression;

    use super::Statement;

    #[test]
    fn node_new_assignment_literal_boolean_true() {
        let ident = "ident";
        let identifier = Expression::new_identifier(ident);
        let value = Expression::new_boolean(&true);
        let node = Statement::new_assignment(&identifier, &value);
        assert!(
            matches!(node, Statement::Assigment { identifier: node_identifier, value: node_value } 
                if node_identifier == identifier && node_value == value)
        );
    }

    #[test]
    fn node_new_assignment_literal_boolean_false() {
        let ident = "ident";
        let identifier = Expression::new_identifier(ident);
        let value = Expression::new_boolean(&false);
        let node = Statement::new_assignment(&identifier, &value);
        assert!(
            matches!(node, Statement::Assigment { identifier: node_identifier, value: node_value } 
                if node_identifier == identifier && node_value == value)
        );
    }

    #[test]
    fn node_new_assignment_literal_identifier() {
        let ident = "ident";
        let identifier = Expression::new_identifier(ident);
        let value = Expression::new_identifier("other ident");
        let node = Statement::new_assignment(&identifier, &value);
        assert!(
            matches!(node, Statement::Assigment { identifier: node_identifier, value: node_value } 
                if node_identifier == identifier && node_value == value)
        );
    }

    #[test]
    fn node_new_assignment_literal_boolean_true_identifiers() {
        let ident = "ident";
        let identifier = Expression::new_identifier(ident);
        let value = Expression::new_boolean(&true);
        let node = Statement::new_assignment(&identifier, &value);
        let identifiers = node.identifiers();
        assert_eq!(identifiers, vec![String::from(ident)])
    }

    #[test]
    fn node_new_assignment_literal_boolean_false_identifiers() {
        let ident = "ident";
        let identifier = Expression::new_identifier(ident);
        let value = Expression::new_boolean(&false);
        let node = Statement::new_assignment(&identifier, &value);
        let identifiers = node.identifiers();
        assert_eq!(identifiers, vec![String::from(ident)])
    }

    #[test]
    fn node_new_assignment_literal_identifier_identifiers() {
        let ident = "ident";
        let rhs_ident = "other ident";
        let identifier = Expression::new_identifier(ident);
        let value = Expression::new_identifier(rhs_ident);
        let node = Statement::new_assignment(&identifier, &value);
        let identifiers = node.identifiers();
        assert_eq!(
            identifiers,
            vec![String::from(ident), String::from(rhs_ident)]
        )
    }
}
