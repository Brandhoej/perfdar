use super::{
    environment::Environment,
    error::Error,
    expression::{BinaryOperator, Expression},
    lang_type::LangType,
    statement::Statement,
    value::Value,
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

    pub fn check_expression(&self, expression: &Expression) -> Result<LangType, Error> {
        match expression {
            Expression::Literal(literal) => self.check_value(literal),
            Expression::Parenthesized(expr) => self.check_expression(expr),
            Expression::Binary(lhs, op, rhs) => {
                let lhs_type = self.check_expression(&lhs).ok().unwrap();
                let rhs_type = self.check_expression(&rhs).ok().unwrap();

                match op {
                    BinaryOperator::LogicalAnd
                    | BinaryOperator::LogicalOr
                    | BinaryOperator::Implication
                    | BinaryOperator::BiImplication => {
                        if lhs_type == LangType::Logical && rhs_type == LangType::Logical {
                            Ok(LangType::Logical)
                        } else {
                            Err(Error::TypeCheckingError {
                                message: String::from(
                                    "Cannot perform logical Logical operator for two non logical types",
                                ),
                            })
                        }
                    }
                    BinaryOperator::Equal | BinaryOperator::NotEqual => {
                        if lhs_type == rhs_type {
                            Ok(LangType::Logical)
                        } else {
                            Err(Error::TypeCheckingError {
                                message: String::from(
                                    "Cannot perform equality on two different types",
                                ),
                            })
                        }
                    }
                }
            }
            Expression::Unary(op, expr) => {
                let expr_type = self.check_expression(&expr).ok().unwrap();
                match op {
                    super::expression::UnaryOperator::Negation => {
                        if expr_type != LangType::Logical {
                            Err(Error::TypeCheckingError {
                                message: String::from(
                                    "Cannot perform logical negation on non-logical type",
                                ),
                            })
                        } else {
                            Ok(LangType::Logical)
                        }
                    }
                }
            }
        }
    }

    pub fn check_statement(&self, statement: &Statement) -> Result<LangType, Error> {
        match statement {
            Statement::Assigment { .. } => Ok(LangType::Void),
        }
    }

    pub fn check_value(&self, value: &Value) -> Result<LangType, Error> {
        match value {
            Value::Bool(_) => Ok(LangType::Logical),
            Value::Identifier(identifier) => {
                if let Some(value) = self.environment.get_value(identifier) {
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
        environment::Environment, expression::Expression, lang_type::LangType,
        statement::Statement, value::Value,
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
    fn type_checker_check_expression() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::new_false());
        let type_checker = TypeChecker::new(&environment);
        let inputs = vec![
            // LITERAL
            (Expression::Literal(Value::new_false()), LangType::Logical),
            (Expression::Literal(Value::new_true()), LangType::Logical),
            (
                Expression::Literal(Value::new_identifier("a")),
                LangType::Logical,
            ),
            // AND
            (
                Expression::new_logical_and(
                    &Expression::new_boolean(&true),
                    &Expression::new_boolean(&true),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_and(
                    &Expression::new_boolean(&false),
                    &Expression::new_boolean(&true),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_and(
                    &Expression::new_boolean(&true),
                    &Expression::new_boolean(&false),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_and(
                    &Expression::new_boolean(&false),
                    &Expression::new_boolean(&false),
                ),
                LangType::Logical,
            ),
            // OR
            (
                Expression::new_logical_or(
                    &Expression::new_boolean(&true),
                    &Expression::new_boolean(&true),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_or(
                    &Expression::new_boolean(&false),
                    &Expression::new_boolean(&true),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_or(
                    &Expression::new_boolean(&true),
                    &Expression::new_boolean(&false),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_or(
                    &Expression::new_boolean(&false),
                    &Expression::new_boolean(&false),
                ),
                LangType::Logical,
            ),
            // IMPLICATION
            (
                Expression::new_logical_implication(
                    &Expression::new_boolean(&true),
                    &Expression::new_boolean(&true),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_implication(
                    &Expression::new_boolean(&false),
                    &Expression::new_boolean(&true),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_implication(
                    &Expression::new_boolean(&true),
                    &Expression::new_boolean(&false),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_implication(
                    &Expression::new_boolean(&false),
                    &Expression::new_boolean(&false),
                ),
                LangType::Logical,
            ),
            // BI-IMPLICATION
            (
                Expression::new_logical_bi_implication(
                    &Expression::new_boolean(&true),
                    &Expression::new_boolean(&true),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_bi_implication(
                    &Expression::new_boolean(&false),
                    &Expression::new_boolean(&true),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_bi_implication(
                    &Expression::new_boolean(&true),
                    &Expression::new_boolean(&false),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_bi_implication(
                    &Expression::new_boolean(&false),
                    &Expression::new_boolean(&false),
                ),
                LangType::Logical,
            ),
            // EQUAL
            (
                Expression::new_logical_equal(
                    &Expression::new_boolean(&true),
                    &Expression::new_boolean(&true),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_equal(
                    &Expression::new_boolean(&false),
                    &Expression::new_boolean(&true),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_equal(
                    &Expression::new_boolean(&true),
                    &Expression::new_boolean(&false),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_equal(
                    &Expression::new_boolean(&false),
                    &Expression::new_boolean(&false),
                ),
                LangType::Logical,
            ),
            // NOT-EQUAL
            (
                Expression::new_logical_not_equal(
                    &Expression::new_boolean(&true),
                    &Expression::new_boolean(&true),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_not_equal(
                    &Expression::new_boolean(&false),
                    &Expression::new_boolean(&true),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_not_equal(
                    &Expression::new_boolean(&true),
                    &Expression::new_boolean(&false),
                ),
                LangType::Logical,
            ),
            (
                Expression::new_logical_not_equal(
                    &Expression::new_boolean(&false),
                    &Expression::new_boolean(&false),
                ),
                LangType::Logical,
            ),
        ];

        for (node, node_type) in inputs {
            let result = type_checker.check_expression(&node);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), node_type);
        }
    }

    #[test]
    fn type_checker_check_statement() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::new_false());
        let type_checker = TypeChecker::new(&environment);
        let inputs = vec![
            (
                Statement::new_simple_assignment("a", &Value::new_false()),
                LangType::Void,
            ),
            (
                Statement::new_simple_assignment("a", &Value::new_true()),
                LangType::Void,
            ),
            (
                Statement::new_simple_assignment("a", &Value::new_identifier("a")),
                LangType::Void,
            ),
        ];

        for (node, node_type) in inputs {
            let result = type_checker.check_statement(&node);

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
            (Value::new_false(), LangType::Logical),
            (Value::new_true(), LangType::Logical),
            (Value::new_identifier("a"), LangType::Logical),
        ];

        for (node, node_type) in inputs {
            let result = type_checker.check_value(&node);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), node_type);
        }
    }
}
