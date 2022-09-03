use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Display;

use crate::language::interpreter::Interpreter;
use crate::language::value::Value;
use crate::transition_system::state::State;

use super::channel::*;
use super::guard::Guard;
use super::location::*;
use super::update::Update;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Edge {
    pub source: Location,
    pub action: Channel,
    pub guard: Guard,
    pub update: Update,
    pub target: Location,
}

impl Edge {
    pub fn new(
        source: &Location,
        action: &Channel,
        guard: &Guard,
        update: &Update,
        target: &Location,
    ) -> Self {
        Self {
            source: source.clone(),
            action: action.clone(),
            guard: guard.clone(),
            update: update.clone(),
            target: target.clone(),
        }
    }

    pub fn new_loop(location: &Location, action: &Channel, guard: &Guard, update: &Update) -> Self {
        Self {
            source: location.clone(),
            action: action.clone(),
            guard: guard.clone(),
            update: update.clone(),
            target: location.clone(),
        }
    }

    pub fn execute(&self, state: &State) -> State {
        State::new(&self.target, &state.environment)
    }

    pub fn enabled(&self, state: &State) -> bool {
        if self.source != state.location {
            return false;
        }

        let mut interpreter = Interpreter::new(&state.environment);
        let mut worklist: VecDeque<&Value> = VecDeque::new();
        let result = interpreter.eval(&self.guard.node).unwrap().unwrap();
        worklist.push_front(&result);

        while !worklist.is_empty() {
            match worklist.pop_front().unwrap() {
                Value::Bool(value) => return *value,
                Value::Identifier(ident) => {
                    worklist.push_back(state.environment.get(&ident).unwrap());
                }
            }
        }

        unreachable!()
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} -({}, {}, {})-> {}",
            self.source, self.action, self.guard, self.update, self.target
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    use crate::{
        automatom::{channel::Channel, invariant::Invariant, location::Location},
        language::node::Node,
    };

    use super::{Edge, Guard, Update};

    #[test]
    fn edge_equality_in_and_out_channels() {
        // A channel can be split into input/output but essentially is just an identifier
        let location = Location::new_initial("initial", &Invariant::new_true());
        let channel_ident = "channel";
        let in_channel = Channel::new_output(channel_ident);
        let out_channel = Channel::new_input(channel_ident);
        let node = Node::new_identifier("ident");
        let in_edge = Edge::new_loop(
            &location,
            &in_channel,
            &Guard::new_true(),
            &Update::new(&node),
        );
        let out_edge = Edge::new_loop(
            &location,
            &out_channel,
            &Guard::new_true(),
            &Update::new(&node),
        );

        let mut in_hasher = DefaultHasher::new();
        in_edge.hash(&mut in_hasher);
        let in_hash = in_hasher.finish();

        let mut out_hasher = DefaultHasher::new();
        out_edge.hash(&mut out_hasher);
        let out_hash = out_hasher.finish();

        assert_eq!(in_edge, out_edge);
        assert_eq!(in_hash, out_hash);
    }
}
