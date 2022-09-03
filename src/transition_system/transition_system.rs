use std::collections::HashSet;

use crate::automatom::{automaton::Automaton, channel::Channel};

use super::{
    state::State, transition_system_breadth_first_search::TransitionSystemBreadthFirstSearch,
};

pub trait TransitionSystem {
    fn predecessors(&self, states: &Vec<State>, actions: &HashSet<Channel>) -> Vec<State>;
    fn input_predecessors(&self, states: &Vec<State>) -> Vec<State>;
    fn output_predecessors(&self, states: &Vec<State>) -> Vec<State>;
    fn successors(&self, state: &State, actions: &HashSet<Channel>) -> Vec<State>;
    fn get_initial_state(&self) -> State;
    fn get_actions(&self) -> &HashSet<Channel>;
    fn get_inputs(&self) -> &HashSet<Channel>;
    fn get_outputs(&self) -> &HashSet<Channel>;
}

impl TransitionSystem for Automaton {
    fn get_initial_state(&self) -> State {
        State::new(
            &self.get_initial_location(),
            &self.get_initial_environment().clone(),
        )
    }

    fn predecessors(&self, states: &Vec<State>, actions: &HashSet<Channel>) -> Vec<State> {
        let mut result = Vec::new();
        for state in states {
            let precedeeing_locations = self.precedeeing_locations(&state.location, &actions);
            let mut states_in_preceding_locations = Vec::new();

            // For all reachable states store the ones in the preceding locations
            for current in TransitionSystemBreadthFirstSearch::new(&actions, self.clone()) {
                if precedeeing_locations.contains(&current.location) {
                    states_in_preceding_locations.push(current)
                }
            }

            // For all the states in the preceding locations the result is the ones which can execute the conencting edge
            let precesing_edges = self.ingoing_edges(&state.location, &actions);
            for preceding_state in states_in_preceding_locations {
                if preceding_state.enables_any(&precesing_edges) {
                    result.push(preceding_state);
                }
            }
        }
        return result;
    }

    fn input_predecessors(&self, states: &Vec<State>) -> Vec<State> {
        self.predecessors(states, self.get_inputs())
    }

    fn output_predecessors(&self, states: &Vec<State>) -> Vec<State> {
        self.predecessors(states, self.get_outputs())
    }

    fn successors(&self, state: &State, actions: &HashSet<Channel>) -> Vec<State> {
        let mut result = Vec::new();
        for edge in self.outgoing_edges(&state.location, actions) {
            if edge.enabled(&state) {
                result.push(edge.execute(&state));
            }
        }
        return result;
    }

    fn get_actions(&self) -> &HashSet<Channel> {
        self.get_actions()
    }

    fn get_inputs(&self) -> &HashSet<Channel> {
        self.get_inputs()
    }

    fn get_outputs(&self) -> &HashSet<Channel> {
        self.get_outputs()
    }
}
