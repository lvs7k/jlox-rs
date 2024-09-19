use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::LoxError, object::Object, token::Token};

#[derive(Debug)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }

        if let Some(enclosing) = &self.enclosing {
            // [!NOTE] enclosing.as_ref() == (&**enclosing)
            return enclosing.as_ref().borrow().get(name);
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

        if let Some(enclosing) = &self.enclosing {
            enclosing.as_ref().borrow_mut().assign(name, value)?;
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
