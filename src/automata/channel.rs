use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Channel {
    pub name: String,
    pub is_input: bool,
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_input {
            return f.write_fmt(format_args!("{}?", self.name));
        }
        f.write_fmt(format_args!("{}!", self.name))
    }
}
