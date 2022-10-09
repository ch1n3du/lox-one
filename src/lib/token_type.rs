use parse_display::Display;

#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    #[display("and")]
    And,
    #[display("or")]
    Or,
    #[display("true")]
    True,
    #[display("false")]
    False,
    #[display("nil")]
    Nil,
    #[display("if")]
    If,
    #[display("else")]
    Else,
    #[display("while")]
    While,
    #[display("for")]
    For,
    #[display("break")]
    Break,
    #[display("continue")]
    Continue,
    #[display("fun")]
    Fun,
    #[display("class")]
    Class,
    #[display("return")]
    Return,
    #[display("print")]
    Print,
    #[display("super")]
    Super,
    #[display("this")]
    This,
    #[display("VAR")]
    Var,
    #[display("EOF")]
    Eof,
}
