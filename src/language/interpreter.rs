use std::collections::VecDeque;

use super::{environment::*, node::*};

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

    pub fn eval<'a>(&'a mut self, node: &'a Node) -> Option<&Value> {
        let mut stack: VecDeque<&Value> = VecDeque::new();
        let mut worklist: VecDeque<&Node> = VecDeque::new();
        worklist.push_back(node);

        while !worklist.is_empty() {
            match worklist.pop_front().unwrap() {
                Node::Literal(value) => stack.push_back(value),
                Node::Assigment { identifier, value } => {
                    let indent = match identifier {
                        Value::Bool(_) => todo!("Handle incorrect ident value type"),
                        Value::Identifier(ident) => ident,
                    };

                    self.environment.set(indent, &value);
                }
            }
        }

        // TODO: Handle if we have more values on the stack.
        return stack.pop_front();
    }
}
