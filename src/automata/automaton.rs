use std::collections::VecDeque;
use std::{collections::HashSet, fmt::Debug};

use super::channel::*;
use super::edge::*;
use super::environment::Environment;
use super::error::*;
use super::location::*;

#[derive(Debug, Clone)]
pub struct Automaton {
    pub name: String,
    locations: HashSet<Location>,
    edges: HashSet<Edge>,
    actions: HashSet<Channel>,
    inputs: HashSet<Channel>,
    outputs: HashSet<Channel>,
    initial: Location,
    environment: Environment,
}

impl Automaton {
    pub fn new(
        name: &str,
        locations: &HashSet<Location>,
        edges: &HashSet<Edge>,
        environment: &Environment,
    ) -> Result<Self, Error> {
        let mut errors: Vec<Error> = Vec::new();
        let mut inputs: HashSet<Channel> = HashSet::new();
        let mut outputs: HashSet<Channel> = HashSet::new();
        let mut actions: HashSet<Channel> = HashSet::new();
        let mut initials: HashSet<Location> = HashSet::new();
        let mut initial: Option<Location> = None;

        // Find all the inputs/outputs used as actions in the edges
        for edge in edges.clone() {
            actions.insert(edge.action.clone());
            if edge.action.is_input {
                inputs.insert(edge.action);
            } else {
                outputs.insert(edge.action);
            }

            // Error handling: Check that all identifiers are in the environment
            if !environment.contains_identifiers_in(edge.transfer) {}
        }

        // Error handling: Check that all identifiers are in the invariants
        let mut locations_worklist: VecDeque<Location> = VecDeque::new();
        locations_worklist.extend(locations.clone());

        while !locations_worklist.is_empty() {
            match locations_worklist.pop_back().unwrap() {
                Location::Normal { name: _, invariant } => {
                    if !environment.contains_identifiers_in(invariant.node) {}
                }
                Location::Product { locations } => {
                    for location in locations {
                        locations_worklist.push_back(location);
                    }
                }
                Location::Initial { name: _, invariant } => {
                    if !environment.contains_identifiers_in(invariant.node) {}
                }
                Location::Inconsistent { name: _ } => todo!(),
                Location::Universal { name: _ } => todo!(),
            }
        }

        // Error handling: Actions are not partitioned into inputs/outputs
        if !inputs.is_disjoint(&outputs) {
            let mut intersection: HashSet<Channel> = HashSet::new();
            for joint in inputs.intersection(&outputs) {
                intersection.insert(joint.clone());
            }
            errors.push(Error::PartitionError {
                file: String::from(file!()),
                line: line!(),
                automaton: String::from(name),
                violating: intersection,
            })
        }

        // Error handling: Empty automaton
        if locations.len() == 0 {
            errors.push(Error::EmptyAutomaton {
                file: String::from(file!()),
                line: line!(),
                automaton: String::from(name),
            })
        }

        // Find all the locations marked as initial
        for location in locations.clone() {
            if let Location::Initial {
                name: _,
                invariant: _,
            } = location
            {
                initial = Some(location.clone());
                initials.insert(location.clone());
            }
        }

        // Error handling: Zero or more than one initial locations
        if initials.len() == 0 {
            errors.push(Error::MissingInitial {
                file: String::from(file!()),
                line: line!(),
                automaton: String::from(name),
            })
        } else if initials.len() > 1 {
            errors.push(Error::TooManyInitialLocations {
                file: String::from(file!()),
                line: line!(),
                automaton: String::from(name),
                initials: initials.clone(),
            })
        }

        // If the collection of errors is nto empty then we return an error
        if errors.len() > 0 {
            return Err(Error::try_aggregate(errors));
        }

        Ok(Automaton {
            name: String::from(name),
            locations: locations.clone(),
            edges: edges.clone(),
            actions,
            inputs,
            outputs,
            initial: initial.unwrap(),
            environment: environment.clone(),
        })
    }

    pub fn get_environment(&self) -> Environment {
        self.environment.clone()
    }

    pub fn get_initial(&self) -> Location {
        self.initial.clone()
    }

    pub fn get_locations(&self) -> HashSet<Location> {
        self.locations.clone()
    }

    pub fn get_edges(&self) -> HashSet<Edge> {
        self.edges.clone()
    }

    pub fn get_actions(&self) -> HashSet<Channel> {
        self.actions.clone()
    }

    pub fn get_inputs(&self) -> HashSet<Channel> {
        self.inputs.clone()
    }

    pub fn get_outputs(&self) -> HashSet<Channel> {
        self.outputs.clone()
    }
}
