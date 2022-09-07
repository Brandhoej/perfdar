use std::{collections::HashSet, fmt::Display, hash::Hash};

use super::invariant::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Location {
    Normal {
        name: String,
        invariant: Invariant,
    },
    Initial {
        name: String,
        invariant: Invariant,
    },
    Conjunction {
        locations: Vec<Location>,
        invariant: Invariant,
    }, // TODO: Make the name a concatenation of composition ||, conjunction &&, quotient \\
    Inconsistent {
        name: String,
    },
    Universal {
        name: String,
    },
}

impl Hash for Location {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Location::Normal { name, .. } => name.hash(state),
            Location::Initial { name, .. } => name.hash(state),
            Location::Conjunction { locations, .. } => locations.hash(state),
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

    pub fn new_conjunction(locations: &Vec<Location>) -> Location {
        let conjoined_name = "some conjoined name";

        if locations.iter().any(|location| match location {
            Location::Inconsistent { .. } => true,
            _ => false,
        }) {
            return Location::Inconsistent {
                name: String::from(conjoined_name),
            };
        }

        if locations.iter().all(|location| match location {
            Location::Universal { .. } => true,
            _ => false,
        }) {
            return Location::Universal {
                name: String::from(conjoined_name),
            };
        }

        let filtered_locations: Vec<Location> = locations
            .iter()
            .filter(|location| match location {
                Location::Normal { .. } => true,
                Location::Initial { .. } => true,
                Location::Conjunction { .. } => true,
                _ => false,
            })
            .map(|location| location.clone())
            .collect();

        let mut invariants: HashSet<Invariant> = HashSet::default();
        for location in filtered_locations.clone() {
            if let Location::Normal { invariant, .. }
            | Location::Initial { invariant, .. }
            | Location::Conjunction { invariant, .. } = location
            {
                invariants.insert(invariant);
            }
        }
        Location::Conjunction {
            locations: filtered_locations,
            invariant: Invariant::new_conjunction(invariants),
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
            Location::Conjunction {
                locations,
                invariant,
            } => f.write_fmt(format_args!("Conjunction ({:?}, {})", locations, invariant)),
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

#[cfg(test)]
mod tests {
    use crate::automatom::invariant::Invariant;

    use super::Location;

    #[test]
    fn location_new_normal_construction() {
        let name = "some random name";
        let invariant = Invariant::new_true();
        let location = Location::new_normal(name, &invariant);
        assert!(
            matches!(location, Location::Normal { name: location_name, invariant: location_invariant }
                if location_name == name && location_invariant == invariant)
        );
    }

    #[test]
    fn location_new_initial_construction() {
        let name = "some random name";
        let invariant = Invariant::new_true();
        let location = Location::new_initial(name, &invariant);
        assert!(
            matches!(location, Location::Initial { name: location_name, invariant: location_invariant }
                if location_name == name && location_invariant == invariant)
        );
    }

    #[test]
    fn location_new_product_construction() {}

    #[test]
    fn location_new_inconsistent_construction() {}

    #[test]
    fn location_new_universal_construction() {}
}
