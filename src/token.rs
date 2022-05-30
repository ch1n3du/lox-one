use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum Literal {
    Todo,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: Option<Vec<u8>>,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(
        ty: TokenType,
        lexeme: Option<Vec<u8>>,
        literal: Option<Literal>,
        line: usize,
    ) -> Token {
        Token {
            ty,
            lexeme,
            literal,
            line,
        }
    }
}
