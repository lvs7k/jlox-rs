use std::io::{self, BufRead, Write};

use jlox_rs::run;

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

    if run(source).is_err() {
        std::process::exit(65);
    }

    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let mut buf;
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush()?;

        buf = String::new();
        stdin.read_line(&mut buf)?;

        if buf.is_empty() {
            break;
        }

        let _ = run(buf);
    }

    Ok(())
}
