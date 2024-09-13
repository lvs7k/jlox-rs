use crate::{object::Object, token_type::TokenType};

pub(crate) struct Token {
    pub(crate) typ: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Object,
    pub(crate) line: usize,
}

impl Token {
    pub(crate) fn new(typ: TokenType, lexeme: String, literal: Object, line: usize) -> Self {
        Self {
            typ,
            lexeme,
            literal,
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}", self.typ, self.lexeme, self.literal)
    }
}
