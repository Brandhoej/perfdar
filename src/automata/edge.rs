use std::fmt::Debug;
use std::fmt::Display;

use super::channel::*;
use super::location::*;
use super::node::*;

#[derive(Debug, Clone, Hash)]
pub struct Edge {
    pub source: Location,
    pub action: Channel,
    pub transfer: Node,
    pub target: Location,
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} -({}, {})-> {}",
            self.source,
            self.action,
            self.transfer.to_string(),
            self.target
        ))
    }
}
