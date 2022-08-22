use crate::automatom::channel::Channel;

use super::state::State;

pub struct Transition {
    pub source: State,
    pub action: Channel,
    pub target: State,
}
