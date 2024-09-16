use std::io::{self, BufRead, Write};

use jlox_rs::{self, error::LoxError, interpreter::Interpreter};

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);

    match args.len() {
        0 => run_prompt()?,
        1 => {
            let path = args.next().unwrap();
            run_file(&path)?;
        }
        _ => {
            println!("Usage: jlox [script]");
            std::process::exit(64);
        }
    }

    Ok(())
}

fn run_file(path: &str) -> io::Result<()> {
    use LoxError::*;

    let source = std::fs::read_to_string(path)?;
    let mut interpreter = Interpreter::new();

    match jlox_rs::run(&source, &mut interpreter) {
        Err(ScanError | ParseError) => std::process::exit(65),
        Err(RuntimeError(..)) => std::process::exit(70),
        _ => (),
    }

    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let mut buf;
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout();

    let mut interpreter = Interpreter::new();

    loop {
        print!("> ");
        stdout.flush()?;

        buf = String::new();
        stdin.read_line(&mut buf)?;

        if buf.is_empty() {
            break;
        }

        let _ = jlox_rs::run(&buf, &mut interpreter);
    }

    Ok(())
}
