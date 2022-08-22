use std::collections::VecDeque;

use crate::{
    automatom::{edge::Edge, location::Location},
    language::{environment::Environment, interpreter::Interpreter, node::Value},
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    pub location: Location,
    pub environment: Environment,
}

impl State {
    pub fn new(location: Location, environment: Environment) -> Self {
        State {
            location,
            environment,
        }
    }

    pub fn enables_any(&self, edges: &Vec<Edge>) -> bool {
        for edge in edges {
            if self.enables(&edge) {
                return true;
            }
        }
        return false;
    }

    pub fn enables(&self, edge: &Edge) -> bool {
        if edge.source != self.location {
            return false;
        }

        let mut interpreter = Interpreter::new(&self.environment);
        let mut worklist: VecDeque<&Value> = VecDeque::new();
        let result = interpreter.eval(&edge.guard.node).unwrap();
        worklist.push_front(&result);

        while !worklist.is_empty() {
            match worklist.pop_front().unwrap() {
                Value::Bool(value) => return *value,
                Value::Identifier(ident) => {
                    worklist.push_back(self.environment.get(&ident).unwrap());
                }
            }
        }

        unreachable!()
    }

    pub fn execute(&mut self, edge: &Edge) {}
}
