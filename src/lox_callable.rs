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
}

impl std::fmt::Display for CallableKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function(fun) => write!(f, "{}", fun),
            Self::Native(fun) => write!(f, "{}", fun),
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
            Self::Function(fun) => fun.call(interpreter, arguments),
            Self::Native(fun) => fun.call(interpreter, arguments),
        }
    }

    fn arity(&self) -> usize {
        match self {
            Self::Function(fun) => fun.arity(),
            Self::Native(fun) => fun.arity(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoxFunction {
    declaration: StmtFunction,
}

impl LoxFunction {
    fn new(declaration: StmtFunction) -> Self {
        Self { declaration }
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
        let mut environment = Environment::new(Some(interpreter.globals.clone()));

        for (param, obj) in self.declaration.params.iter().zip(arguments) {
            environment.define(param.lexeme.clone(), obj.clone());
        }

        interpreter.execute_block(&self.declaration.body, Rc::new(RefCell::new(environment)))?;

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
