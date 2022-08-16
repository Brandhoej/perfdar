use std::{collections::HashSet, fmt::Display};

use super::{channel::Channel, edge::Edge, location::Location};

#[derive(Debug, Clone)]
pub enum Error {
    MissingInitial {
        file: String,
        line: u32,
        automaton: String,
    },
    EmptyAutomaton {
        file: String,
        line: u32,
        automaton: String,
    },
    PartitionError {
        file: String,
        line: u32,
        automaton: String,
        violating: HashSet<Channel>,
    },
    TooManyInitialLocations {
        file: String,
        line: u32,
        automaton: String,
        initials: HashSet<Location>,
    },
    EdgeTransferMissingIdentifiers {
        file: String,
        line: u32,
        automaton: String,
        edge: Edge,
        identifiers: Vec<String>,
    },
    LocationInvariantMissingIdentifiers {
        file: String,
        line: u32,
        automaton: String,
        location: Location,
        identifiers: Vec<String>,
    },
    AggregatedError {
        errors: Vec<Error>,
    },
}

impl Error {
    pub fn try_aggregate(errors: Vec<Error>) -> Error {
        if errors.len() == 0 {
            panic!("No errors to aggregate")
        } else if errors.len() == 1 {
            return errors[0].clone();
        }
        Self::AggregatedError { errors }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingInitial {
                file: _,
                line: _,
                automaton,
            } => f.write_fmt(format_args!(
                "Automaton {} is missing an initial location",
                automaton
            )),
            Self::TooManyInitialLocations {
                file: _,
                line: _,
                automaton,
                initials,
            } => f.write_fmt(format_args!(
                "Automaton {} has too many initial locations: {:?}",
                automaton, initials
            )),
            Self::EmptyAutomaton {
                file: _,
                line: _,
                automaton,
            } => f.write_fmt(format_args!("Automaton {} is missing locations", automaton)),
            Self::PartitionError {
                file: _,
                line: _,
                automaton,
                violating,
            } => f.write_fmt(format_args!(
                "The actions of automaton {} is not a partition, caused by the actions {:?}",
                automaton, violating
            )),
            Self::AggregatedError { errors } => {
                let mut res = f.write_str("Multiple errors were encountered:\n");
                for error in errors {
                    res = (error as &dyn Display).fmt(f);
                }
                res
            }
            Error::EdgeTransferMissingIdentifiers {
                file: _,
                line: _,
                automaton,
                edge,
                identifiers,
            } => f.write_fmt(format_args!(
                "Automaton {} edge {} missing the identifiers {:?}",
                automaton, edge, identifiers
            )),
            Error::LocationInvariantMissingIdentifiers {
                file: _,
                line: _,
                automaton,
                location,
                identifiers,
            } => f.write_fmt(format_args!(
                "Automaton {} location {} missing identifiers {:?}",
                automaton, location, identifiers
            )),
        }
    }
}
