pub mod error;
pub mod parser;
pub mod scanner;

mod ast_printer;
mod expr;
mod object;
mod token;
mod token_type;

use error::LoxError;
use scanner::Scanner;

pub fn run(source: String) -> Result<(), LoxError> {
    print!("source: {}", source);

    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    // For now, just print the tokens.
    for token in tokens {
        println!("token: {:?}", token);
    }

    Ok(())
}
