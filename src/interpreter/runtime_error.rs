use std::fmt;

use crate::lox_literal::LoxLiteral;

#[derive(Debug)]
pub enum RuntimeError {
    Generic(String, usize),
    DivisionByZero(usize),
    VarDoesNotExist {
        name: String,
        line_no: usize,
    },
    IncorrectArity {
        name: String,
        line_no: usize,
    },
    NotCallable {
        type_name: LoxLiteral,
        line_no: usize,
    },
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RuntimeError::*;
        match self {
            Generic(reason, line_no) => write!(f, "{}, on line {}", reason, line_no),
            DivisionByZero(line_no) => {
                write!(f, "Division by zero error, on line {}.", line_no)
            }
            VarDoesNotExist { name, line_no } => {
                write!(
                    f,
                    "Variable '{}' isn't declared, on line {}.",
                    name, line_no
                )
            }
            IncorrectArity { name, line_no } => {
                write!(
                    f,
                    "Incorrect number of arguments in function '{}', on line {}",
                    name, line_no
                )
            }
            NotCallable { type_name, line_no } => {
                write!(
                    f,
                    "Type '{}' is not callable, on line {}.",
                    type_name, line_no
                )
            }
        }
    }
}
