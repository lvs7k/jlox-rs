use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment, error::LoxError, interpreter::Interpreter, object::Object, stmt::*,
};

pub trait LoxCallable {
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Object])
        -> Result<Object, LoxError>;
    fn arity(&self) -> usize;
}

#[derive(Debug, Clone)]
pub enum CallableKind {
    Function(LoxFunction),
    Native(NativeFunction),
    Class(LoxClass),
}

impl std::fmt::Display for CallableKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function(callable) => write!(f, "{}", callable),
            Self::Native(callable) => write!(f, "{}", callable),
            Self::Class(callable) => write!(f, "{}", callable),
        }
    }
}

impl LoxCallable for CallableKind {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Object],
    ) -> Result<Object, LoxError> {
        match self {
            Self::Function(callable) => callable.call(interpreter, arguments),
            Self::Native(callable) => callable.call(interpreter, arguments),
            Self::Class(callable) => callable.call(interpreter, arguments),
        }
    }

    fn arity(&self) -> usize {
        match self {
            Self::Function(callable) => callable.arity(),
            Self::Native(callable) => callable.arity(),
            Self::Class(callable) => callable.arity(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoxFunction {
    declaration: StmtFunction,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn new(declaration: StmtFunction, closure: Rc<RefCell<Environment>>) -> Self {
        Self {
            declaration,
            closure,
        }
    }
}

impl std::fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.declaration.name.lexeme)
    }
}

impl LoxCallable for LoxFunction {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Object],
    ) -> Result<Object, LoxError> {
        let mut environment = Environment::new(Some(self.closure.clone()));

        for (param, obj) in self.declaration.params.iter().zip(arguments) {
            environment.define(param.lexeme.clone(), obj.clone());
        }

        if let Err(LoxError::Return(return_value)) =
            interpreter.execute_block(&self.declaration.body, Rc::new(RefCell::new(environment)))
        {
            return Ok(return_value);
        }

        Ok(Object::Null)
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}

#[derive(Debug, Clone)]
pub struct NativeFunction {
    pointer: fn(&mut Interpreter, &[Object]) -> Result<Object, LoxError>,
    arity: usize,
}

impl NativeFunction {
    pub fn new(
        pointer: fn(&mut Interpreter, &[Object]) -> Result<Object, LoxError>,
        arity: usize,
    ) -> Self {
        Self { pointer, arity }
    }
}

impl std::fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}

impl LoxCallable for NativeFunction {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Object],
    ) -> Result<Object, LoxError> {
        (self.pointer)(interpreter, arguments)
    }

    fn arity(&self) -> usize {
        self.arity
    }
}

#[derive(Debug, Clone)]
pub struct LoxClass {
    name: String,
}

impl LoxClass {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl std::fmt::Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}

impl LoxCallable for LoxClass {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Object],
    ) -> Result<Object, LoxError> {
        todo!();
    }

    fn arity(&self) -> usize {
        0
    }
}
