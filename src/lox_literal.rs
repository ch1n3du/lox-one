use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum LoxLiteral {
    Boolean(bool),
    Number(f64),
    String(String),
    Nil,
}

impl fmt::Display for LoxLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LoxLiteral::*;

        match self {
            Boolean(b) => write!(f, "{}", b),
            Number(n) => write!(f, "{}", n),
            String(s) => write!(f, "{}", s),
            Nil => write!(f, "nil"),
        }
    }
}
