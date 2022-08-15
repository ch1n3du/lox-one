use std::fmt;

use crate::{lox_value::LoxValue, token::Position};

#[derive(Debug)]
pub enum RuntimeError {
    Generic(String, Position),
    DivisionByZero(Position),
    VarDoesNotExist {
        name: String,
        position: Position,
    },
    IncorrectArity {
        name: String,
        position: Position,
    },
    NotCallable {
        type_name: LoxValue,
        position: Position,
    },
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RuntimeError::*;
        match self {
            Generic(reason, position ) => write!(f, "{}, {}.", reason, position),
            DivisionByZero(position) => {
                write!(f, "Division by zero error, {}.", position)
            }
            VarDoesNotExist { name, position } => {
                write!(
                    f,
                    "Variable '{}' isn't declared, {}.",
                    name, position
                )
            }
            IncorrectArity { name, position } => {
                write!(
                    f,
                    "Incorrect number of arguments in function '{}', on line {}",
                    name, position
                )
            }
            NotCallable { type_name, position } => {
                write!(
                    f,
                    "Type '{}' is not callable, on line {}.",
                    type_name, position
                )
            }
        }
    }
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;
