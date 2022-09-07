use std::collections::HashSet;

use crate::automatom::{automaton::Automaton, channel::Channel};

use super::{
    state::State, transition_system_breadth_first_search::TransitionSystemBreadthFirstSearch,
};

pub trait TransitionSystem {
    fn predecessors(&self, state: &State, actions: &HashSet<Channel>) -> Vec<State>;
    fn input_predecessors(&self, state: &State) -> Vec<State>;
    fn output_predecessors(&self, state: &State) -> Vec<State>;
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

    fn predecessors(&self, state: &State, actions: &HashSet<Channel>) -> Vec<State> {
        let mut result = Vec::new();
        let precedeeing_locations = self.precedeeing_locations(&state.location, &actions);
        let mut states_in_preceding_locations = Vec::new();

        // For all reachable states store the ones in the preceding locations
        for current in TransitionSystemBreadthFirstSearch::new(&actions, self.clone()) {
            if precedeeing_locations.contains(&current.location) {
                states_in_preceding_locations.push(current)
            }
        }

        // For all the states in the preceding locations the result is the ones which can execute the conencting edge
        let preceding_edges = self.ingoing_edges(&state.location, &actions);
        for preceding_state in states_in_preceding_locations {
            if preceding_state.enables_any(&preceding_edges) {
                result.push(preceding_state);
            }
        }
        return result;
    }

    fn input_predecessors(&self, state: &State) -> Vec<State> {
        self.predecessors(state, self.get_inputs())
    }

