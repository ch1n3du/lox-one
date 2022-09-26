use crate::{interpreter::error::RuntimeError, parser::error::ParserError};
use colored::Colorize;

#[derive(Debug)]
pub enum LoxError {
    Parser(ParserError),
    Runtime(RuntimeError),
    IO(std::io::Error),
}

impl std::fmt::Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use LoxError::*;
        match self {
            Parser(e) => write!(f, "{} {}", "Parser Error:".red().bold(), e),
            Runtime(e) => write!(f, "{} {}", "Runtime Error".red().bold(), e),
            IO(e) => write!(f, "{} {}", "IO Error:".red().bold(), e),
        }
    }
}

pub type LoxResult<T> = Result<T, LoxError>;
