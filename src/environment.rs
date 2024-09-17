use std::collections::HashMap;

use crate::{error::LoxError, object::Object, token::Token};

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<&Object, LoxError> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value),
            None => Err(LoxError::RuntimeError(
                name.clone(),
                format!("Undefined variable '{}'.", name.lexeme),
            )),
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.to_string(), value);
            return Ok(());
        }

        Err(LoxError::RuntimeError(
            name.clone(),
            format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }
}
