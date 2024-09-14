#[derive(Debug)]
pub enum LoxError {
    ScanError,
}

pub fn lox_error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, where_: &str, message: &str) {
    eprintln!("[line {line}] Error {where_}: {message}");
}
