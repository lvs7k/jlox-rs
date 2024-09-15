use std::{collections::HashMap, sync::LazyLock};

use crate::{
    error::{lox_error_line, LoxError},
    object::Object,
    token::Token,
    token_type::TokenType,
};

static KEYWORDS: LazyLock<HashMap<String, TokenType>> = LazyLock::new(|| {
    use TokenType::*;

    let mut m = HashMap::new();
    m.insert("and".to_string(), And);
    m.insert("class".to_string(), Class);
    m.insert("else".to_string(), Else);
    m.insert("false".to_string(), False);
    m.insert("for".to_string(), For);
    m.insert("fun".to_string(), Fun);
    m.insert("if".to_string(), If);
    m.insert("nil".to_string(), Nil);
    m.insert("or".to_string(), Or);
    m.insert("print".to_string(), Print);
    m.insert("return".to_string(), Return);
    m.insert("super".to_string(), Super);
    m.insert("this".to_string(), This);
    m.insert("true".to_string(), True);
    m.insert("var".to_string(), Var);
    m.insert("while".to_string(), While);

    m
});

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

        while !self.is_at_end() {
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
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    lox_error_line(self.line, "Unexpected character.");
                    return Err(LoxError::ScanError);
                }
            }
        }

        Ok(())
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        if let Some(typ) = KEYWORDS.get(&text) {
            self.add_token(*typ, Object::Nil);
        } else {
            self.add_token(TokenType::Identifier, Object::Nil);
        }
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let str: String = self.source[self.start..self.current].iter().collect();
        let value = str.parse().unwrap();

        self.add_token(TokenType::Number, Object::Num(value));
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            lox_error_line(self.line, "Unterminated string.");
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

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scan_tokens_succeed_for_correct_if_expression() {
        use TokenType::*;

        let source = "if true { id_a + 123.456 } else { \"hello\" != 789 }";
        let scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens().unwrap();

        let answers = vec![
            Token::new(If, "if".into(), Object::Nil, 1),
            Token::new(True, "true".into(), Object::Nil, 1),
            Token::new(LeftBrace, "{".into(), Object::Nil, 1),
            Token::new(Identifier, "id_a".into(), Object::Nil, 1),
            Token::new(Plus, "+".into(), Object::Nil, 1),
            Token::new(Number, "123.456".into(), Object::Num(123.456), 1),
            Token::new(RightBrace, "}".into(), Object::Nil, 1),
            Token::new(Else, "else".into(), Object::Nil, 1),
            Token::new(LeftBrace, "{".into(), Object::Nil, 1),
            Token::new(String, "\"hello\"".into(), Object::Str("hello".into()), 1),
            Token::new(BangEqual, "!=".into(), Object::Nil, 1),
            Token::new(Number, "789".into(), Object::Num(789f64), 1),
            Token::new(RightBrace, "}".into(), Object::Nil, 1),
            Token::new(Eof, "".into(), Object::Nil, 1),
        ];

        assert_eq!(tokens, answers);
        for (token, ref answer) in tokens.iter().zip(answers) {
            assert_eq!(token, answer);
        }
    }

    #[test]
    fn scan_tokens_succeed_for_multiple_line_of_code() {
        use TokenType::*;

        let source = "// This is comment.\n123\n+\n456";
        let scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens().unwrap();

        let answers = vec![
            Token::new(Number, "123".into(), Object::Num(123f64), 2),
            Token::new(Plus, "+".into(), Object::Nil, 3),
            Token::new(Number, "456".into(), Object::Num(456f64), 4),
            Token::new(Eof, "".into(), Object::Nil, 4),
        ];

        assert_eq!(tokens, answers);
        for (token, ref answer) in tokens.iter().zip(answers) {
            assert_eq!(token, answer);
        }
    }

    #[test]
    #[should_panic]
    fn scan_tokens_failed_for_non_terminated_string() {
        let source = "  \"hello ";
        let scanner = Scanner::new(source.to_string());
        let _tokens = scanner.scan_tokens().unwrap(); // should panic
    }
}
