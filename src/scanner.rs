use crate::{
    error::{lox_error, LoxError},
    object::Object,
    token::Token,
    token_type::TokenType,
};

#[derive(Debug)]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let chars = source.chars().collect();
        Self {
            source: chars,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, LoxError> {
        let mut had_error = false;

        while self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            if self.scan_token().is_err() {
                had_error = true;
            }
        }

        if had_error {
            return Err(LoxError::ScanError);
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            "".into(),
            Object::Nil,
            self.line,
        ));

        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        use Object::Nil;
        use TokenType::*;

        let c = self.advance();
        match c {
            '(' => self.add_token(LeftParen, Nil),
            ')' => self.add_token(RightParen, Nil),
            '{' => self.add_token(LeftBrace, Nil),
            '}' => self.add_token(RightBrace, Nil),
            ',' => self.add_token(Comma, Nil),
            '.' => self.add_token(Dot, Nil),
            '-' => self.add_token(Minus, Nil),
            '+' => self.add_token(Plus, Nil),
            ';' => self.add_token(Semicolon, Nil),
            '*' => self.add_token(Star, Nil),
            '!' => {
                let typ = if self.match_char('=') {
                    BangEqual
                } else {
                    Bang
                };
                self.add_token(typ, Nil);
            }
            '=' => {
                let typ = if self.match_char('=') {
                    EqualEqual
                } else {
                    Equal
                };
                self.add_token(typ, Nil);
            }
            '<' => {
                let typ = if self.match_char('=') {
                    LessEqual
                } else {
                    Less
                };
                self.add_token(typ, Nil);
            }
            '>' => {
                let typ = if self.match_char('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token(typ, Nil);
            }
            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash, Nil);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string()?,
            _ => {
                lox_error(self.line, "Unexpected character.");
                return Err(LoxError::ScanError);
            }
        }

        Ok(())
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            lox_error(self.line, "Unterminated string.");
            return Err(LoxError::ScanError);
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        let literal = Object::Str(value);
        self.add_token(TokenType::String, literal);

        Ok(())
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let char = self.source[self.current];
        self.current += 1;
        char
    }

    fn add_token(&mut self, typ: TokenType, literal: Object) {
        let text = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(typ, text, literal, self.line));
    }
}
