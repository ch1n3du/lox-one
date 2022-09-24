use crate::token::{Position, Token};
use crate::token_type::TokenType;

use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ParserError {
    #[error("End of file reached, {0}.")]
    Eof(Position),
    #[error("Expected closing brace, {0}.")]
    ExpectedClosingBrace(Position),
    #[error("Unexpected token {0}, {1}.")]
    UnexpectedToken(Token, Position),
    #[error("{msg}, found '{found}' at {position}")]
    Expected {
        found: TokenType,
        msg: String,
        position: Position,
    },
    #[error("Arguments exceeded limit of 250, {0}")]
    ArgumentLimitReached(Position),
}

pub type ParserResult<T> = Result<T, ParserError>;
