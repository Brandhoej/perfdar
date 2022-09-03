use crate::language::node_type::NodeType;

use super::{channel::Channel, edge::Edge, location::Location};
use std::collections::HashSet;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Automaton {automaton:} is missing an initial location")]
    MissingInitialLocation { automaton: String },
    #[error("Automaton {automaton:} is empty")]
    EmptyAutomaton { automaton: String },
    #[error(
        "Automaton {automaton:} actions are not partitioned, violating actions is {violating:?}"
    )]
    PartitionError {
        automaton: String,
        violating: HashSet<Channel>,
    },
    #[error("Automaton {automaton:} has too many initial locations: {:?}")]
    TooManyInitialLocations {
        automaton: String,
        initials: HashSet<Location>,
    },
    #[error("Automaton {automaton:} {location:} is an inconsistent initial location")]
    InconsistentInitialLocation {
        automaton: String,
        location: Location,
    },
    #[error("Automaton {automaton:} {location:} is missing the identifiers {identifiers:?}")]
    LocationInvariantMissingIdentifiers {
        automaton: String,
        location: Location,
        identifiers: Vec<String>,
    },
    #[error("Automaton {automaton:} edge {:}-{:}->{:} guard {:} is missing the identifiers {missing:?}", .edge.source, .edge.action, edge.target, .edge.guard)]
    MissingIdentifiersInEdgeGuard {
        automaton: String,
        edge: Edge,
        missing: Vec<String>,
    },
    #[error("Automaton {automaton:} edge {:}-{:}->{:} guard {:} is not {:} but instead {:}", .edge.source, .edge.action, edge.target, .edge.guard, NodeType::Logical, actual)]
    EdgeGuardIsNotLogical {
        automaton: String,
        edge: Edge,
        actual: NodeType,
    },
    #[error("Automaton {automaton:} edge {:}-{:}->{:} update {:} is missing the identifiers {missing:?}", .edge.source, .edge.action, edge.target, .edge.update)]
    MissingIdentifiersInEdgeUpdate {
        automaton: String,
        edge: Edge,
        missing: Vec<String>,
    },
    #[error("Automaton {automaton:} {location:} is missing the identifiers {missing:?}")]
    MissingIdentifiersInLocationInvariant {
        automaton: String,
        location: Location,
        missing: Vec<String>,
    },
}
