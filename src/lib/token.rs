use crate::lox_value::LoxValue;
use crate::token_type::TokenType;

use parse_display::Display;

#[derive(Debug, Display, Hash, PartialEq, Eq, Clone, Copy)]
#[display("line {line}, column {column}")]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Position {
        Position { line, column }
    }
}

#[derive(Debug, Display, Clone)]
#[display("<type: {token_type}, literal: {literal:?}, at {position}>")]
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
