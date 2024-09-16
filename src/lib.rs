pub mod error;
pub mod interpreter;
pub mod parser;
pub mod scanner;

mod ast_printer;
mod expr;
mod object;
mod token;
mod token_type;

use error::LoxError;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

pub fn run(source: &str, interpreter: &mut Interpreter) -> Result<(), LoxError> {
    print!("source: {}", source);

    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let expression = parser.parse()?;

    interpreter.interpret(&expression)?;

    Ok(())
}
