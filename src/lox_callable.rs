use crate::{interpreter::Interpreter, object::Object};

pub trait LoxCallable {
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Object]) -> Object;
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
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Object]) -> Object {
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
pub struct LoxFunction {}

impl std::fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<lox fn>")
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Object]) -> Object {
        todo!();
    }

    fn arity(&self) -> usize {
        todo!();
    }
}

#[derive(Debug, Clone)]
pub struct NativeFunction {
    pointer: fn(&mut Interpreter, &[Object]) -> Object,
    arity: usize,
}

impl NativeFunction {
    pub fn new(pointer: fn(&mut Interpreter, &[Object]) -> Object, arity: usize) -> Self {
        Self { pointer, arity }
    }
}

impl std::fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}

impl LoxCallable for NativeFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Object]) -> Object {
        (self.pointer)(interpreter, arguments)
    }

    fn arity(&self) -> usize {
        self.arity
    }
}
