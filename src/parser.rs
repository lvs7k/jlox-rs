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

    pub fn parse(&mut self) -> Result<Option<Expr>, LoxError> {
        match self.expression() {
            Ok(expr) => Ok(Some(expr)),
            Err(LoxError::ParseError) => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        use TokenType::*;

        let mut expr = self.comparison()?;

        while self.match_tokentype(&[BangEqual, EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        use TokenType::*;

        let mut expr = self.term()?;

        while self.match_tokentype(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        use TokenType::*;

        let mut expr = self.factor()?;

        while self.match_tokentype(&[Minus, Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        use TokenType::*;

        let mut expr = self.unary()?;

        while self.match_tokentype(&[Slash, Star]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        use TokenType::*;

        if self.match_tokentype(&[Bang, Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::unary(operator, right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        use TokenType::*;

        if self.match_tokentype(&[False]) {
            return Ok(Expr::literal(Object::Bool(false)));
        }
        if self.match_tokentype(&[True]) {
            return Ok(Expr::literal(Object::Bool(true)));
        }
        if self.match_tokentype(&[Nil]) {
            return Ok(Expr::literal(Object::Nil));
        }

        if self.match_tokentype(&[Number, String]) {
            return Ok(Expr::literal(self.previous().clone().literal));
        }

        if self.match_tokentype(&[LeftParen]) {
            let expr = self.expression()?;
            self.consume(RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::grouping(expr));
        }

        Err(self.error(self.peek(), "Expect expression."))
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
