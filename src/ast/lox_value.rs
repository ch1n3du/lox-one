use std::fmt;

use super::{callable::Callable, function::Function};

#[derive(Debug, Clone, PartialEq)]
pub enum LoxValue {
    Boolean(bool),
    Number(f64),
    String(String),
    Function(Function),
    Identifier(String),
    Nil,
}

impl LoxValue {
    /// Returns the truthiness of aLoxValue
    pub fn is_truthy(&self) -> bool {
        use LoxValue::*;

        match self {
            Boolean(b) => b.to_owned(),
            Nil => false,
            _ => true,
        }
    }

    pub fn as_callable(&self) -> Option<Box<dyn Callable>> {
        use LoxValue::*;

        match self {
            Function(fun) => Some(Box::new(fun.to_owned())),
            _ => None,
        }
    }
}

impl fmt::Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LoxValue::*;

        match self {
            Boolean(b) => write!(f, "{}", b),
            Number(n) => write!(f, "{}", n),
            String(s) => write!(f, "{}", s),
            Identifier(id) => write!(f, "{}", id),
            Nil => write!(f, "nil"),
            Function(fun) => write!(f, "{:?}", fun),
        }
    }
}
