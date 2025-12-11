#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Number(i64),
    Plus,
    Minus,
    StringLiteral(String),
    Star,
    Slash,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    EOF,
    Identifier(String),
    Equals,
    Semicolon,
    Colon,
    Comma,
    Dot,
    LeftBrace,    // {
    RightBrace,   // }
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=
    EqualEqual,   // ==
    BangEqual,    // !=
    Bang,         // !

    // Keywords
    Let,
    Print,
    If,
    Else,
    While,
    True,
    False,
    And,
    Or,
    Fn,
    Return,
    For,
    In,
}

#[derive(Debug, Clone)]
pub struct TokenWithSpan {
    pub token: Token,
    pub span: (usize, usize),
}
