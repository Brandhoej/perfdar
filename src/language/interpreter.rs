use std::collections::VecDeque;

use super::{
    environment::*, error::Error, evaluation::Evaluation, expression::*, statement::Statement,
    value::Value,
};

#[derive(Default, Clone, Eq, PartialEq)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new(environment: &Environment) -> Self {
        Self {
            environment: environment.clone(),
        }
    }

    pub fn empty() -> Self {
        Interpreter::default()
    }

    pub fn eval_expression(&mut self, expression: &Expression) -> Result<Evaluation, Error> {
        let mut stack: VecDeque<Value> = VecDeque::new();
        let mut worklist: VecDeque<&Expression> = VecDeque::new();
        worklist.push_back(expression);

        while !worklist.is_empty() {
            match worklist.pop_front().unwrap() {
                Expression::Literal(literal) => {
                    if let Value::Identifier(ident) = literal {
                        if let Some(value) = self.environment.get_value(ident) {
                            stack.push_back(value.clone());
                        } else {
                            return Err(Error::RuntimeError {
                                message: String::from("Unknown identifier"),
                            });
                        }
                    } else {
                        stack.push_back(literal.clone())
                    }
                }
                Expression::Parenthesized(expr) => worklist.push_back(expr),
                Expression::Binary(lhs, op, rhs) => {
                    let lhs_evaluation = self.eval_expression(&lhs).ok().unwrap();
                    let rhs_evaluation = self.eval_expression(&rhs).ok().unwrap();
                    let evaluation: Value = match op {
                        BinaryOperator::LogicalAnd => {
                            let lhs_bool: bool = lhs_evaluation.into();
                            let rhs_bool: bool = rhs_evaluation.into();
                            Value::Bool(lhs_bool && rhs_bool)
                        }
                        BinaryOperator::LogicalOr => {
                            let lhs_bool: bool = lhs_evaluation.into();
                            let rhs_bool: bool = rhs_evaluation.into();
                            Value::Bool(lhs_bool || rhs_bool)
                        }
                        BinaryOperator::Equal | BinaryOperator::BiImplication => {
                            let lhs_bool: bool = lhs_evaluation.into();
                            let rhs_bool: bool = rhs_evaluation.into();
                            Value::Bool(lhs_bool == rhs_bool)
                        }
                        BinaryOperator::NotEqual => {
                            let lhs_bool: bool = lhs_evaluation.into();
                            let rhs_bool: bool = rhs_evaluation.into();
                            Value::Bool(lhs_bool != rhs_bool)
                        }
                        BinaryOperator::Implication => {
                            let lhs_bool: bool = lhs_evaluation.into();
                            let rhs_bool: bool = rhs_evaluation.into();
                            Value::Bool(!lhs_bool || rhs_bool)
                        }
                    };
                    stack.push_back(evaluation);
                }
                Expression::Unary(op, expr) => {
                    let expr_evaluation = self.eval_expression(&expr).ok().unwrap();
                    let evaluation: Value = match op {
                        UnaryOperator::Negation => {
                            let expr_bool: bool = expr_evaluation.into();
                            Value::Bool(expr_bool)
                        }
                    };
                    stack.push_back(evaluation);
                }
            }
        }

        if stack.len() > 1 {
            return Err(Error::RuntimeError {
                message: String::from("More than one element on the stack"),
            });
        }

        match stack.pop_front() {
            Some(value) => match value {
                Value::Bool(value) => Ok(Evaluation::Bool(value)),
                Value::Identifier(_) => Err(Error::RuntimeError {
                    message: String::from("An identifier cannot be an evaluation"),
                }),
            },
            None => Ok(Evaluation::Void),
        }
    }

    pub fn eval_expression_identifier(&mut self, expression: &Expression) -> Result<String, Error> {
        match expression {
            Expression::Literal(literal) => match literal {
                Value::Bool(_) => Err(Error::RuntimeError {
                    message: String::from("Boolean is not an identifier"),
                }),
                Value::Identifier(ident) => Ok(String::from(ident)),
            },
            Expression::Parenthesized(expr) => self.eval_expression_identifier(expr),
            _ => Err(Error::RuntimeError {
                message: String::from("Cannot evaluate to an identifier"),
            }),
        }
    }

    pub fn eval_statement(&mut self, statement: &Statement) -> Option<Error> {
        let mut worklist: VecDeque<&Statement> = VecDeque::new();
        worklist.push_back(statement);

        while !worklist.is_empty() {
            match worklist.pop_front().unwrap() {
                Statement::Assigment { identifier, value } => {
                    let ident = match self.eval_expression_identifier(identifier) {
                        Ok(ident_value) => ident_value,
                        Err(error) => return Some(error),
                    };

                    let val: Value = match self.eval_expression(value) {
                        Ok(evaulation) => evaulation.into(),
                        Err(error) => return Some(error),
                    };

                    if !self.environment.set(&ident, &val) {
                        return Some(Error::RuntimeError {
                            message: String::from("Unknown identifier"),
                        });
                    }
                }
            }
        }

        None
    }

    pub fn get_environment(&self) -> Environment {
        self.environment.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::language::{
        environment::Environment,
        evaluation::Evaluation,
        expression::{BinaryOperator, Expression},
        statement::Statement,
        value::Value,
    };

    use super::Interpreter;

    fn new_logical_and(lhs: bool, rhs: bool) -> Expression {
        Expression::new_binary_expression(
            &Expression::new_boolean(&lhs),
            &BinaryOperator::LogicalAnd,
            &Expression::new_boolean(&rhs),
        )
    }

    fn new_logical_or(lhs: bool, rhs: bool) -> Expression {
        Expression::new_binary_expression(
            &Expression::new_boolean(&lhs),
            &BinaryOperator::LogicalOr,
            &Expression::new_boolean(&rhs),
        )
    }

    fn new_logical_implication(lhs: bool, rhs: bool) -> Expression {
        Expression::new_binary_expression(
            &Expression::new_boolean(&lhs),
            &BinaryOperator::Implication,
            &Expression::new_boolean(&rhs),
        )
    }

    fn new_logical_bi_implication(lhs: bool, rhs: bool) -> Expression {
        Expression::new_binary_expression(
            &Expression::new_boolean(&lhs),
            &BinaryOperator::BiImplication,
            &Expression::new_boolean(&rhs),
        )
    }

    fn new_logical_equal(lhs: bool, rhs: bool) -> Expression {
        Expression::new_binary_expression(
            &Expression::new_boolean(&lhs),
            &BinaryOperator::Equal,
            &Expression::new_boolean(&rhs),
        )
    }

    fn new_logical_not_equal(lhs: bool, rhs: bool) -> Expression {
        Expression::new_binary_expression(
            &Expression::new_boolean(&lhs),
            &BinaryOperator::NotEqual,
            &Expression::new_boolean(&rhs),
        )
    }

    #[test]
    fn test_interpreter_eval_logical_expressions() {
        let expressions = vec![
            // AND
            (new_logical_and(true, true), Evaluation::new_true()),
            (new_logical_and(false, true), Evaluation::new_false()),
            (new_logical_and(true, false), Evaluation::new_false()),
            (new_logical_and(false, false), Evaluation::new_false()),
            // OR
            (new_logical_or(true, true), Evaluation::new_true()),
            (new_logical_or(false, true), Evaluation::new_true()),
            (new_logical_or(true, false), Evaluation::new_true()),
            (new_logical_or(false, false), Evaluation::new_false()),
            // IMPLICATION
            (new_logical_implication(true, true), Evaluation::new_true()),
            (new_logical_implication(false, true), Evaluation::new_true()),
            (
                new_logical_implication(true, false),
                Evaluation::new_false(),
            ),
            (
                new_logical_implication(false, false),
                Evaluation::new_true(),
            ),
            // BI-IMPLICATION
            (
                new_logical_bi_implication(true, true),
                Evaluation::new_true(),
            ),
            (
                new_logical_bi_implication(false, true),
                Evaluation::new_false(),
            ),
            (
                new_logical_bi_implication(true, false),
                Evaluation::new_false(),
            ),
            (
                new_logical_bi_implication(false, false),
                Evaluation::new_true(),
            ),
            // EQUAL
            (new_logical_equal(true, true), Evaluation::new_true()),
            (new_logical_equal(false, true), Evaluation::new_false()),
            (new_logical_equal(true, false), Evaluation::new_false()),
            (new_logical_equal(false, false), Evaluation::new_true()),
            // NOT-EQUAL
            (new_logical_not_equal(true, true), Evaluation::new_false()),
            (new_logical_not_equal(false, true), Evaluation::new_true()),
            (new_logical_not_equal(true, false), Evaluation::new_true()),
            (new_logical_not_equal(false, false), Evaluation::new_false()),
        ];

        let mut interpreter = Interpreter::default();
        for (expression, expected) in expressions {
            let result = interpreter.eval_expression(&expression).ok().unwrap();

            assert_eq!(
                result,
                expected,
                "expression '{}' expected '{}' but got '{}'",
                expression.to_string(),
                expected,
                result
            );
        }
    }

    #[test]
    fn interpreter_eval_logical_expressions_associative_property() {
        let operators = vec![
            BinaryOperator::LogicalAnd,
            BinaryOperator::LogicalOr,
            BinaryOperator::BiImplication,
        ];
        let values = vec![true, false];

        let mut interpreter = Interpreter::default();

        for operator in operators {
            for first in values.clone() {
                for second in values.clone() {
                    for third in values.clone() {
                        let lhs = Expression::new_binary_expression(
                            &Expression::new_boolean(&first),
                            &operator,
                            &Expression::new_binary_expression(
                                &Expression::new_boolean(&second),
                                &operator,
                                &Expression::new_boolean(&third),
                            ),
                        );
                        let rhs = Expression::new_binary_expression(
                            &Expression::new_binary_expression(
                                &Expression::new_boolean(&first),
                                &operator,
                                &Expression::new_boolean(&second),
                            ),
                            &operator,
                            &Expression::new_boolean(&third),
                        );

                        let lhs_evaluation = interpreter.eval_expression(&lhs).ok().unwrap();
                        let rhs_evaluation = interpreter.eval_expression(&rhs).ok().unwrap();

                        assert_eq!(
                            lhs_evaluation,
                            rhs_evaluation,
                            "Expected associativity '{}={}' but got '{}!={}'",
                            lhs.to_string(),
                            rhs.to_string(),
                            lhs_evaluation,
                            rhs_evaluation
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn interpreter_eval_logical_expressions_commutative_property() {
        let operators = vec![
            BinaryOperator::LogicalAnd,
            BinaryOperator::LogicalOr,
            BinaryOperator::BiImplication,
        ];
        let values = vec![true, false];

        let mut interpreter = Interpreter::default();

        for operator in operators {
            for first in values.clone() {
                for second in values.clone() {
                    let lhs = Expression::new_binary_expression(
                        &Expression::new_boolean(&first),
                        &operator,
                        &Expression::new_boolean(&second),
                    );
                    let rhs = Expression::new_binary_expression(
                        &Expression::new_boolean(&second),
                        &operator,
                        &Expression::new_boolean(&first),
                    );

                    let lhs_evaluation = interpreter.eval_expression(&lhs).ok().unwrap();
                    let rhs_evaluation = interpreter.eval_expression(&rhs).ok().unwrap();

                    assert_eq!(
                        lhs_evaluation,
                        rhs_evaluation,
                        "Expected commutative '{}={}' but got '{}!={}'",
                        lhs.to_string(),
                        rhs.to_string(),
                        lhs_evaluation,
                        rhs_evaluation
                    );
                }
            }
        }
    }

    #[test]
    fn interpreter_eval_bool_literal_returns_value() {
        let environment = Environment::new_empty();
        let mut interpreter = Interpreter::new(&environment);
        let boolean = Expression::Literal(Value::Bool(false));

        let result = interpreter.eval_expression(&boolean).unwrap();

        assert_eq!(result, Evaluation::new_false());
    }

    #[test]
    fn interpreter_eval_identifier_returns_stored_value() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));
        let mut interpreter = Interpreter::new(&environment);
        let identifier = Expression::Literal(Value::Identifier(String::from("a")));

        let result = interpreter.eval_expression(&identifier).unwrap();

        assert_eq!(result, Evaluation::new_true());
    }

    #[test]
    fn interpreter_eval_unknown_identifier_panics() {
        let environment = Environment::new_empty();
        let mut interpreter = Interpreter::new(&environment);
        let literal = Expression::Literal(Value::Identifier(String::from("unknown identifier")));

        let result = interpreter.eval_expression(&literal);

        assert!(result.is_err());
    }

    #[test]
    fn interpreter_eval_correct_assignment_returns_none() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));
        let mut interpreter = Interpreter::new(&environment);
        let assignment = Statement::new_simple_assignment("a", &Value::new_false());

        let result = interpreter.eval_statement(&assignment);

        assert_eq!(result, None);
    }

    #[test]
    fn interpreter_eval_assignment_unknown_identifier() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));
        let mut interpreter = Interpreter::new(&environment);
        let assignment = Statement::new_simple_assignment("unknown", &Value::new_false());

        let result = interpreter.eval_statement(&assignment);

        assert_ne!(result, None);
    }

    #[test]
    fn interpreter_eval_assignment_incorrect_identifier_value_type() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));
        let mut interpreter = Interpreter::new(&environment);
        let assignment = Statement::new_assignment(
            &Expression::new_boolean(&false),
            &Expression::new_boolean(&false),
        );

        let result = interpreter.eval_statement(&assignment);

        assert_ne!(result, None);
    }
}
