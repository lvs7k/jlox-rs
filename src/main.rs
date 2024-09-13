use std::io::{self, BufRead, Write};

use jlox_rs::scanner::Scanner;

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
    let source = std::fs::read_to_string(path)?;
    run(source);
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let mut buf = String::new();
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush()?;

        buf.clear();
        stdin.read_line(&mut buf)?;

        if buf.is_empty() {
            break;
        }

        run(buf);
        buf = String::new();
    }

    Ok(())
}

fn run(source: String) {
    print!("source: {}", source);

    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    // For now, just print the tokens.
    for token in tokens {
        println!("token: {:?}", token);
    }
}
