#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Bool(bool),
    Num(f64),
    Str(String),
    Null,
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(v) => write!(f, "{}", v),
            Self::Num(v) => write!(f, "{}", v),
            Self::Str(v) => write!(f, "{}", v),
            Self::Null => write!(f, "nil"),
        }
    }
}
