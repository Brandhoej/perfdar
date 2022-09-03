use std::collections::VecDeque;
use std::{collections::HashSet, fmt::Debug};

use crate::language::environment::Environment;
use crate::language::interpreter::Interpreter;
use crate::language::node::Node;
use crate::language::node_type::NodeType;
use crate::language::type_checker::TypeChecker;
use crate::language::value::Value;

use super::channel::*;
use super::edge::*;
use super::error::*;
use super::invariant::Invariant;
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
    initial_environment: Environment,
}

impl Automaton {
    pub fn new(
        name: &str,
        edges: &HashSet<Edge>,
        environment: Option<&Environment>,
    ) -> Result<Self, Error> {
        let mut inputs: HashSet<Channel> = HashSet::new();
        let mut outputs: HashSet<Channel> = HashSet::new();
        let mut actions: HashSet<Channel> = HashSet::new();
        let mut initials: HashSet<Location> = HashSet::new();
        let mut locations: HashSet<Location> = HashSet::new();
        let mut initial: Option<Location> = None;
        // If we have no environment we automatically declare variables from edges and such
        let declare_variables = environment == None;

        let mut initial_environment = if declare_variables {
            Environment::new_empty()
        } else {
            environment.unwrap().clone()
        };

        // Find all the inputs/outputs used as actions in the edges
        let handle_edge_identifiers =
            |envir: &mut Environment, node: &Node| -> Option<Vec<String>> {
                let missing = envir.missing_identifiers_in_node(node);

                if declare_variables {
                    for identifier in missing.clone() {
                        envir.insert(&*identifier, &Value::new_false());
                    }
                }
                return if missing.len() == 0 {
                    None
                } else {
                    Some(missing)
                };
            };

        for edge in edges.clone() {
            actions.insert(edge.action.clone());
            match edge.action {
                Channel::In(_) => inputs.insert(edge.action.clone()),
                Channel::Out(_) => outputs.insert(edge.action.clone()),
            };

            if let Some(missing_identifiers) =
                handle_edge_identifiers(&mut initial_environment, &edge.guard.node)
            {
                // Error handling: Check that all identifiers in the guard is declared
                if !declare_variables {
                    return Err(Error::MissingIdentifiersInEdgeGuard {
                        automaton: String::from(name),
                        edge: edge.clone(),
                        missing: missing_identifiers,
                    });
                }
            }

            if let Some(update) = edge.clone().update.node {
                if let Some(missing_identifiers) =
                    handle_edge_identifiers(&mut initial_environment, &update)
                {
                    // Error handling: Check that all identifiers in the update is declared
                    if !declare_variables {
                        return Err(Error::MissingIdentifiersInEdgeUpdate {
                            automaton: String::from(name),
                            edge: edge.clone(),
                            missing: missing_identifiers,
                        });
                    }
                }
            }

            // Error handling: Check that the guard is a logical node
            let actual = TypeChecker::new(&initial_environment)
                .check_node(&edge.guard.node)
                .unwrap();
            if actual != NodeType::Logical {
                return Err(Error::EdgeGuardIsNotLogical {
                    automaton: String::from(name),
                    edge: edge.clone(),
                    actual,
                });
            }
        }

        // Find all locations referenced by the edges
        for edge in edges {
            locations.insert(edge.source.clone());
            locations.insert(edge.target.clone());
        }

        // Error handling: Check that all identifiers are in the invariants
        let mut locations_worklist: VecDeque<Location> = VecDeque::new();
        locations_worklist.extend(locations.clone());

        let mut check_invariant = |location: &Location, invariant: &Invariant| -> Option<Error> {
            // un-declared identifiers are handle the same way for invariants as edges
            if let Some(missing_identifiers) =
                handle_edge_identifiers(&mut initial_environment, &invariant.node)
            {
                if !declare_variables {
                    return Some(Error::MissingIdentifiersInLocationInvariant {
                        automaton: String::from(name),
                        location: location.clone(),
                        missing: missing_identifiers,
                    });
                }
            }
            None
        };

        while !locations_worklist.is_empty() {
            let current = locations_worklist.pop_back().unwrap();
            match current.clone() {
                Location::Normal { name: _, invariant } => {
                    if let Some(error) = check_invariant(&current, &invariant) {
                        return Err(error);
                    }
                }
                Location::Product { locations } => {
                    for location in locations {
                        locations_worklist.push_back(location);
                    }
                }
                Location::Initial { name: _, invariant } => {
                    if let Some(error) = check_invariant(&current, &invariant) {
                        return Err(error);
                    }
                }
                // All other location variants does not have an invariant or is a composition
                _ => (),
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
            return Err(Error::MissingInitialLocation {
                automaton: String::from(name),
            });
        } else if initials.len() > 1 {
            return Err(Error::TooManyInitialLocations {
                automaton: String::from(name),
                initials: initials.clone(),
            });
        }

        // Error handling: The invariant of the initial location is never enabled
        let unwrapped_initial = initial.unwrap();
        if let Location::Initial {
            name: _,
            ref invariant,
        } = unwrapped_initial
        {
            let mut interpreter = Interpreter::new(&initial_environment);
            let evaluation_result = interpreter.eval(&invariant.node.clone());
            if evaluation_result.is_err() {
                return Err(Error::InconsistentInitialLocation {
                    automaton: String::from(name),
                    location: unwrapped_initial.clone(),
                });
            }

            if let Some(evaluation) = evaluation_result.ok() {
                if evaluation.is_false() {
                    return Err(Error::InconsistentInitialLocation {
                        automaton: String::from(name),
                        location: unwrapped_initial.clone(),
                    });
                }
            }
        }

        Ok(Automaton {
            name: String::from(name),
            locations: locations.clone(),
            edges: edges.clone(),
            actions,
            inputs,
            outputs,
            initial: unwrapped_initial,
            initial_environment: initial_environment.clone(),
        })
    }

    pub fn get_initial_environment(&self) -> Environment {
        self.initial_environment.clone()
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
            channel::Channel, edge::Edge, error::Error, guard::Guard, invariant::Invariant,
            location::Location, update::Update,
        },
        language::{environment::Environment, node::Node, value::Value},
    };

