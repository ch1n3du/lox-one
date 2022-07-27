use std::{fmt, sync::mpsc::TryRecvError};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    //Single character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    QuestionMark,
    Minus,
    Plus,
    Colon,
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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenType::*;

        match self {
            Bang => write!(f, "!"),
            BangEqual => write!(f, "!"),
            Equal => write!(f, "="),
            EqualEqual => write!(f, "=="),
            Less => write!(f, "<"),
            LessEqual => write!(f, "<="),
            Greater => write!(f, ">"),
            GreaterEqual => write!(f, ">="),
            LeftBrace => write!(f, "{{"),
            RightBrace => write!(f, "}}"),
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            And => write!(f, "and"),
            Or => write!(f, "or"),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Star => write!(f, "*"),
            Slash => write!(f, "/"),
            Dot => write!(f, "."),
            QuestionMark => write!(f, "?"),
            Colon => write!(f, ":"),
            Semicolon => write!(f, ";"),
            Number => write!(f, "NUMBER"),
            String => write!(f, "STRING"),
            True => write!(f, "true"),
            False => write!(f, "false"),
            Identifier => write!(f, "IDENTIFIER"),
            _ => write!(f, ""),
        }
    }
}
