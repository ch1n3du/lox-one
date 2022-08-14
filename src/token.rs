use crate::lox_value::LoxValue;
use crate::token_type::TokenType;

use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Position {
        Position { line, column }
    }
}


impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Option<LoxValue>,
    pub position: Position,
}

impl Token {
    pub fn new(token_type: TokenType, literal: Option<LoxValue>, position: Position) -> Token {
        Token {
            token_type,
            literal,
            position,
        }
    }
}
