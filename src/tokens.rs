#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
    EOF,
    Identifier(String),
    Equals,
    Semicolon,
    Comma,
    LeftBrace,    // {
    RightBrace,   // }
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=
    EqualEqual,   // ==
    BangEqual,    // !=
    Bang,         // !
    Let,
    Print,
    If,
    Else,
    While,
    True,
    False,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub struct TokenWithSpan {
    pub token: Token,
    pub span: (usize, usize),
}
