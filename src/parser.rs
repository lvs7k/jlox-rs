use crate::{
    error::{self, LoxError},
    expr::Expr,
    object::Object,
    stmt::Stmt,
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(Some(stmt)) => statements.push(stmt),
                Ok(None) => (), // synchronize
                Err(e) => return Err(e),
            }
        }

        Ok(statements)
    }

    // before 8.1.2 Parsing statements
    pub(crate) fn parse_one_expr(&mut self) -> Result<Expr, LoxError> {
        match self.expression() {
            Ok(expr) => Ok(expr),
            Err(err) => Err(err),
        }
    }

    fn declaration(&mut self) -> Result<Option<Stmt>, LoxError> {
        let res = if self.match_tokentype(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        match res {
            Ok(stmt) => Ok(Some(stmt)),
            Err(LoxError::ParseError) => {
                self.synchronize();
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.match_tokentype(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_tokentype(&[TokenType::LeftBrace]) {
            let statements = self.block()?;
            return Ok(Stmt::block(statements));
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::print(value))
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")?
            .clone();

        let mut initializer = None;
        if self.match_tokentype(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::var(name, initializer))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::expression(expr))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements = vec![];

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let stmt = self.declaration()?;
            if let Some(stmt) = stmt {
                statements.push(stmt);
            }
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.equality()?;

        if self.match_tokentype(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable(var) = expr {
                let name = var.name;
                return Ok(Expr::assign(name, value));
            }

            error::lox_error_token(&equals, "Invalid assignment target.");
            // 8.4.1 Assignment syntax
            // [!WARNING] Original implementation doesn't throw error here.
            return Err(LoxError::ParseError);
        }

        Ok(expr)
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
            let right = self.unary()?;
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
            return Ok(Expr::literal(Object::Null));
        }

        if self.match_tokentype(&[Number, String]) {
            return Ok(Expr::literal(self.previous().clone().literal));
        }

        if self.match_tokentype(&[Identifier]) {
            let name = self.previous().clone();
            return Ok(Expr::variable(name));
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
        if !self.is_at_end() {
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

#[cfg(test)]
mod test {
    use crate::scanner::Scanner;

    use super::*;

    fn parse_source(source: &str) -> Result<Expr, LoxError> {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        parser.parse_one_expr()
    }

    #[test]
    fn parse_primary() {
        assert_eq!(
            parse_source("true").unwrap(),
            Expr::literal(Object::Bool(true))
        );
        assert_eq!(
            parse_source("false").unwrap(),
            Expr::literal(Object::Bool(false))
        );
        assert_eq!(parse_source("nil").unwrap(), Expr::literal(Object::Null));
        assert_eq!(
            parse_source("123.456").unwrap(),
            Expr::literal(Object::Num(123.456))
        );
        assert_eq!(
            parse_source("\"hello, world\"").unwrap(),
            Expr::literal(Object::Str("hello, world".to_string()))
        );
        assert_eq!(
            parse_source("(1 + 2) * 3").unwrap(),
            Expr::binary(
                Expr::grouping(Expr::binary(
                    Expr::literal(Object::Num(1f64)),
                    Token::new(TokenType::Plus, "+".into(), Object::Null, 1),
                    Expr::literal(Object::Num(2f64))
                )),
                Token::new(TokenType::Star, "*".into(), Object::Null, 1),
                Expr::literal(Object::Num(3f64))
            )
        );
    }

    #[test]
    fn parse_unary() {
        assert_eq!(
            parse_source("-123.456").unwrap(),
            Expr::unary(
                Token::new(TokenType::Minus, "-".into(), Object::Null, 1),
                Expr::literal(Object::Num(123.456))
            )
        );
        assert_eq!(
            parse_source("!false").unwrap(),
            Expr::unary(
                Token::new(TokenType::Bang, "!".into(), Object::Null, 1),
                Expr::literal(Object::Bool(false))
            )
        );
        assert_eq!(
            parse_source("!!true").unwrap(),
            Expr::unary(
                Token::new(TokenType::Bang, "!".into(), Object::Null, 1),
                Expr::unary(
                    Token::new(TokenType::Bang, "!".into(), Object::Null, 1),
                    Expr::literal(Object::Bool(true))
                )
            )
        );
    }

    #[test]
    fn parse_factor() {
        assert_eq!(
            parse_source("123 * 456 / 789").unwrap(),
            Expr::binary(
                Expr::binary(
                    Expr::literal(Object::Num(123f64)),
                    Token::new(TokenType::Star, "*".into(), Object::Null, 1),
                    Expr::literal(Object::Num(456f64)),
                ),
                Token::new(TokenType::Slash, "/".into(), Object::Null, 1),
                Expr::literal(Object::Num(789f64))
            )
        )
    }

    #[test]
    fn parse_term() {
        assert_eq!(
            parse_source("123 + 456 - 789").unwrap(),
            Expr::binary(
                Expr::binary(
                    Expr::literal(Object::Num(123f64)),
                    Token::new(TokenType::Plus, "+".into(), Object::Null, 1),
                    Expr::literal(Object::Num(456f64)),
                ),
                Token::new(TokenType::Minus, "-".into(), Object::Null, 1),
                Expr::literal(Object::Num(789f64))
            )
        )
    }

    #[test]
    fn parse_comparison() {
        assert_eq!(
            parse_source("123 >= 456 < 789").unwrap(),
            Expr::binary(
                Expr::binary(
                    Expr::literal(Object::Num(123f64)),
                    Token::new(TokenType::GreaterEqual, ">=".into(), Object::Null, 1),
                    Expr::literal(Object::Num(456f64)),
                ),
                Token::new(TokenType::Less, "<".into(), Object::Null, 1),
                Expr::literal(Object::Num(789f64))
            )
        )
    }

    #[test]
    fn parse_equality() {
        assert_eq!(
            parse_source("123 != 456 == 789").unwrap(),
            Expr::binary(
                Expr::binary(
                    Expr::literal(Object::Num(123f64)),
                    Token::new(TokenType::BangEqual, "!=".into(), Object::Null, 1),
                    Expr::literal(Object::Num(456f64)),
                ),
                Token::new(TokenType::EqualEqual, "==".into(), Object::Null, 1),
                Expr::literal(Object::Num(789f64))
            )
        )
    }
}
