use crate::{object::Object, token::Token, token_type::TokenType};

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

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            "".into(),
            Object::Nil,
            self.line,
        ));
        self.tokens
    }

    fn scan_token(&mut self) {
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
            _ => (),
        }
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
