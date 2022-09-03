use crate::{
    automatom::{edge::Edge, location::Location},
    language::environment::Environment,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    pub location: Location,
    pub environment: Environment,
}

impl State {
    pub fn new(location: &Location, environment: &Environment) -> Self {
        State {
            location: location.clone(),
            environment: environment.clone(),
        }
    }

    pub fn enables_any(&self, edges: &Vec<Edge>) -> bool {
        for edge in edges {
            if edge.enabled(&self) {
                return true;
            }
        }
        return false;
    }
}
