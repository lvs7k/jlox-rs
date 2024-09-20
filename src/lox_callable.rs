use crate::{interpreter::Interpreter, object::Object};

pub trait LoxCallable: std::fmt::Debug + std::clone::Clone {
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Object]) -> Object;
}

#[derive(Debug, Clone)]
pub enum CallableKind {}
