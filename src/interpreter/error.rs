use crate::{lox_value::LoxValue, token::Position};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("{0}, {1}.")]
    Generic(String, Position),
    #[error("Division by zero error, {0}.")]
    DivisionByZero(Position),
    #[error("Variable '{name}' isn't declared, {position}.")]
    VarDoesNotExist { name: String, position: Position },
    #[error("Incorrect number of arguments in function '{name}', on line {position}")]
    IncorrectArity { name: String, position: Position },
    #[error("Type '{type_name}' is not callable, on line {position}.")]
    NotCallable {
        type_name: LoxValue,
        position: Position,
    },
    #[error("")]
    ValidContinue,
    #[error("'continue' can only be used within blocks, {0}.")]
    InvalidContinue(Position),
    #[error("")]
    ValidBreak,
    #[error("'break' can only be used within blocks, {0}.")]
    InvalidBreak(Position),
    #[error("'return' can only be used within blocks, {0}.")]
    InvalidReturn(Position),
    #[error("var '{0}' is being used in it's initializer, {1}.")]
    VarUsedInOwnInitializer(String, Position),
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;
