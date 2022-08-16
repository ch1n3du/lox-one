use super::{callable::Callable, function::Function};

use parse_display::Display;

#[derive(Debug, Display, Clone, PartialEq)]
pub enum LoxValue {
    #[display("{0}")]
    Boolean(bool),
    #[display("{0}")]
    Number(f64),
    #[display("{0}")]
    String(String),
    #[display("{0}")]
    Function(Function),
    #[display("{0}")]
    Identifier(String),
    #[display("nil")]
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