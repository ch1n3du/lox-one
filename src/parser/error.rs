use crate::token::{Position, Token};
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum ParserError {
    Eof(Position),
    ExpectedClosingBrace(Position),
    UnexpectedToken(Token, Position),
    Expected {
        found: TokenType,
        msg: String,
        position: Position,
    },
    ArgumentLimitReached(Position),
    Bundle(Vec<ParserError>),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParserError::*;
        match self {
            Eof(p) => write!(f, "End of file reached, {p}."),
            ExpectedClosingBrace(p) => write!(f, "Expected closing brace, {p}."),
            UnexpectedToken(token, p) => write!(f, "Unexpected token {token}, {p}"),
            Expected {
                found,
                msg,
                position,
            } => write!(f, "{msg}, found '{found}' at {position}"),
            ArgumentLimitReached(p) => write!(f, "Arguments exceeded limit of 250, {p}"),
            Bundle(errs) => {
                for err in errs {
                    writeln!(f, "{err}")?;
                }

                Ok(())
            }
        }
    }
}

pub type ParserResult<T> = Result<T, ParserError>;
