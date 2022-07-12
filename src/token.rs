use crate::lox_literal::LoxLiteral;
use crate::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub line: usize,
    pub token_type: TokenType,
    pub literal: Option<LoxLiteral>,
}

impl Token {
    pub fn new(line: usize, token_type: TokenType, literal: Option<LoxLiteral>) -> Token {
        Token {
            line,
            token_type,
            literal,
        }
    }
}
