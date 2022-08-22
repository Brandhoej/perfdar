use std::collections::VecDeque;
use std::{collections::HashSet, fmt::Debug};

use crate::language::environment::Environment;
use crate::language::node::NodeType;
use crate::language::type_checker::TypeChecker;
use crate::transition_system::state::State;

use super::channel::*;
use super::edge::*;
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
        environment: Option<&Environment>,
    ) -> Result<Self, Error> {
        let mut inputs: HashSet<Channel> = HashSet::new();
        let mut outputs: HashSet<Channel> = HashSet::new();
        let mut actions: HashSet<Channel> = HashSet::new();
        let mut initials: HashSet<Location> = HashSet::new();
        let mut initial: Option<Location> = None;
        let declare_variables = environment != None;

        let initial_environment = if declare_variables {
            Environment::empty()
        } else {
            environment.unwrap().clone()
        };
        let type_checker = TypeChecker::new(&initial_environment);

        // Find all the inputs/outputs used as actions in the edges
        for edge in edges.clone() {
            actions.insert(edge.action.clone());
            if edge.action.is_input {
                inputs.insert(edge.action.clone());
            } else {
                outputs.insert(edge.action.clone());
            }

            if declare_variables {
                todo!("Add missing guard variables");
            } else {
                // Error handling: Check that all identifiers in the guard is declared
                if !initial_environment.contains_identifiers_in_node(&edge.guard.node) {
                    let missing = initial_environment.missing_identifiers_in_node(&edge.guard.node);
                    return Err(Error::MissingIdentifiersInEdgeGuard {
                        automaton: String::from(name),
                        edge: edge.clone(),
                        missing,
                    });
                }
            }
            // Error handling: Check that the guard is a logical node
            let actual = type_checker.check_node(&edge.guard.node);
            if actual != NodeType::Logical {
                return Err(Error::EdgeGuardIsNotLogical {
                    automaton: String::from(name),
                    edge: edge.clone(),
                    actual,
                });
            }

            // Error handling: Check that all identifiers in the update is declared
            if let Some(update) = edge.clone().update.node {
                if declare_variables {
                    todo!("Add missing update variables");
                } else {
                    if !initial_environment.contains_identifiers_in_node(&update) {
                        let missing = initial_environment.missing_identifiers_in_node(&update);
                        return Err(Error::MissingIdentifiersInEdgeUpdate {
                            automaton: String::from(name),
                            edge: edge.clone(),
                            missing,
                        });
                    }
                }

                // Error handling check that the update is a void node
                let actual = type_checker.check_node(&update);
                if actual != NodeType::Void {
                    return Err(Error::EdgeUpdateIsNotVoid {
                        automaton: String::from(name),
                        edge: edge.clone(),
                        actual,
                    });
                }
            }
        }

        // Error handling: Check that all identifiers are in the invariants
        let mut locations_worklist: VecDeque<Location> = VecDeque::new();
        locations_worklist.extend(locations.clone());

        while !locations_worklist.is_empty() {
            match locations_worklist.pop_back().unwrap() {
                Location::Normal { name: _, invariant } => {
                    if declare_variables {
                        todo!("Declare missing variable in normal location")
                    } else if !initial_environment.contains_identifiers_in_node(&invariant.node) {
                        todo!("return error")
                    }
                }
                Location::Product { locations } => {
                    for location in locations {
                        locations_worklist.push_back(location);
                    }
                }
                Location::Initial { name: _, invariant } => {
                    if declare_variables {
                        todo!("Declare missing variable in initial location")
                    } else if !initial_environment.contains_identifiers_in_node(&invariant.node) {
                        todo!("return error")
                    }
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
            return Err(Error::PartitionError {
                automaton: String::from(name),
                violating: intersection,
            });
        }

        // Error handling: Empty automaton
        if locations.len() == 0 {
            return Err(Error::EmptyAutomaton {
                automaton: String::from(name),
            });
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
            return Err(Error::MissingInitial {
                automaton: String::from(name),
            });
        } else if initials.len() > 1 {
            return Err(Error::TooManyInitialLocations {
                automaton: String::from(name),
                initials: initials.clone(),
            });
        }

        Ok(Automaton {
            name: String::from(name),
            locations: locations.clone(),
            edges: edges.clone(),
            actions,
            inputs,
            outputs,
            initial: initial.unwrap(),
            environment: environment.unwrap().clone(),
        })
    }

    pub fn get_environment(&self) -> Environment {
        self.environment.clone()
    }

    pub fn get_initial_location(&self) -> Location {
        self.initial.clone()
    }

    pub fn get_locations(&self) -> &HashSet<Location> {
        &self.locations
    }

    pub fn get_edges(&self) -> &HashSet<Edge> {
        &self.edges
    }

    pub fn get_actions(&self) -> &HashSet<Channel> {
        &self.actions
    }

    pub fn get_inputs(&self) -> &HashSet<Channel> {
        &self.inputs
    }

    pub fn get_outputs(&self) -> &HashSet<Channel> {
        &self.outputs
    }

    pub fn get_initial_state(&self) -> State {
        State::new(self.get_initial_location(), self.environment.clone())
    }

    pub fn ingoing_edges(&self, location: &Location, actions: &HashSet<Channel>) -> Vec<Edge> {
        let mut result = Vec::new();
        for edge in self.edges.clone() {
            if edge.target == *location && actions.contains(&edge.action) {
                result.push(edge);
            }
        }
        return result;
    }

    pub fn precedeeing_locations(
        &self,
        location: &Location,
        actions: &HashSet<Channel>,
    ) -> Vec<Location> {
        let mut result = Vec::new();
        for edge in self.ingoing_edges(location, actions) {
            result.push(edge.source);
        }
        return result;
    }

    pub fn outgoing_edges(&self, location: &Location, actions: &HashSet<Channel>) -> Vec<Edge> {
        let mut result = Vec::new();
        for edge in self.edges.clone() {
            if edge.source == *location && actions.contains(&edge.action) {
                result.push(edge.clone());
            }
        }
        return result;
    }

    pub fn sucedeeing_locations(
        &self,
        location: &Location,
        actions: &HashSet<Channel>,
    ) -> Vec<Location> {
        let mut result = Vec::new();
        for edge in self.outgoing_edges(location, actions) {
            result.push(edge.target);
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{
        automatom::{
            channel::Channel,
            edge::{Edge, Guard, Update},
            invariant::Invariant,
            location::Location,
        },
        language::environment::Environment,
    };

    use super::Automaton;

    fn create_first_automaton() -> Automaton {
        /* This is the structure of the automaton
         *    0
         *    |
         * (a?, true)
         *    V
         *    1
         *    |
         * (a?, false)
         *    V
         *    2<->(b?, true)
         * */
        let location_0 = Location::new_initial("0", &Invariant::new_true());
        let location_1 = Location::new_normal("1", &Invariant::new_true());
        let location_2 = Location::new_normal("2", &Invariant::new_true());

        let channel_a = Channel::new_input("a");
        let channel_b = Channel::new_input("b");

        let edge_0_1 = Edge::new(
            &location_0,
            &channel_a,
            &Guard::new_true(),
            &Update::empty(),
            &location_1,
        );
        let edge_1_2 = Edge::new(
            &location_1,
            &channel_a,
            &Guard::new_false(),
            &Update::empty(),
            &location_2,
        );
        let edge_2_2 = Edge::new(
            &location_2,
            &channel_b,
            &Guard::new_true(),
            &Update::empty(),
            &location_2,
        );

        let environment = Environment::empty();

        let locations = HashSet::from([location_0, location_1, location_2]);
        let edges = HashSet::from([edge_0_1, edge_1_2, edge_2_2]);

        match Automaton::new("first automaton", &locations, &edges, Some(&environment)) {
            Ok(automaton) => return automaton,
            Err(err) => panic!("{}", err),
        }
    }
}
