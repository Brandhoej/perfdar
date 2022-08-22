use std::collections::{HashSet, VecDeque};

use crate::automatom::{automaton::Automaton, channel::Channel};

use super::state::State;

pub trait TransitionSystem {
    fn predecessors(&self, states: &Vec<State>, actions: &HashSet<Channel>) -> Vec<State>;
    fn input_predecessors(&self, states: &Vec<State>) -> Vec<State>;
    fn output_predecessors(&self, states: &Vec<State>) -> Vec<State>;
    fn successors(&self, states: &Vec<State>) -> Vec<State>;
}

impl TransitionSystem for Automaton {
    fn predecessors(&self, states: &Vec<State>, actions: &HashSet<Channel>) -> Vec<State> {
        let mut result = Vec::new();
        for state in states {
            let precedeeing_locations = self.precedeeing_locations(&state.location, &actions);
            let mut states_in_preceding_locations = Vec::new();

            // For all reachable states store the ones in the preceding locations
            for current in BreadthFirstSearch::new(&actions, self) {
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

    fn successors(&self, states: &Vec<State>) -> Vec<State> {
        let mut result = Vec::new();
        for state in states {
            for edge in self.outgoing_edges(&state.location, self.get_actions()) {
                if state.enables(&edge) {
                    let mut new_state = state.clone();
                    new_state.execute(&edge);
                    result.push(new_state);
                }
            }
        }
        return result;
    }
}

#[derive(Debug, Clone)]
pub struct BreadthFirstSearch {
    automaton: Automaton,
    actions: HashSet<Channel>,
    visited: Vec<State>,
    frontier: VecDeque<State>,
}

impl BreadthFirstSearch {
    pub fn new(actions: &HashSet<Channel>, automaton: &Automaton) -> Self {
        BreadthFirstSearch {
            automaton: automaton.clone(),
            actions: actions.clone(),
            visited: Vec::new(),
            frontier: VecDeque::new(),
        }
    }
}

impl Iterator for BreadthFirstSearch {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        // If true, then we have the first invocation to next and init it
        if self.visited.is_empty() && self.frontier.is_empty() {
            self.frontier.push_back(self.automaton.get_initial_state());
        }

        // If true, then we have finished the search
        if self.frontier.is_empty() {
            return None;
        }

        let state = self.frontier.pop_back().unwrap();
        for edge in self
            .automaton
            .outgoing_edges(&state.location, &self.actions)
        {
            if state.enables(&edge) {
                let mut next_state = state.clone();
                next_state.execute(&edge);

                if !self.visited.contains(&next_state) && !self.frontier.contains(&next_state) {
                    self.frontier.push_back(next_state);
                }
            }
        }

        None
    }
}
