#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Object {
    Bool(bool),
    Num(f64),
    Str(String),
    Nil,
}