    use super::Automaton;

    macro_rules! assert_err {
        ($result: ident, $err_type: pat_param) => {
            assert!($result.is_err());
            let err = $result.err().unwrap();
            assert!(matches!(err, $err_type), "error = {}", err);
        };
    }

    macro_rules! assert_ok {
        ($result: ident) => {
            if $result.is_err() {
                assert_eq!(
                    $result.is_err(),
                    false,
                    "error = {}",
                    $result.err().unwrap()
                );
            }
        };
    }

    #[test]
    fn automaton_new() {
        let invariant = &Invariant::new_true();
        let channel_in = Channel::new_input("in");
        let channel_out = Channel::new_output("out");
        let channels: HashSet<Channel> = HashSet::from([channel_in.clone(), channel_out.clone()]);
        let guard = Guard::new_true();
        let update = Update::new_pure();
        let a = Location::new_initial("a", invariant);
        let b = Location::new_normal("b", invariant);
        let c = Location::new_normal("c", invariant);
        let d = Location::new_normal("d", invariant);
        let e = Location::new_normal("e", invariant);
        let f = Location::new_normal("f", invariant);
        let a_b = Edge::new(&a, &channel_in, &guard, &update, &b);
        let b_c = Edge::new(&b, &channel_out, &guard, &update, &c);
        let c_d = Edge::new(&c, &channel_in, &guard, &update, &d);
        let c_e = Edge::new(&c, &channel_out, &guard, &update, &e);
        let c_f = Edge::new(&c, &channel_in, &guard, &update, &f);
        let edges = &HashSet::from([
            a_b.clone(),
            b_c.clone(),
            c_d.clone(),
            c_e.clone(),
            c_f.clone(),
        ]);

        let result = Automaton::new("my first automaton", edges, None);
        assert_ok!(result);

        let automaton = result.ok().unwrap();
        assert!(automaton.get_initial_environment().is_empty());
        assert_eq!(automaton.get_initial_location(), a);
        assert!(automaton.get_locations().contains(&a));
        assert!(automaton.get_locations().contains(&b));
        assert!(automaton.get_locations().contains(&c));
        assert!(automaton.get_locations().contains(&d));
        assert!(automaton.get_locations().contains(&e));
        assert!(automaton.get_locations().contains(&f));
        assert_eq!(automaton.get_locations().len(), 6);
        assert!(automaton.get_edges().contains(&a_b));
        assert!(automaton.get_edges().contains(&b_c));
        assert!(automaton.get_edges().contains(&c_d));
        assert!(automaton.get_edges().contains(&c_e));
        assert!(automaton.get_edges().contains(&c_f));
        assert_eq!(automaton.get_edges().len(), 5);
        assert!(automaton.get_actions().contains(&channel_in));
        assert!(automaton.get_actions().contains(&channel_out));
        assert_eq!(automaton.get_actions().len(), 2);
        assert!(automaton.get_inputs().contains(&channel_in));
        assert_eq!(automaton.get_inputs().len(), 1);
        assert!(automaton.get_outputs().contains(&channel_out));
        assert_eq!(automaton.get_outputs().len(), 1);

        let ingoing_edges_a = automaton.ingoing_edges(&a, &channels);
        let preceding_locations_a = automaton.precedeeing_locations(&a, &channels);
        let outgoing_edges_a = automaton.outgoing_edges(&a, &channels);
        let sucedeeing_locations_a = automaton.sucedeeing_locations(&a, &channels);
        assert_eq!(ingoing_edges_a.len(), 0);
        assert_eq!(preceding_locations_a.len(), 0);
        assert!(outgoing_edges_a.contains(&a_b));
        assert_eq!(outgoing_edges_a.len(), 1);
        assert!(sucedeeing_locations_a.contains(&b));
        assert_eq!(sucedeeing_locations_a.len(), 1);

        let ingoing_edges_b = automaton.ingoing_edges(&b, &channels);
        let preceding_locations_b = automaton.precedeeing_locations(&b, &channels);
        let outgoing_edges_b = automaton.outgoing_edges(&b, &channels);
        let sucedeeing_locations_b = automaton.sucedeeing_locations(&b, &channels);
        assert!(ingoing_edges_b.contains(&a_b));
        assert_eq!(ingoing_edges_b.len(), 1);
        assert!(preceding_locations_b.contains(&a));
        assert_eq!(preceding_locations_b.len(), 1);
        assert!(outgoing_edges_b.contains(&b_c));
        assert_eq!(outgoing_edges_b.len(), 1);
        assert!(sucedeeing_locations_b.contains(&c));
        assert_eq!(sucedeeing_locations_b.len(), 1);

        let ingoing_edges_c = automaton.ingoing_edges(&c, &channels);
        let preceding_locations_c = automaton.precedeeing_locations(&c, &channels);
        let outgoing_edges_c = automaton.outgoing_edges(&c, &channels);
        let sucedeeing_locations_c = automaton.sucedeeing_locations(&c, &channels);
        assert!(ingoing_edges_c.contains(&b_c));
        assert_eq!(ingoing_edges_c.len(), 1);
        assert!(preceding_locations_c.contains(&b));
        assert_eq!(preceding_locations_c.len(), 1);
        assert!(outgoing_edges_c.contains(&c_d));
        assert!(outgoing_edges_c.contains(&c_e));
        assert!(outgoing_edges_c.contains(&c_f));
        assert_eq!(outgoing_edges_c.len(), 3);
        assert!(sucedeeing_locations_c.contains(&d));
        assert!(sucedeeing_locations_c.contains(&e));
        assert!(sucedeeing_locations_c.contains(&f));
        assert_eq!(sucedeeing_locations_c.len(), 3);

        let ingoing_edges_d = automaton.ingoing_edges(&d, &channels);
        let preceding_locations_d = automaton.precedeeing_locations(&d, &channels);
        let outgoing_edges_d = automaton.outgoing_edges(&d, &channels);
        let sucedeeing_locations_d = automaton.sucedeeing_locations(&d, &channels);
        assert!(ingoing_edges_d.contains(&c_d));
        assert_eq!(ingoing_edges_d.len(), 1);
        assert!(preceding_locations_d.contains(&c));
        assert_eq!(preceding_locations_d.len(), 1);
        assert_eq!(outgoing_edges_d.len(), 0);
        assert_eq!(sucedeeing_locations_d.len(), 0);

        let ingoing_edges_e = automaton.ingoing_edges(&e, &channels);
        let preceding_locations_e = automaton.precedeeing_locations(&e, &channels);
        let outgoing_edges_e = automaton.outgoing_edges(&e, &channels);
        let sucedeeing_locations_e = automaton.sucedeeing_locations(&e, &channels);
        assert!(ingoing_edges_e.contains(&c_e));
        assert_eq!(ingoing_edges_e.len(), 1);
        assert!(preceding_locations_e.contains(&c));
        assert_eq!(preceding_locations_e.len(), 1);
        assert_eq!(outgoing_edges_e.len(), 0);
        assert_eq!(sucedeeing_locations_e.len(), 0);

        let ingoing_edges_f = automaton.ingoing_edges(&f, &channels);
        let preceding_locations_f = automaton.precedeeing_locations(&f, &channels);
        let outgoing_edges_f = automaton.outgoing_edges(&f, &channels);
        let sucedeeing_locations_f = automaton.sucedeeing_locations(&f, &channels);
        assert!(ingoing_edges_f.contains(&c_f));
        assert_eq!(ingoing_edges_f.len(), 1);
        assert!(preceding_locations_f.contains(&c));
        assert_eq!(preceding_locations_f.len(), 1);
        assert_eq!(outgoing_edges_f.len(), 0);
        assert_eq!(sucedeeing_locations_f.len(), 0);
    }

