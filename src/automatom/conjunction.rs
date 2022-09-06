use super::{automaton::Automaton, error::Error};

#[derive(Debug, Clone)]
pub struct Conjunction<'a> {
    automata: Vec<&'a Automaton>
}

impl<'a> Conjunction<'a> {
    pub fn new(
        automata: Vec<&'a Automaton>
    ) -> Result<Self, Error> {
        todo!()
    }
}