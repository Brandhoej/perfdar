use std::{fmt::Display, hash::Hash, hash::Hasher};

#[derive(Debug, Clone, Eq)]
pub enum Channel {
    In(String),
    Out(String),
}

impl PartialEq for Channel {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::In(l0), Self::In(r0)) => l0 == r0,
            (Self::Out(l0), Self::Out(r0)) => l0 == r0,
            (Self::Out(l0), Self::In(r0)) => l0 == r0,
            (Self::In(l0), Self::Out(r0)) => l0 == r0,
        }
    }
}

impl Hash for Channel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Channel::In(name) => name.hash(state),
            Channel::Out(name) => name.hash(state),
        }
    }
}

impl Channel {
    pub fn new(name: &str, is_input: bool) -> Self {
        if is_input {
            Self::new_input(name)
        } else {
            Self::new_output(name)
        }
    }

    pub fn new_input(name: &str) -> Self {
        Channel::In(String::from(name))
    }

    pub fn new_output(name: &str) -> Self {
        Channel::Out(String::from(name))
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Channel::In(name) => f.write_fmt(format_args!("{}?", name)),
            Channel::Out(name) => f.write_fmt(format_args!("{}!", name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{hash_map::DefaultHasher, HashSet},
        hash::{Hash, Hasher},
    };

    use super::Channel;

    #[test]
    fn channel_equality() {
        let name = "name";
        let channel_a = Channel::new_input(name);
        let channel_b = Channel::new_output(name);
        assert_eq!(channel_a, channel_b);
        assert_eq!(channel_b, channel_a);
        assert_eq!(channel_a, channel_a);
        assert_eq!(channel_b, channel_b);
    }

    #[test]
    fn channel_hash() {
        let name = "name";
        let channel_a = Channel::new_input(name);
        let channel_b = Channel::new_output(name);
        let mut hasher_a = DefaultHasher::new();
        channel_a.hash(&mut hasher_a);
        let hash_a = hasher_a.finish();
        let mut hasher_b = DefaultHasher::new();
        channel_b.hash(&mut hasher_b);
        let hash_b = hasher_b.finish();
        assert_eq!(hash_a, hash_b);
    }

    #[test]
    fn channel_hashset() {
        let name = "name";
        let channel_a = Channel::new_input(name);
        let channel_b = Channel::new_output(name);
        let mut set_a: HashSet<Channel> = HashSet::new();
        let mut set_b: HashSet<Channel> = HashSet::new();
        set_a.insert(channel_a);
        set_b.insert(channel_b);

        let mut intersection: HashSet<&Channel> = HashSet::new();
        for element in set_a.intersection(&set_b) {
            intersection.insert(element);
        }

        assert_eq!(intersection.len(), 1);
        assert!(!set_a.is_disjoint(&set_b));
    }
}