    #[test]
    fn automaton_new_missing_identifiers_in_edge_guard() {
        let location = Location::new_initial("initial", &Invariant::new_true());
        let channel = Channel::new_output("channel");
        let node = Node::new_identifier("ident");
        let edge = Edge::new_loop(&location, &channel, &Guard::new(&node), &Update::empty());
        let edges = HashSet::from([edge]);
        let environment = Environment::new_empty();
        let automaton = Automaton::new("automaton", &edges, Some(&environment));
        assert_err!(automaton, Error::MissingIdentifiersInEdgeGuard { .. });
    }

    #[test]
    fn automaton_new_edge_guard_is_not_logical() {
        let location = Location::new_initial("initial", &Invariant::new_true());
        let channel = Channel::new_output("channel");
        let identifier = "ident";
        let node = Node::new_assignment(identifier, &Value::new_false());
        let edge = Edge::new_loop(&location, &channel, &Guard::new(&node), &Update::empty());
        let edges = HashSet::from([edge]);
        let mut environment = Environment::new_empty();
        environment.insert(identifier, &Value::new_true());
        let automaton = Automaton::new("automaton", &edges, Some(&environment));
        assert_err!(automaton, Error::EdgeGuardIsNotLogical { .. });
    }

    #[test]
    fn automaton_new_missing_identifiers_in_edge_update() {
        let location = Location::new_initial("initial", &Invariant::new_true());
        let channel = Channel::new_output("channel");
        let node = Node::new_identifier("ident");
        let edge = Edge::new_loop(&location, &channel, &Guard::new_true(), &Update::new(&node));
        let edges = HashSet::from([edge]);
        let environment = Environment::new_empty();
        let automaton = Automaton::new("automaton", &edges, Some(&environment));
        assert_err!(automaton, Error::MissingIdentifiersInEdgeUpdate { .. });
    }

