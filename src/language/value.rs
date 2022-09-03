use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Value {
    Bool(bool),
    Identifier(String),
}

impl Value {
    pub fn new_true() -> Self {
        Value::Bool(true)
    }

    pub fn new_false() -> Self {
        Value::Bool(false)
    }

    pub fn new_identifier(ident: &str) -> Self {
        Value::Identifier(String::from(ident))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(value) => return f.write_str(&value.to_string()),
            Value::Identifier(identifier) => return f.write_str(identifier),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Value;

    #[test]
    fn value_new_true_construction() {
        let value = Value::new_true();
        assert!(matches!(value, Value::Bool(true)));
    }

    #[test]
    fn value_new_false_construction() {
        let value = Value::new_false();
        assert!(matches!(value, Value::Bool(false)));
    }

    #[test]
    fn value_new_identifier_construction() {
        let ident = String::from("ident");
        let value = Value::new_identifier(&ident.to_owned());
        assert!(matches!(value, Value::Identifier(value_ident) if value_ident == ident))
    }
}
