use std::collections::VecDeque;

use super::value::Value;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BinaryOperator {
    LogicalAnd,
    LogicalOr,
    Equal,
    NotEqual,
    Implication,
    BiImplication,
}

impl ToString for BinaryOperator {
    fn to_string(&self) -> String {
        match self {
            BinaryOperator::LogicalAnd => String::from("&&"),
            BinaryOperator::LogicalOr => String::from("||"),
            BinaryOperator::Equal => String::from("=="),
            BinaryOperator::NotEqual => String::from("!="),
            BinaryOperator::Implication => String::from("-->"),
            BinaryOperator::BiImplication => String::from("<-->"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum UnaryOperator {
    Negation,
}

impl ToString for UnaryOperator {
    fn to_string(&self) -> String {
        match self {
            UnaryOperator::Negation => String::from("!"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Expression {
    Literal(Value),
    Parenthesized(Box<Expression>),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
    Unary(UnaryOperator, Box<Expression>),
}

impl Expression {
    pub fn new_literal(value: &Value) -> Expression {
        Expression::Literal(value.clone())
    }

    pub fn new_boolean(value: &bool) -> Expression {
        Expression::Literal(Value::Bool(*value))
    }

    pub fn new_binary_expression(
        lhs: &Expression,
        op: &BinaryOperator,
        rhs: &Expression,
    ) -> Expression {
        Expression::Binary(
            Box::new(lhs.to_owned()),
            op.to_owned(),
            Box::new(rhs.to_owned()),
        )
    }

    pub fn new_identifier(ident: &str) -> Expression {
        Self::new_literal(&Value::new_identifier(ident))
    }

    pub fn new_parenthesized(expression: &Expression) -> Expression {
        Self::Parenthesized(Box::new(expression.clone()))
    }

    pub fn new_logical_and(lhs: &Expression, rhs: &Expression) -> Expression {
        Expression::new_binary_expression(lhs, &BinaryOperator::LogicalAnd, rhs)
    }

    pub fn new_logical_or(lhs: &Expression, rhs: &Expression) -> Expression {
        Expression::new_binary_expression(lhs, &BinaryOperator::LogicalOr, rhs)
    }

    pub fn new_logical_implication(lhs: &Expression, rhs: &Expression) -> Expression {
        Expression::new_binary_expression(lhs, &BinaryOperator::Implication, rhs)
    }

    pub fn new_logical_bi_implication(lhs: &Expression, rhs: &Expression) -> Expression {
        Expression::new_binary_expression(lhs, &BinaryOperator::BiImplication, rhs)
    }

    pub fn new_logical_equal(lhs: &Expression, rhs: &Expression) -> Expression {
        Expression::new_binary_expression(lhs, &BinaryOperator::Equal, rhs)
    }

    pub fn new_logical_not_equal(lhs: &Expression, rhs: &Expression) -> Expression {
        Expression::new_binary_expression(lhs, &BinaryOperator::NotEqual, rhs)
    }

    pub fn identifiers(&self) -> Vec<String> {
        let mut identifiers: Vec<String> = Vec::new();

        let mut worklist: VecDeque<Expression> = VecDeque::new();
        worklist.push_back(self.clone());

        let mut visit_value = |value: Value| {
            if let Value::Identifier(ident) = value {
                identifiers.push(ident.clone());
            }
        };

        while worklist.len() > 0 {
            match worklist.pop_back().unwrap() {
                Expression::Literal(literal) => {
                    visit_value(literal);
                }
                Expression::Parenthesized(expr) => worklist.push_back(*expr),
                Expression::Binary(lhs, _, rhs) => {
                    worklist.push_back(*lhs);
                    worklist.push_back(*rhs);
                }
                Expression::Unary(_, expr) => worklist.push_back(*expr),
            }
        }

        identifiers
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Literal(value) => value.to_string(),
            Expression::Parenthesized(expr) => "(".to_owned() + &expr.to_string() + ")",
            Expression::Binary(lhs, op, rhs) => {
                lhs.to_string() + " " + &op.to_string() + " " + &rhs.to_string()
            }
            Expression::Unary(op, expr) => op.to_string() + &expr.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Expression, Value};

    #[test]
    fn node_new_literal_boolean_true() {
        let value = Value::new_true();
        let node = Expression::new_literal(&value);
        assert!(matches!(node, Expression::Literal(node_value) if node_value == value));
    }

    #[test]
    fn node_new_literal_boolean_false() {
        let value = Value::new_false();
        let node = Expression::new_literal(&value);
        assert!(matches!(node, Expression::Literal(node_value) if node_value == value));
    }

    #[test]
    fn node_new_literal_identifier() {
        let ident = "ident";
        let value = Value::new_identifier(ident);
        let node = Expression::new_literal(&value);
        assert!(matches!(node, Expression::Literal(node_value) if node_value == value));
    }

    #[test]
    fn node_new_literal_boolean_true_identifiers() {
        let value = Value::new_true();
        let node = Expression::new_literal(&value);
        let identifiers = node.identifiers();
        assert!(identifiers.is_empty());
    }

    #[test]
    fn node_new_literal_boolean_false_identifiers() {
        let value = Value::new_false();
        let node = Expression::new_literal(&value);
        let identifiers = node.identifiers();
        assert!(identifiers.is_empty());
    }

    #[test]
    fn node_new_literal_identifier_identifiers() {
        let ident = "ident";
        let value = Value::new_identifier(ident);
        let node = Expression::new_literal(&value);
        let identifiers = node.identifiers();
        assert_eq!(identifiers, vec![String::from(ident)])
    }
}
