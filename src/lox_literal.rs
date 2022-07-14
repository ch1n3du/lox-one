use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum LoxLiteral {
    Boolean(bool),
    Number(f64),
    String(String),
    Identifier(String),
    Nil,
}

impl LoxLiteral {
    /// Returns the truthiness of a LoxLiteral
    pub fn is_truthy(&self) -> bool {
        use LoxLiteral::*;

        match self {
            Boolean(b) => b.to_owned(),
            Nil => false,
            _ => true,
        }
    }
}

impl fmt::Display for LoxLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LoxLiteral::*;

        match self {
            Boolean(b) => write!(f, "{}", b),
            Number(n) => write!(f, "{}", n),
            String(s) => write!(f, "{}", s),
            Identifier(id) => write!(f, "{}", id),
            Nil => write!(f, "nil"),
        }
    }
}
