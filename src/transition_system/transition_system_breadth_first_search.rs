use std::collections::{HashSet, VecDeque};

use crate::automatom::channel::Channel;

use super::{state::State, transition_system::TransitionSystem};

#[derive(Clone)]
pub struct TransitionSystemBreadthFirstSearch<TS: TransitionSystem> {
    transition_system: TS,
    actions: HashSet<Channel>,
    visited: Vec<State>,
    frontier: VecDeque<State>,
}

impl<TS: TransitionSystem> TransitionSystemBreadthFirstSearch<TS> {
    pub fn new(actions: &HashSet<Channel>, transition_system: TS) -> Self {
        TransitionSystemBreadthFirstSearch {
            transition_system,
            actions: actions.clone(),
            visited: Vec::new(),
            frontier: VecDeque::new(),
        }
    }
}

impl<TS: TransitionSystem> Iterator for TransitionSystemBreadthFirstSearch<TS> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        // If true, then we have the first invocation to next and init it
        if self.visited.is_empty() && self.frontier.is_empty() {
            self.frontier
                .push_back(self.transition_system.get_initial_state());
        }

        // If true, then we have finished the search
        if self.frontier.is_empty() {
            return None;
        }

        let state = self.frontier.pop_back().unwrap();
        for next in self.transition_system.successors(&state, &self.actions) {
            if !self.visited.contains(&next) && !self.frontier.contains(&next) {
                self.frontier.push_back(next);
            }
        }

        None
    }
}
