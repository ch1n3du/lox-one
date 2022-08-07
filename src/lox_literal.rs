use std::fmt;

use crate::{callable::Callable, function::Function, interpreter::runtime_error::RuntimeError};

#[derive(Debug, Clone, PartialEq)]
pub enum LoxLiteral {
    Boolean(bool),
    Number(f64),
    String(String),
    Function(Function),
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

    pub fn as_callable(&self) -> Option<Box<dyn Callable>> {
        use LoxLiteral::*;

        match self {
            Function(fun) => Some(Box::new(fun.to_owned())),
            _ => None,
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
            Function(fun) => write!(f, "{:?}", fun),
        }
    }
}
