use std::collections::HashMap;

use crate::{error::LoxError, object::Object, token::Token};

#[derive(Debug)]
pub struct Environment<'a> {
    enclosing: Option<&'a mut Environment<'a>>,
    values: HashMap<String, Object>,
}

impl<'a> Environment<'a> {
    pub fn new(enclosing: Option<&'a mut Environment<'a>>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<&Object, LoxError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value);
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }

        Err(LoxError::RuntimeError(
            name.clone(),
            format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.to_string(), value);
            return Ok(());
        }

        if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value)?;
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
