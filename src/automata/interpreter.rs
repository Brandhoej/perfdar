use super::{environment::*, node::*};

#[derive(Default, Clone, Eq, PartialEq)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    fn new(environment: Environment) -> Self {
        Self { environment }
    }

    fn empty() -> Self {
        Interpreter::default()
    }

    pub fn eval(&mut self, node: Node) -> Option<Node> {
        return match node {
            Node::Boolean(_) => Some(node),
            Node::Identifier(identifier) => self.environment.lookup(&identifier),
            Node::Assigment { identifier, value } => {
                if let Node::Identifier(ident) = *identifier {
                    self.environment.set(&ident, &value);
                }
                None
            }
        };
    }
}
