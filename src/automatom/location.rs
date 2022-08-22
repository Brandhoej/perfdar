use std::{fmt::Display, hash::Hash};

use super::invariant::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Location {
    Normal { name: String, invariant: Invariant },
    Initial { name: String, invariant: Invariant },
    Product { locations: Vec<Location> }, // TODO: Make the name a concatenation of composition ||, conjunction &&, quotient \\
    Inconsistent { name: String },
    Universal { name: String },
}

impl Hash for Location {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Location::Normal { name, invariant: _ } => name.hash(state),
            Location::Initial { name, invariant: _ } => name.hash(state),
            Location::Product { locations } => locations.hash(state),
            Location::Inconsistent { name } => name.hash(state),
            Location::Universal { name } => name.hash(state),
        }
    }
}

impl Location {
    pub fn new_normal(name: &str, invariant: &Invariant) -> Location {
        Location::Normal {
            name: String::from(name),
            invariant: invariant.clone(),
        }
    }

    pub fn new_initial(name: &str, invariant: &Invariant) -> Location {
        Location::Initial {
            name: String::from(name),
            invariant: invariant.clone(),
        }
    }

    pub fn new_product(locations: Vec<&Location>) -> Location {
        Location::Product {
            locations: locations.into_iter().cloned().collect(),
        }
    }

    pub fn new_inconsistent(name: &str) -> Location {
        Location::Inconsistent {
            name: String::from(name),
        }
    }

    pub fn new_universal(name: &str) -> Location {
        Location::Universal {
            name: String::from(name),
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Normal { name, invariant } => {
                f.write_fmt(format_args!("Location ({}, {})", name, invariant))
            }
            Location::Product { locations } => f.write_fmt(format_args!("Product {:?}", locations)),
            Location::Initial { name, invariant } => {
                f.write_fmt(format_args!("Initial location ({}, {})", name, invariant))
            }
            Location::Inconsistent { name } => {
                f.write_fmt(format_args!("Inconsistent location {}", name))
            }
            Location::Universal { name } => {
                f.write_fmt(format_args!("Universal location {}", name))
            }
        }
    }
}
