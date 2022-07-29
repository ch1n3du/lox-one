use std::fmt;

use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum ParserError {
    Eof(usize),
    ExpectedClosingBrace(usize),
    UnexpectedToken {
        line_no: usize,
        token: Token,
    },
    ExpectedOneOf {
        line_no: usize,
        token_types: Vec<TokenType>,
    },
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParserError::*;

        match self {
            Eof(line_no) => write!(f, "End of file reached, on line {}.", line_no),
            ExpectedClosingBrace(line_no) => {
                write!(f, "Expected closing brace, on line {}.", line_no)
            }
            UnexpectedToken { line_no, token } => write!(
                f,
                "Unexpected token '{}', on line {}",
                token.token_type, line_no
            ),
            ExpectedOneOf {
                line_no,
                token_types,
            } => match token_types.len() {
                1 => write!(f, "Expected '{}', on line {}", token_types[0], line_no),
                _ => {
                    let prefix = token_types[1..token_types.len() - 1]
                        .iter()
                        .fold(String::from(" "), |acc, tok_type| {
                            format!("{}, '{}'", acc, tok_type)
                        });

                    write!(
                        f,
                        "Expected one of '{}'{} or '{}' on line {}",
                        token_types[0],
                        prefix,
                        token_types[token_types.len() - 1],
                        line_no
                    )
                }
            },
        }
    }
}
