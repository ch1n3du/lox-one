use std::hash::Hash;

use crate::token_type::TokenType;

use super::{callable::Callable, function::Function};

use parse_display::Display;

#[derive(Debug, Display, Clone)]
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

    pub fn is_nil(&self) -> bool {
        match self {
            Self::Nil => true,
            _ => false,
        }
    }

    pub fn get_token_type(&self) -> TokenType {
        use LoxValue::*;
        match self {
            Number(_) => TokenType::Number,
            String(_) => TokenType::String,
            Boolean(true) => TokenType::True,
            Boolean(false) => TokenType::False,
            Function(_) => TokenType::Fun,
            Identifier(_) => TokenType::Identifier,
            Nil => TokenType::Nil,
        }
    }

    pub fn to_string(&self) -> String {
        use LoxValue::*;

        match self {
            Number(n) => n.to_string(),
            String(s) => s.clone(),
            Boolean(b) => b.to_string(),
            Function(f) => format!("{f}"),
            Identifier(_) => panic!("You can't concatenate an identifier stupid."),
            Nil => "nil".to_string(),
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
