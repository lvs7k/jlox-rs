use crate::{token::Token, token_type::TokenType};

#[derive(Debug)]
pub enum LoxError {
    ScanError,
    ParseError,
    RuntimeError(Token, String),
}

pub fn lox_error_line(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, where_: &str, message: &str) {
    eprintln!("[line {line}] Error{where_}: {message}");
}

pub fn lox_error_token(token: &Token, message: &str) {
    if token.typ == TokenType::Eof {
        report(token.line, " at end", message);
    } else {
        report(token.line, &format!(" at '{}'", token.lexeme), message);
    }
}

pub fn lox_runtime_error(token: &Token, message: &str) {
    eprintln!("{}\n[line {}]", message, token.line);
}
