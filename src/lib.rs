pub mod error;
pub mod interpreter;
pub mod parser;
pub mod scanner;

mod ast_printer;
mod expr;
mod object;
mod token;
mod token_type;

use ast_printer::AstPrinter;
use error::LoxError;
use parser::Parser;
use scanner::Scanner;

pub fn run(source: &str) -> Result<(), LoxError> {
    print!("source: {}", source);

    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let expression = parser.parse()?;

    if let Some(expression) = expression {
        println!("{}", AstPrinter.print(&expression));
    }

    Ok(())
}
