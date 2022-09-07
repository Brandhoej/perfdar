use std::collections::{hash_set::Intersection, HashSet};

use crate::language::environment::Environment;

use super::{automaton::Automaton, channel::Channel, edge::Edge, error::Error, location::Location};

#[derive(Debug, Clone)]
pub struct Conjunction<'a> {
    automata: Vec<&'a Automaton>,
    locations: HashSet<Location>,
    edges: HashSet<Edge>,
    actions: HashSet<Channel>,
    inputs: HashSet<Channel>,
    outputs: HashSet<Channel>,
    initial: Location,
    initial_environment: Environment,
}

impl<'a> Conjunction<'a> {
    pub fn new(automata: Vec<&'a Automaton>) -> Result<Self, Error> {
        // Atleast two automatons are required for a conjunction
        if automata.len() < 2 {
            // Too few operands/automata
        }

        let mut inputs: HashSet<Channel> = HashSet::default();
        let mut outputs: HashSet<Channel> = HashSet::default();
        let mut initial_locations: HashSet<Location> = HashSet::default();
        for i in automata.iter() {
            // Act_i = ⋃_{i∈I} Act_i^i
            inputs.extend(i.get_inputs().to_owned());
            // Act_o = ⋃_{i∈I} Act_o^i
            outputs.extend(i.get_outputs().to_owned());

            initial_locations.insert(i.get_initial_location());
        }
        let mut actions: HashSet<Channel> = HashSet::default();
        actions.extend(inputs.clone());
        actions.extend(outputs.clone());

        /* ∄a ∈ ⋃_{i∈I} Act^i s.t. a ∈ Act_i^i ∧ a ∈ Act_j^o, i, j ∈ I
         *    Where I is the indices for the set of automata*/
        for a in actions.iter() {
            if inputs.contains(a) && outputs.contains(a) {
                // Partition error
            }
        }

        /* Initial environment is the concatenated environment with output hiding */

        todo!()
    }

    fn combine_locations(locations: &Vec<Location>) -> Location {
        todo!()
    }
}
