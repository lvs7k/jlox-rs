use crate::{interpreter::Interpreter, object::Object};

pub trait LoxCallable {
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Object]) -> Object;
    fn arity(&self) -> usize;
}

#[derive(Debug, Clone)]
pub enum CallableKind {}

impl LoxCallable for CallableKind {
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Object]) -> Object {
        todo!();
    }

    fn arity(&self) -> usize {
        todo!();
    }
}