    fn output_predecessors(&self, state: &State) -> Vec<State> {
        self.predecessors(state, self.get_outputs())
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{
        automatom::{
            automaton::Automaton, channel::Channel, edge::Edge, guard::Guard, invariant::Invariant,
            location::Location, update::Update,
        },
        language::{environment::Environment, statement::Statement, value::Value},
        transition_system::{
            state::State,
            transition_system_breadth_first_search::TransitionSystemBreadthFirstSearch,
        },
    };

    use super::TransitionSystem;

    #[test]
    fn transition_system_get_initial_state() {
        let location = Location::new_initial("Initial", &Invariant::new_true());
        let channel = Channel::new_output("out");
        let guard = Guard::new_true();
        let update = Update::new_pure();
        let edge = Edge::new_loop(&location, &channel, &guard, &update);
        let edges = HashSet::from([edge]);
        let automaton = Automaton::new("Automaton", &edges, None).ok().unwrap();

        let initial_state = automaton.get_initial_state();

        assert_eq!(initial_state.location, automaton.get_initial_location());
        assert_eq!(
            initial_state.environment,
            automaton.get_initial_environment()
        );
    }

    #[test]
    fn transition_system_in_out_different_states_same_location() {
        let invariant = &Invariant::new_true();
        let channel_in = Channel::new_input("in");
        let channel_out = Channel::new_output("out");
        let in_channels: HashSet<Channel> = HashSet::from([channel_in.clone()]);
        let out_channels: HashSet<Channel> = HashSet::from([channel_out.clone()]);
        let channels: HashSet<Channel> = HashSet::from([channel_in.clone(), channel_out.clone()]);
        let a = Location::new_initial("a", invariant);
        let b = Location::new_normal("b", invariant);
        let guard = Guard::new_true();
        let mut environment = Environment::new_empty();
        environment.insert("ident", &Value::new_false());
        let update_false = Update::new(&Statement::new_simple_assignment(
            "ident",
            &Value::new_false(),
        ));
        let update_true = Update::new(&Statement::new_simple_assignment(
            "ident",
            &Value::new_true(),
        ));
        let a_b_in = Edge::new(&a, &channel_in, &guard, &update_false, &b);
        let a_b_out = Edge::new(&a, &channel_out, &guard, &update_true, &b);
        let edges = &HashSet::from([a_b_in.clone(), a_b_out.clone()]);

        let automaton = Automaton::new("my first automaton", edges, Some(&environment))
            .ok()
            .unwrap();

        let all_bfs = TransitionSystemBreadthFirstSearch::new(&channels, automaton.clone());
        let all_reachable_states: Vec<State> = all_bfs.collect();
        assert_eq!(all_reachable_states.len(), 3);

        let in_bfs = TransitionSystemBreadthFirstSearch::new(&in_channels, automaton.clone());
        let in_reachable_states: Vec<State> = in_bfs.collect();
        assert_eq!(in_reachable_states.len(), 2);

        let out_bfs = TransitionSystemBreadthFirstSearch::new(&out_channels, automaton.clone());
        let out_reachable_states: Vec<State> = out_bfs.collect();
        assert_eq!(out_reachable_states.len(), 2);
    }

    #[test]
    fn transition_system_full_system_test() {
        fn contains_location(states: &Vec<State>, location: &Location) -> bool {
            for state in states {
                if state.location == *location {
                    return true;
                }
            }
            false
        }

        let invariant = &Invariant::new_true();
        let channel_in = Channel::new_input("in");
        let channel_out = Channel::new_output("out");
        let in_channels: HashSet<Channel> = HashSet::from([channel_in.clone()]);
        let out_channels: HashSet<Channel> = HashSet::from([channel_out.clone()]);
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
        let a_c = Edge::new(&a, &channel_in, &guard, &update, &c);
        let a_d = Edge::new(&a, &channel_out, &guard, &update, &d);
        let a_e = Edge::new(&a, &channel_out, &guard, &update, &e);
        let b_f = Edge::new(&b, &channel_out, &guard, &update, &f);
        let c_f = Edge::new(&c, &channel_out, &guard, &update, &f);
        let d_f = Edge::new(&d, &channel_in, &guard, &update, &f);
        let e_f = Edge::new(&e, &channel_in, &guard, &update, &f);
        let edges = &HashSet::from([
            a_b.clone(),
            a_c.clone(),
            a_d.clone(),
            a_e.clone(),
            b_f.clone(),
            c_f.clone(),
            d_f.clone(),
            e_f.clone(),
        ]);

        let automaton = Automaton::new("my first automaton", edges, None)
            .ok()
            .unwrap();

        let initial_state = automaton.get_initial_state();
        assert_eq!(initial_state.location, automaton.get_initial_location());
        assert_eq!(
            initial_state.environment,
            automaton.get_initial_environment()
        );

        let bfs = TransitionSystemBreadthFirstSearch::new(&channels, automaton.clone());
        let all_states: Vec<State> = bfs.collect();
        assert_eq!(all_states.len(), 6);

        let a_state = &initial_state;
        let a_all_successors = automaton.successors(a_state, &channels);
        let a_in_successors = automaton.successors(a_state, &in_channels);
        let a_out_successors = automaton.successors(a_state, &out_channels);
        let a_all_predecessors = automaton.predecessors(a_state, &channels);
        let a_in_predecessors = automaton.predecessors(a_state, &in_channels);
        let a_out_predecessors = automaton.predecessors(a_state, &out_channels);
        assert!(contains_location(&a_all_successors, &b));
        assert!(contains_location(&a_all_successors, &c));
        assert!(contains_location(&a_all_successors, &d));
        assert!(contains_location(&a_all_successors, &e));
        assert_eq!(a_all_successors.len(), 4);
        assert!(contains_location(&a_in_successors, &b));
        assert!(contains_location(&a_in_successors, &c));
        assert_eq!(a_in_successors.len(), 2);
        assert!(contains_location(&a_out_successors, &d));
        assert!(contains_location(&a_out_successors, &e));
        assert_eq!(a_out_successors.len(), 2);
        assert_eq!(a_all_predecessors.len(), 0);
        assert_eq!(a_out_predecessors.len(), 0);
        assert_eq!(a_in_predecessors.len(), 0);

        let b_state = &State::new(&b, &automaton.get_initial_environment());
        let b_all_successors = automaton.successors(b_state, &channels);
        let b_in_successors = automaton.successors(b_state, &in_channels);
        let b_out_successors = automaton.successors(b_state, &out_channels);
        let b_all_predecessors = automaton.predecessors(b_state, &channels);
        let b_in_predecessors = automaton.predecessors(b_state, &in_channels);
        let b_out_predecessors = automaton.predecessors(b_state, &out_channels);
        assert!(contains_location(&b_all_successors, &f));
        assert_eq!(b_all_successors.len(), 1);
        assert_eq!(b_in_successors.len(), 0);
        assert!(contains_location(&b_out_successors, &f));
        assert_eq!(b_out_successors.len(), 1);
        assert!(contains_location(&b_all_predecessors, &a));
        assert_eq!(b_all_predecessors.len(), 1);
        assert!(contains_location(&b_in_predecessors, &a));
        assert_eq!(b_in_predecessors.len(), 1);
        assert_eq!(b_out_predecessors.len(), 0);

        let c_state = &State::new(&c, &automaton.get_initial_environment());
        let c_all_successors = automaton.successors(c_state, &channels);
        let c_in_successors = automaton.successors(c_state, &in_channels);
        let c_out_successors = automaton.successors(c_state, &out_channels);
        let c_all_predecessors = automaton.predecessors(c_state, &channels);
        let c_in_predecessors = automaton.predecessors(c_state, &in_channels);
        let c_out_predecessors = automaton.predecessors(c_state, &out_channels);
        assert!(contains_location(&c_all_successors, &f));
        assert_eq!(c_all_successors.len(), 1);
        assert_eq!(c_in_successors.len(), 0);
        assert!(contains_location(&c_out_successors, &f));
        assert_eq!(c_out_successors.len(), 1);
        assert!(contains_location(&c_all_predecessors, &a));
        assert_eq!(c_all_predecessors.len(), 1);
        assert!(contains_location(&c_in_predecessors, &a));
        assert_eq!(c_in_predecessors.len(), 1);
        assert_eq!(c_out_predecessors.len(), 0);

        let d_state = &State::new(&d, &automaton.get_initial_environment());
        let d_all_successors = automaton.successors(d_state, &channels);
        let d_in_successors = automaton.successors(d_state, &in_channels);
        let d_out_successors = automaton.successors(d_state, &out_channels);
        let d_all_predecessors = automaton.predecessors(d_state, &channels);
        let d_in_predecessors = automaton.predecessors(d_state, &in_channels);
        let d_out_predecessors = automaton.predecessors(d_state, &out_channels);
        assert!(contains_location(&d_all_successors, &f));
        assert_eq!(d_all_successors.len(), 1);
        assert!(contains_location(&d_in_successors, &f));
        assert_eq!(d_in_successors.len(), 1);
        assert_eq!(d_out_successors.len(), 0);
        assert!(contains_location(&d_all_predecessors, &a));
        assert_eq!(d_all_predecessors.len(), 1);
        assert_eq!(d_in_predecessors.len(), 0);
        assert!(contains_location(&d_out_predecessors, &a));
        assert_eq!(d_out_predecessors.len(), 1);

        let e_state = &State::new(&e, &automaton.get_initial_environment());
        let e_all_successors = automaton.successors(e_state, &channels);
        let e_in_successors = automaton.successors(e_state, &in_channels);
        let e_out_successors = automaton.successors(e_state, &out_channels);
        let e_all_predecessors = automaton.predecessors(e_state, &channels);
        let e_in_predecessors = automaton.predecessors(e_state, &in_channels);
        let e_out_predecessors = automaton.predecessors(e_state, &out_channels);
        assert!(contains_location(&e_all_successors, &f));
        assert_eq!(e_all_successors.len(), 1);
        assert!(contains_location(&e_in_successors, &f));
        assert_eq!(e_in_successors.len(), 1);
        assert_eq!(e_out_successors.len(), 0);
        assert!(contains_location(&e_all_predecessors, &a));
        assert_eq!(e_all_predecessors.len(), 1);
        assert_eq!(e_in_predecessors.len(), 0);
        assert!(contains_location(&e_out_predecessors, &a));
        assert_eq!(e_out_predecessors.len(), 1);

        let f_state = &State::new(&f, &automaton.get_initial_environment());
        let f_all_successors = automaton.successors(f_state, &channels);
        let f_in_successors = automaton.successors(f_state, &in_channels);
        let f_out_successors = automaton.successors(f_state, &out_channels);
        let f_all_predecessors = automaton.predecessors(f_state, &channels);
        let f_in_predecessors = automaton.predecessors(f_state, &in_channels);
        let f_out_predecessors = automaton.predecessors(f_state, &out_channels);
        assert_eq!(f_all_successors.len(), 0);
        assert_eq!(f_in_successors.len(), 0);
        assert_eq!(f_out_successors.len(), 0);
        assert!(contains_location(&f_all_predecessors, &b));
        assert!(contains_location(&f_all_predecessors, &c));
        assert!(contains_location(&f_all_predecessors, &d));
        assert!(contains_location(&f_all_predecessors, &e));
        assert_eq!(f_all_predecessors.len(), 4);
        // We have zero predecessors because no path exists with the actions from a to f
        assert_eq!(f_in_predecessors.len(), 0);
        assert_eq!(f_out_predecessors.len(), 0);
    }
}
