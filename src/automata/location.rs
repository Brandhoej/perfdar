use std::fmt::Display;

use super::invariant::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Location {
    Normal { name: String, invariant: Invariant },
    Product { locations: Vec<Location> },
    Initial { name: String, invariant: Invariant },
    Inconsistent { name: String },
    Universal { name: String },
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
