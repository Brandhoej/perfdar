use std::collections::VecDeque;

use super::{
    environment::*,
    error::Error,
    evaluation::{self, Evaluation},
    node::*,
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

    pub fn eval<'a>(&'a mut self, node: &'a Node) -> Result<Evaluation, Error> {
        let mut stack: VecDeque<Value> = VecDeque::new();
        let mut worklist: VecDeque<&Node> = VecDeque::new();
        worklist.push_back(node);

        while !worklist.is_empty() {
            match worklist.pop_front().unwrap() {
                Node::Literal(literal) => {
                    if let Value::Identifier(ident) = literal {
                        if let Some(value) = self.environment.get(ident) {
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
                Node::Assigment { identifier, value } => {
                    let indent = match identifier {
                        Value::Bool(_) => {
                            return Err(Error::RuntimeError {
                                message: String::from("Assignment identifier is a boolean"),
                            })
                        }
                        Value::Identifier(ident) => ident,
                    };

                    if !self.environment.set(indent, &value) {
                        return Err(Error::RuntimeError {
                            message: String::from("Unknown identifier"),
                        });
                    }
                }
            }
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

    pub fn get_environment(&self) -> Environment {
        self.environment.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::language::{
        environment::Environment, evaluation::Evaluation, node::Node, value::Value,
    };

    use super::Interpreter;

    #[test]
    fn interpreter_eval_bool_literal_returns_value() {
        let environment = Environment::new_empty();
        let mut interpreter = Interpreter::new(&environment);
        let boolean = Node::Literal(Value::Bool(false));

        let result = interpreter.eval(&boolean).unwrap();

        assert_eq!(result, Evaluation::new_false());
    }

    #[test]
    fn interpreter_eval_identifier_returns_stored_value() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));
        let mut interpreter = Interpreter::new(&environment);
        let identifier = Node::Literal(Value::Identifier(String::from("a")));

        let result = interpreter.eval(&identifier).unwrap();

        assert_eq!(result, Evaluation::new_true());
    }

    #[test]
    fn interpreter_eval_unknown_identifier_panics() {
        let environment = Environment::new_empty();
        let mut interpreter = Interpreter::new(&environment);
        let literal = Node::Literal(Value::Identifier(String::from("unknown identifier")));

        let result = interpreter.eval(&literal);

        assert!(result.is_err());
    }

    #[test]
    fn interpreter_eval_correct_assignment_returns_none() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));
        let mut interpreter = Interpreter::new(&environment);
        let assignment = Node::Assigment {
            identifier: Value::Identifier(String::from("a")),
            value: Value::Bool(false),
        };

        let result = interpreter.eval(&assignment).ok().unwrap();

        assert_eq!(result, Evaluation::Void);
    }

    #[test]
    fn interpreter_eval_assignment_unknown_identifier() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));
        let mut interpreter = Interpreter::new(&environment);
        let assignment = Node::Assigment {
            identifier: Value::new_identifier("unknown"),
            value: Value::new_false(),
        };

        let result = interpreter.eval(&assignment);

        assert!(result.is_err());
    }

    #[test]
    fn interpreter_eval_assignment_incorrect_identifier_value_type() {
        let mut environment = Environment::new_empty();
        environment.insert("a", &Value::Bool(true));
        let mut interpreter = Interpreter::new(&environment);
        let assignment = Node::Assigment {
            identifier: Value::new_false(),
            value: Value::new_false(),
        };

        let result = interpreter.eval(&assignment);

        assert!(result.is_err());
    }
}
