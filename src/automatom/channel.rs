use std::{fmt::Display, hash::Hash, hash::Hasher};

#[derive(Debug, Clone, Eq)]
pub struct Channel {
    pub name: String,
    pub is_input: bool,
}

impl PartialEq for Channel {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Channel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Channel {
    pub fn new(name: &str, is_input: bool) -> Self {
        Self {
            name: String::from(name),
            is_input,
        }
    }

    pub fn new_input(name: &str) -> Self {
        Self::new(name, true)
    }

    pub fn new_output(name: &str) -> Self {
        Self::new(name, false)
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_input {
            return f.write_fmt(format_args!("{}?", self.name));
        }
        f.write_fmt(format_args!("{}!", self.name))
    }
}
