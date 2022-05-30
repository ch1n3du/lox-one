#[derive(Debug, Clone)]
pub enum TokenType {
    //Single character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Or,
    True,
    False,
    Nil,
    If,
    Else,
    While,
    For,
    Fun,
    Class,
    Return,
    Print,
    Super,
    This,
    Var,

    Eof,
}