    #[test]
    fn automaton_new_partition_error() {
        let location = Location::new_initial("initial", &Invariant::new_true());
        let channel_ident = "channel";
        let in_channel = Channel::new_output(channel_ident);
        let out_channel = Channel::new_input(channel_ident);
        let node = Node::new_identifier("ident");
        let in_edge = Edge::new_loop(
            &location,
            &in_channel,
            &Guard::new_false(),
            &Update::new(&node),
        );
        let out_edge = Edge::new_loop(
            &location,
            &out_channel,
            &Guard::new_true(),
            &Update::new(&node),
        );
        let edges = HashSet::from([in_edge, out_edge]);
        let automaton = Automaton::new("automaton", &edges, None);
        assert_eq!(edges.len(), 2); // If the edges are the same then there is only a single element in the set
        assert_err!(automaton, Error::PartitionError { .. });
    }

    #[test]
    fn automaton_new_empty_automaton() {
        let edges = HashSet::default();
        let environment = Environment::new_empty();
        let automaton = Automaton::new("automaton", &edges, Some(&environment));
        assert_err!(automaton, Error::EmptyAutomaton { .. });
    }

    #[test]
    fn automaton_new_missing_initial_location() {
        let location = Location::new_normal("loc", &Invariant::new_true());
        let in_channel = Channel::new_output("in");
        let edge = Edge::new_loop(
            &location,
            &in_channel,
            &Guard::new_false(),
            &Update::new_pure(),
        );
        let edges = HashSet::from([edge]);
        let automaton = Automaton::new("automaton", &edges, None);
        assert_err!(automaton, Error::MissingInitialLocation { .. });
    }

    #[test]
    fn automaton_new_too_many_initial_location() {
        let location_a = Location::new_initial("a", &Invariant::new_true());
        let location_b = Location::new_initial("b", &Invariant::new_true());
        let in_channel = Channel::new_output("in");
        let edge = Edge::new(
            &location_a,
            &in_channel,
            &Guard::new_false(),
            &Update::new_pure(),
            &location_b,
        );
        let edges = HashSet::from([edge]);
        let automaton = Automaton::new("automaton", &edges, None);
        assert_err!(automaton, Error::TooManyInitialLocations { .. });
    }

    #[test]
    fn automaton_new_inconsistent_initial_location() {
        let location = Location::new_initial("a", &Invariant::new_false());
        let in_channel = Channel::new_output("in");
        let edge = Edge::new_loop(
            &location,
            &in_channel,
            &Guard::new_false(),
            &Update::new_pure(),
        );
        let edges = HashSet::from([edge]);
        let automaton = Automaton::new("automaton", &edges, None);
        assert_err!(automaton, Error::InconsistentInitialLocation { .. });
    }

    #[test]
    fn automaton_new_inconsistent_initial_location_with_boolean_evaluation() {
        let invariant_node = Node::new_identifier("bool");
        let mut environment = Environment::new_empty();
        environment.insert("bool", &Value::new_false());
        let location = Location::new_initial("a", &Invariant::new(&invariant_node));
        let in_channel = Channel::new_output("in");
        let edge = Edge::new_loop(
            &location,
            &in_channel,
            &Guard::new_false(),
            &Update::new_pure(),
        );
        let edges = HashSet::from([edge]);
        let automaton = Automaton::new("automaton", &edges, Some(&environment));
        assert_err!(automaton, Error::InconsistentInitialLocation { .. });
    }
}
