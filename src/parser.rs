<<<<<<< HEAD
use crate::ast::{BinOp, Expr, LogicalOp, Program, Stmt, UnaryOp};
use crate::tokens::{Token, TokenWithSpan};

pub struct Parser {
    tokens: Vec<TokenWithSpan>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithSpan>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Program {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }
        Program { statements }
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.matches(&[Token::Let]) {
            if let Ok(stmt) = self.let_declaration() {
                Some(stmt)
            } else {
                self.synchronize();
                None
            }
        } else {
            self.statement()
        }
    }

    fn let_declaration(&mut self) -> Result<Stmt, String> {
        let name = match self.consume_identifier() {
            Some(name) => name,
            None => return Err("Expected variable name after 'let'".to_string()),
        };

        let initializer = if self.matches(&[Token::Equals]) {
            Some(self.expression())
        } else {
            None
        };

        self.consume(Token::Semicolon, "Expected ';' after variable declaration");
        Ok(Stmt::Let { name, initializer })
    }

    fn statement(&mut self) -> Option<Stmt> {
        if self.matches(&[Token::If]) {
            self.if_statement()
        } else if self.matches(&[Token::While]) {
            self.while_statement()
        } else if self.matches(&[Token::LeftBrace]) {
            Some(self.block())
        } else if self.matches(&[Token::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn if_statement(&mut self) -> Option<Stmt> {
        self.consume(Token::LeftParen, "Expect '(' after 'if'.");
        let condition = self.expression();
        self.consume(Token::RightParen, "Expect ')' after if condition.");

        let then_branch = self.statement().expect("Expect statement for if body.");

        let else_branch = if self.matches(&[Token::Else]) {
            Some(Box::new(
                self.statement().expect("Expect statement for else body."),
            ))
        } else {
            None
        };

        Some(Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Option<Stmt> {
        self.consume(Token::LeftParen, "Expect '(' after 'while'.");
        let condition = self.expression();
        self.consume(Token::RightParen, "Expect ')' after while condition.");

        let body = self.statement().expect("Expect statement for while body.");

        Some(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }

    fn block(&mut self) -> Stmt {
        let mut statements = Vec::new();

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            if let Some(decl) = self.declaration() {
                statements.push(decl);
            }
        }

        self.consume(Token::RightBrace, "Expect '}' after block.");
        Stmt::Block(statements)
    }

    fn print_statement(&mut self) -> Option<Stmt> {
        let expr = self.expression();
        self.consume(Token::Semicolon, "Expected ';' after value");
        Some(Stmt::Print(expr))
    }

    fn expression_statement(&mut self) -> Option<Stmt> {
        let expr = self.expression();
        self.consume(Token::Semicolon, "Expected ';' after expression");
        Some(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> Expr {
        self.assignment()
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.logical_or();

        if self.matches(&[Token::Equals]) {
            let value = self.assignment();

            if let Expr::Variable(name) = expr {
                return Expr::Assign(name, Box::new(value));
            } else {
                panic!("Invalid assignment target");
            }
        }

        expr
    }

    fn logical_or(&mut self) -> Expr {
        let mut expr = self.logical_and();

        while self.matches(&[Token::Or]) {
            let operator = LogicalOp::Or;
            let right = self.logical_and();
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn logical_and(&mut self) -> Expr {
        let mut expr = self.equality();

        while self.matches(&[Token::And]) {
            let operator = LogicalOp::And;
            let right = self.equality();
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.matches(&[Token::EqualEqual, Token::BangEqual]) {
            let operator = match self.previous().token {
                Token::EqualEqual => BinOp::EqualEqual,
                Token::BangEqual => BinOp::BangEqual,
                _ => unreachable!(),
            };
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.matches(&[
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual,
        ]) {
            let operator = match self.previous().token {
                Token::Greater => BinOp::Greater,
                Token::GreaterEqual => BinOp::GreaterEqual,
                Token::Less => BinOp::Less,
                Token::LessEqual => BinOp::LessEqual,
                _ => unreachable!(),
            };
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.matches(&[Token::Plus, Token::Minus]) {
            let operator = match self.previous().token {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Subtract,
                _ => unreachable!(),
            };
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.matches(&[Token::Star, Token::Slash]) {
            let operator = match self.previous().token {
                Token::Star => BinOp::Multiply,
                Token::Slash => BinOp::Divide,
                _ => unreachable!(),
            };
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.matches(&[Token::Bang, Token::Minus]) {
            let operator = match self.previous().token {
                Token::Bang => UnaryOp::Not,
                Token::Minus => UnaryOp::Negate,
                _ => unreachable!(),
            };
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        // Check for boolean literals
        if self.matches(&[Token::True]) {
            return Expr::Boolean(true);
        }
        if self.matches(&[Token::False]) {
            return Expr::Boolean(false);
        }

        // Check for number
        if let Token::Number(value) = &self.tokens[self.current].token {
            let val = *value;
            self.advance();
            return Expr::Number(val);
        }

        // Check for identifier
        if let Token::Identifier(name) = &self.tokens[self.current].token {
            let name_clone = name.clone();
            self.advance();

            if self.check(&Token::LeftParen) {
                self.advance();
                let arguments = self.arguments();
                self.consume(Token::RightParen, "Expected ')' after arguments");
                return Expr::Call {
                    callee: name_clone,
                    arguments,
                };
            } else {
                return Expr::Variable(name_clone);
            }
        }

        if self.matches(&[Token::LeftParen]) {
            let expr = self.expression();
            self.consume(Token::RightParen, "Expected ')' after expression");
            return expr;
        }

        let current_token = &self.tokens[self.current].token;
        panic!(
            "Expected expression, found {:?} at position {}",
            current_token, self.current
        );
    }

    fn arguments(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();

        if !self.check(&Token::RightParen) {
            loop {
                args.push(self.expression());
                if !self.matches(&[Token::Comma]) {
                    break;
                }
            }
        }

        args
    }

    fn consume_identifier(&mut self) -> Option<String> {
        if let Token::Identifier(name) = &self.tokens[self.current].token {
            let name = name.clone();
            self.advance();
            Some(name)
        } else {
            None
        }
    }

    fn matches(&mut self, tokens: &[Token]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }

        let current_token = &self.tokens[self.current].token;

        match (current_token, token_type) {
            (Token::Identifier(_), Token::Identifier(_)) => true,
            (a, b) => a == b,
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if let Token::Semicolon = self.previous().token {
                return;
            }

            match self.tokens[self.current].token {
                Token::Let | Token::Print | Token::If | Token::While => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn advance(&mut self) -> &TokenWithSpan {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &TokenWithSpan {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token: Token, message: &str) {
        if self.check(&token) {
            self.advance();
        } else {
            panic!("{}", message);
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || matches!(self.tokens[self.current].token, Token::EOF)
    }
}
=======
use crate::ast::{BinOp, Expr, LogicalOp, Program, Stmt, UnaryOp};
use crate::tokens::{Token, TokenWithSpan};

pub struct Parser {
    tokens: Vec<TokenWithSpan>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithSpan>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Program {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }
        Program { statements }
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.matches(&[Token::Let]) {
            if let Ok(stmt) = self.let_declaration() {
                Some(stmt)
            } else {
                self.synchronize();
                None
            }
        } else {
            self.statement()
        }
    }

    fn let_declaration(&mut self) -> Result<Stmt, String> {
        let name = match self.consume_identifier() {
            Some(name) => name,
            None => return Err("Expected variable name after 'let'".to_string()),
        };

        let initializer = if self.matches(&[Token::Equals]) {
            Some(self.expression())
        } else {
            None
        };

        self.consume(Token::Semicolon, "Expected ';' after variable declaration");
        Ok(Stmt::Let { name, initializer })
    }

    fn statement(&mut self) -> Option<Stmt> {
        if self.matches(&[Token::If]) {
            self.if_statement()
        } else if self.matches(&[Token::While]) {
            self.while_statement()
        } else if self.matches(&[Token::LeftBrace]) {
            Some(self.block())
        } else if self.matches(&[Token::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn if_statement(&mut self) -> Option<Stmt> {
        self.consume(Token::LeftParen, "Expect '(' after 'if'.");
        let condition = self.expression();
        self.consume(Token::RightParen, "Expect ')' after if condition.");

        let then_branch = self.statement().expect("Expect statement for if body.");

        let else_branch = if self.matches(&[Token::Else]) {
            Some(Box::new(
                self.statement().expect("Expect statement for else body."),
            ))
        } else {
            None
        };

        Some(Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Option<Stmt> {
        self.consume(Token::LeftParen, "Expect '(' after 'while'.");
        let condition = self.expression();
        self.consume(Token::RightParen, "Expect ')' after while condition.");

        let body = self.statement().expect("Expect statement for while body.");

        Some(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }

    fn block(&mut self) -> Stmt {
        let mut statements = Vec::new();

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            if let Some(decl) = self.declaration() {
                statements.push(decl);
            }
        }

        self.consume(Token::RightBrace, "Expect '}' after block.");
        Stmt::Block(statements)
    }

    fn print_statement(&mut self) -> Option<Stmt> {
        let expr = self.expression();
        self.consume(Token::Semicolon, "Expected ';' after value");
        Some(Stmt::Print(expr))
    }

    fn expression_statement(&mut self) -> Option<Stmt> {
        let expr = self.expression();
        self.consume(Token::Semicolon, "Expected ';' after expression");
        Some(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> Expr {
        self.assignment()
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.logical_or();

        if self.matches(&[Token::Equals]) {
            let value = self.assignment();

            if let Expr::Variable(name) = expr {
                return Expr::Assign(name, Box::new(value));
            } else {
                panic!("Invalid assignment target");
            }
        }

        expr
    }

    fn logical_or(&mut self) -> Expr {
        let mut expr = self.logical_and();

        while self.matches(&[Token::Or]) {
            let operator = LogicalOp::Or;
            let right = self.logical_and();
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn logical_and(&mut self) -> Expr {
        let mut expr = self.equality();

        while self.matches(&[Token::And]) {
            let operator = LogicalOp::And;
            let right = self.equality();
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.matches(&[Token::EqualEqual, Token::BangEqual]) {
            let operator = match self.previous().token {
                Token::EqualEqual => BinOp::EqualEqual,
                Token::BangEqual => BinOp::BangEqual,
                _ => unreachable!(),
            };
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.matches(&[
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual,
        ]) {
            let operator = match self.previous().token {
                Token::Greater => BinOp::Greater,
                Token::GreaterEqual => BinOp::GreaterEqual,
                Token::Less => BinOp::Less,
                Token::LessEqual => BinOp::LessEqual,
                _ => unreachable!(),
            };
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.matches(&[Token::Plus, Token::Minus]) {
            let operator = match self.previous().token {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Subtract,
                _ => unreachable!(),
            };
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.matches(&[Token::Star, Token::Slash]) {
            let operator = match self.previous().token {
                Token::Star => BinOp::Multiply,
                Token::Slash => BinOp::Divide,
                _ => unreachable!(),
            };
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.matches(&[Token::Bang, Token::Minus]) {
            let operator = match self.previous().token {
                Token::Bang => UnaryOp::Not,
                Token::Minus => UnaryOp::Negate,
                _ => unreachable!(),
            };
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        // Check for boolean literals
        if self.matches(&[Token::True]) {
            return Expr::Boolean(true);
        }
        if self.matches(&[Token::False]) {
            return Expr::Boolean(false);
        }

        // Check for number
        if let Token::Number(value) = &self.tokens[self.current].token {
            let val = *value;
            self.advance();
            return Expr::Number(val);
        }

        // Check for identifier
        if let Token::Identifier(name) = &self.tokens[self.current].token {
            let name_clone = name.clone();
            self.advance();

            if self.check(&Token::LeftParen) {
                self.advance();
                let arguments = self.arguments();
                self.consume(Token::RightParen, "Expected ')' after arguments");
                return Expr::Call {
                    callee: name_clone,
                    arguments,
                };
            } else {
                return Expr::Variable(name_clone);
            }
        }

        if self.matches(&[Token::LeftParen]) {
            let expr = self.expression();
            self.consume(Token::RightParen, "Expected ')' after expression");
            return expr;
        }

        let current_token = &self.tokens[self.current].token;
        panic!(
            "Expected expression, found {:?} at position {}",
            current_token, self.current
        );
    }

    fn arguments(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();

        if !self.check(&Token::RightParen) {
            loop {
                args.push(self.expression());
                if !self.matches(&[Token::Comma]) {
                    break;
                }
            }
        }

        args
    }

    fn consume_identifier(&mut self) -> Option<String> {
        if let Token::Identifier(name) = &self.tokens[self.current].token {
            let name = name.clone();
            self.advance();
            Some(name)
        } else {
            None
        }
    }

    fn matches(&mut self, tokens: &[Token]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }

        let current_token = &self.tokens[self.current].token;

        match (current_token, token_type) {
            (Token::Identifier(_), Token::Identifier(_)) => true,
            (a, b) => a == b,
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if let Token::Semicolon = self.previous().token {
                return;
            }

            match self.tokens[self.current].token {
                Token::Let | Token::Print | Token::If | Token::While => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn advance(&mut self) -> &TokenWithSpan {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &TokenWithSpan {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token: Token, message: &str) {
        if self.check(&token) {
            self.advance();
        } else {
            panic!("{}", message);
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || matches!(self.tokens[self.current].token, Token::EOF)
    }
}
>>>>>>> f2fa646c3511ab8df1b1775b0c72186b2f2536cf
