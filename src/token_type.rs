use parse_display::Display;

#[derive(Debug, Display, Clone, PartialEq)]
pub enum TokenType {
    //Single character tokens.
    #[display("(")]
    LeftParen,
    #[display(")")]
    RightParen,
    #[display("{{")]
    LeftBrace,
    #[display("}}")]
    RightBrace,
    #[display(",")]
    Comma,
    #[display(".")]
    Dot,
    #[display("?")]
    QuestionMark,
    #[display("-")]
    Minus,
    #[display("+")]
    Plus,
    #[display(":")]
    Colon,
    #[display(";")]
    Semicolon,
    #[display("/")]
    Slash,
    #[display("*")]
    Star,

    // One or two character tokens.
    #[display("!")]
    Bang,
    #[display("!=")]
    BangEqual,
    #[display("=")]
    Equal,
    #[display("==")]
    EqualEqual,
    #[display(">")]
    Greater,
    #[display(">=")]
    GreaterEqual,
    #[display("<")]
    Less,
    #[display("<=")]
    LessEqual,

    // Literals
    #[display("IDENTIFIER")]
    Identifier,
    #[display("STRING")]
    String,
    #[display("NUMBER")]
    Number,

    // Keywords
    #[display("AND")]
    And,
    #[display("OR")]
    Or,
    #[display("TRUE")]
    True,
    #[display("FALSE")]
    False,
    #[display("NIL")]
    Nil,
    #[display("IF")]
    If,
    #[display("ELSE")]
    Else,
    #[display("WHILE")]
    While,
    #[display("FOR")]
    For,
    #[display("BREAK")]
    Break,
    #[display("CONTINUE")]
    Continue,
    #[display("FUN")]
    Fun,
    #[display("CLASS")]
    Class,
    #[display("RETURN")]
    Return,
    #[display("PRINT")]
    Print,
    #[display("SUPER")]
    Super,
    #[display("THIS")]
    This,
    #[display("VAR")]
    Var,
    #[display("EOF")]
    Eof,
}
