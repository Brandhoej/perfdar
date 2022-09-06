use std::fmt::Display;

use crate::language::statement::Statement;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Update {
    pub node: Option<Statement>,
}

impl Default for Update {
    fn default() -> Self {
        Self::empty()
    }
}

impl Display for Update {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(node) = &self.node {
            return f.write_fmt(format_args!("{}", node.to_string()));
        }
        return f.write_str("void");
    }
}

impl Update {
    pub fn new(statement: &Statement) -> Self {
        Self {
            node: Some(statement.clone()),
        }
    }

    pub fn new_pure() -> Self {
        Self::empty()
    }

    pub fn empty() -> Self {
        Self { node: None }
    }
}
