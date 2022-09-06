use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum LangType {
    Logical,
    Void,
}

impl Display for LangType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LangType::Logical => f.write_str("logical"),
            LangType::Void => f.write_str("void"),
        }
    }
}
