pub mod error;
pub mod interpreter;
pub mod parser;
pub mod resolver;
pub mod scanner;

mod ast_printer;
mod environment;
mod expr;
mod lox_callable;
mod object;
mod stmt;
mod token;
mod token_type;

use error::LoxError;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

pub fn run(source: &str, interpreter: &mut Interpreter) -> Result<(), LoxError> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    interpreter.interpret(&statements)?;

    Ok(())
}
