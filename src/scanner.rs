use crate::tokens::{Token, TokenWithSpan};

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<TokenWithSpan> {
        let mut tokens = Vec::new();
        while !self.is_at_end() {
            self.start = self.current;
            if let Some(token) = self.scan_token() {
                tokens.push(TokenWithSpan {
                    token,
                    span: (self.start, self.current),
                });
            }
        }
        tokens.push(TokenWithSpan {
            token: Token::EOF,
            span: (self.current, self.current),
        });
        tokens
    }

    fn scan_token(&mut self) -> Option<Token> {
        let c = self.advance();
        match c {
            '"' => {
                let mut string = String::new();
                while self.peek() != '"' && !self.is_at_end() {
                    if self.peek() == '\n' {
                        self.line += 1;
                    }
                    string.push(self.advance());
                }
                if self.is_at_end() {
                    panic!("Unterminated string at line {}", self.line);
                }
                self.advance(); // consume closing "
                Some(Token::StringLiteral(string))
            }

            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Star),
            '/' => {
                if self.peek() == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    None
                } else {
                    Some(Token::Slash)
                }
            }
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '{' => Some(Token::LeftBrace),
            '}' => Some(Token::RightBrace),
            ';' => Some(Token::Semicolon),
            ',' => Some(Token::Comma),

            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    Some(Token::BangEqual)
                } else {
                    Some(Token::Bang)
                }
            }
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    Some(Token::EqualEqual)
                } else {
                    Some(Token::Equals)
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    Some(Token::GreaterEqual)
                } else {
                    Some(Token::Greater)
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    Some(Token::LessEqual)
                } else {
                    Some(Token::Less)
                }
            }

            ' ' | '\t' | '\r' => None,
            '\n' => {
                self.line += 1;
                None
            }

            '0'..='9' => {
                while self.peek().is_ascii_digit() {
                    self.advance();
                }
                let num_str: String = self.source[self.start..self.current].iter().collect();
                Some(Token::Number(num_str.parse().unwrap()))
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                while self.peek().is_alphanumeric() || self.peek() == '_' {
                    self.advance();
                }
                let text: String = self.source[self.start..self.current].iter().collect();
                match text.as_str() {
                    "let" => Some(Token::Let),
                    "print" => Some(Token::Print),
                    "if" => Some(Token::If),
                    "else" => Some(Token::Else),
                    "while" => Some(Token::While),
                    "true" => Some(Token::True),
                    "false" => Some(Token::False),
                    "and" => Some(Token::And),
                    "or" => Some(Token::Or),
                    "fn" => Some(Token::Fn),
                    "return" => Some(Token::Return),
                    _ => Some(Token::Identifier(text)),
                }
            }

            _ => panic!("Unexpected character: '{}' at line {}", c, self.line),
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
