use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    environment::Environment, error::LoxError, interpreter::Interpreter, object::Object, stmt::*,
    token::Token,
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
    is_initializer: bool,
}

impl LoxFunction {
    pub fn new(
        declaration: StmtFunction,
        closure: Rc<RefCell<Environment>>,
        is_initializer: bool,
    ) -> Self {
        Self {
            declaration,
            closure,
            is_initializer,
        }
    }

    pub fn bind(self, instance: LoxInstance) -> LoxFunction {
        let mut environment = Environment::new(Some(self.closure.clone()));

        environment.define("this".to_string(), Object::Instance(instance));

        LoxFunction::new(
            self.declaration,
            Rc::new(RefCell::new(environment)),
            self.is_initializer,
        )
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
            if self.is_initializer {
                let this = self.closure.as_ref().borrow_mut().get_at(0, "this");
                return Ok(this);
            }

            return Ok(return_value);
        }

        if self.is_initializer {
            return Ok(self.closure.as_ref().borrow().get_at(0, "this"));
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
    name: Rc<String>,
    superclass: Option<Rc<LoxClass>>,
    methods: Rc<RefCell<HashMap<String, LoxFunction>>>,
}

impl LoxClass {
    pub fn new(
        name: String,
        superclass: Option<Rc<LoxClass>>,
        methods: HashMap<String, LoxFunction>,
    ) -> Self {
        Self {
            name: Rc::new(name),
            superclass,
            methods: Rc::new(RefCell::new(methods)),
        }
    }

    pub fn find_method(&self, name: &str) -> Option<LoxFunction> {
        if let Some(function) = self.methods.as_ref().borrow().get(name) {
            return Some(function.clone());
        }

        if let Some(ref superclass) = self.superclass {
            return superclass.find_method(name);
        }

        None
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
        let instance = LoxInstance::new(self.clone());

        if let Some(initializer) = self.find_method("init") {
            initializer
                .bind(instance.clone())
                .call(interpreter, arguments)?;
        }

        Ok(Object::Instance(instance))
    }

    fn arity(&self) -> usize {
        if let Some(initializer) = self.find_method("init") {
            return initializer.arity();
        }

        0
    }
}

// LoxInstance is not CallableKind, but we will place it here.
#[derive(Debug, Clone)]
pub struct LoxInstance {
    klass: LoxClass,
    fields: Rc<RefCell<HashMap<String, Object>>>,
}

impl LoxInstance {
    pub fn new(klass: LoxClass) -> Self {
        Self {
            klass,
            fields: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxError> {
        if let Some(field) = self.fields.as_ref().borrow().get(&name.lexeme) {
            return Ok(field.clone());
        }

        if let Some(method) = self.klass.find_method(&name.lexeme) {
            let function = method.bind(self.clone());
            return Ok(Object::Callable(CallableKind::Function(function)));
        }

        Err(LoxError::RuntimeError(
            name.clone(),
            format!("Undefined property '{}'.", name.lexeme),
        ))
    }

    pub fn set(&mut self, name: Token, value: Object) {
        self.fields.borrow_mut().insert(name.lexeme, value);
    }
}

impl std::fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", &self.klass.name)
    }
}
