use crate::{
    error::{self, LoxError},
    expr::Expr,
    object::Object,
    token::Token,
    token_type::TokenType,
};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        use TokenType::*;

        let mut expr = self.comparison();

        while self.match_tokentype(&[BangEqual, EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::binary(expr, operator, right);
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        use TokenType::*;

        let mut expr = self.term();

        while self.match_tokentype(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::binary(expr, operator, right);
        }

        expr
    }

    fn term(&mut self) -> Expr {
        use TokenType::*;

        let mut expr = self.factor();

        while self.match_tokentype(&[Minus, Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::binary(expr, operator, right);
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        use TokenType::*;

        let mut expr = self.unary();

        while self.match_tokentype(&[Slash, Star]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::binary(expr, operator, right);
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        use TokenType::*;

        if self.match_tokentype(&[Bang, Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::unary(operator, right);
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        use TokenType::*;

        if self.match_tokentype(&[False]) {
            return Expr::literal(Object::Bool(false));
        }
        if self.match_tokentype(&[True]) {
            return Expr::literal(Object::Bool(true));
        }
        if self.match_tokentype(&[Nil]) {
            return Expr::literal(Object::Nil);
        }

        if self.match_tokentype(&[Number, String]) {
            return Expr::literal(self.previous().clone().literal);
        }

        if self.match_tokentype(&[LeftParen]) {
            let expr = self.expression();
            self.consume(RightParen, "Expect ')' after expression.");
            return Expr::grouping(expr);
        }

        todo!();
    }

    fn match_tokentype(&mut self, types: &[TokenType]) -> bool {
        for typ in types {
            if self.check(*typ) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, typ: TokenType, message: &str) -> Result<&Token, LoxError> {
        if self.check(typ) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(), message))
    }

    fn check(&self, typ: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().typ == typ
    }

    fn advance(&mut self) -> &Token {
        if self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn error(&self, token: &Token, message: &str) -> LoxError {
        error::lox_error_token(token, message);
        LoxError::ParseError
    }

    fn synchronize(&mut self) {
        use TokenType::*;

        self.advance();

        while !self.is_at_end() {
            if self.previous().typ == Semicolon {
                return;
            }

            match self.peek().typ {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => (),
            }

            self.advance();
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().typ == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
}
