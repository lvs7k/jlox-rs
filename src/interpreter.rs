use std::{cell::RefCell, collections::HashMap, rc::Rc, time::SystemTime};

use crate::{
    environment::Environment,
    error::{self, LoxError},
    expr::*,
    lox_callable::*,
    object::Object,
    stmt::*,
    token::Token,
    token_type::TokenType,
};

#[derive(Debug)]
pub struct Interpreter {
    globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
    locals: HashMap<Expr, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new(None);

        let fn_clock = {
            fn clock(
                _interpreter: &mut Interpreter,
                _arguments: &[Object],
            ) -> Result<Object, LoxError> {
                // the number of non-leap seconds since the start of 1970 UTC.
                let time = SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64();
                Ok(Object::Num(time))
            }
            NativeFunction::new(clock, 0)
        };

        globals.define(
            "clock".to_string(),
            Object::Callable(CallableKind::Native(fn_clock)),
        );

        let globals = Rc::new(RefCell::new(globals));
        Self {
            environment: globals.clone(),
            globals,
            locals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> Result<(), LoxError> {
        for statement in statements {
            match self.execute(statement) {
                Err(LoxError::RuntimeError(token, message)) => {
                    error::lox_runtime_error(&token, &message);
                    return Err(LoxError::RuntimeError(token, message));
                }
                Err(error) => return Err(error),
                _ => (),
            }
        }
        Ok(())
    }

    pub fn resolve(&mut self, expr: &Expr, depth: usize) {
        self.locals.insert(expr.clone(), depth);
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), LoxError> {
        let previous = self.environment.clone();
        self.environment = environment;

        for statement in statements {
            if let Err(e) = self.execute(statement) {
                self.environment = previous;
                return Err(e);
            }
        }
        self.environment = previous;

        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
        stmt.accept(self)
    }

    fn look_up_variable(&self, name: &Token, expr: &Expr) -> Result<Object, LoxError> {
        if let Some(distance) = self.locals.get(expr) {
            Ok(self
                .environment
                .as_ref()
                .borrow()
                .get_at(*distance, &name.lexeme))
        } else {
            self.globals.as_ref().borrow().get(name)
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter::new()
    }
}

impl ExprVisitor<Result<Object, LoxError>> for Interpreter {
    fn visit_literal_expr(&mut self, expr: &ExprLiteral) -> Result<Object, LoxError> {
        Ok(expr.value.clone())
    }

    fn visit_unary_expr(&mut self, expr: &ExprUnary) -> Result<Object, LoxError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.typ {
            TokenType::Bang => Ok(!right),
            TokenType::Minus => {
                check_number_operand(&expr.operator, &right)?;
                Ok(-right)
            }
            _ => unreachable!(),
        }
    }

    fn visit_binary_expr(&mut self, expr: &ExprBinary) -> Result<Object, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.typ {
            TokenType::Greater => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Object::Bool(left > right))
            }
            TokenType::GreaterEqual => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Object::Bool(left >= right))
            }
            TokenType::Less => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Object::Bool(left < right))
            }
            TokenType::LessEqual => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(Object::Bool(left <= right))
            }
            TokenType::BangEqual => Ok(Object::Bool(left != right)),
            TokenType::EqualEqual => Ok(Object::Bool(left == right)),
            TokenType::Minus => {
                check_number_operand(&expr.operator, &right)?;
                Ok(left - right)
            }
            TokenType::Plus => {
                if left.is_num() && right.is_num() {
                    return Ok(left + right);
                }
                if left.is_str() && right.is_str() {
                    return Ok(left + right);
                }
                Err(LoxError::RuntimeError(
                    expr.operator.clone(),
                    "Operands must be two numbers or two strings.".into(),
                ))
            }
            TokenType::Slash => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(left / right)
            }
            TokenType::Star => {
                check_number_operands(&expr.operator, &left, &right)?;
                Ok(left * right)
            }
            _ => unreachable!(),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &ExprGrouping) -> Result<Object, LoxError> {
        self.evaluate(&expr.expression)
    }

    fn visit_variable_expr(&mut self, expr: &ExprVariable) -> Result<Object, LoxError> {
        self.look_up_variable(&expr.name, &Expr::Variable(expr.clone()))
    }

    fn visit_assign_expr(&mut self, expr: &ExprAssign) -> Result<Object, LoxError> {
        let value = self.evaluate(&expr.value)?;

        if let Some(distance) = self.locals.get(&Expr::Assign(expr.clone())) {
            self.environment
                .as_ref()
                .borrow_mut()
                .assign_at(*distance, &expr.name, value.clone());
        } else {
            self.globals
                .as_ref()
                .borrow_mut()
                .assign(&expr.name, value.clone())?
        }

        Ok(value)
    }

    fn visit_logical_expr(&mut self, expr: &ExprLogical) -> Result<Object, LoxError> {
        let left = self.evaluate(&expr.left)?;

        if expr.operator.typ == TokenType::Or {
            if left.is_truthy() {
                return Ok(left);
            }
        } else if !left.is_truthy() {
            return Ok(left);
        }

        self.evaluate(&expr.right)
    }

    fn visit_call_expr(&mut self, expr: &ExprCall) -> Result<Object, LoxError> {
        let callee = self.evaluate(&expr.callee)?;

        let mut arguments = vec![];
        for argument in &expr.arguments {
            arguments.push(self.evaluate(argument)?);
        }

        let function = match callee {
            Object::Callable(f) => f,
            _ => {
                return Err(LoxError::RuntimeError(
                    expr.paren.clone(),
                    "Can only call functions and classes.".to_string(),
                ))
            }
        };

        if arguments.len() != function.arity() {
            return Err(LoxError::RuntimeError(
                expr.paren.clone(),
                format!(
                    "Expected {} arguments but got {}.",
                    function.arity(),
                    arguments.len()
                ),
            ));
        }

        function.call(self, &arguments)
    }

    fn visit_get_expr(&mut self, expr: &ExprGet) -> Result<Object, LoxError> {
        let object = self.evaluate(&expr.object)?;

        if let Object::Instance(instance) = object {
            let obj = instance.get(&expr.name)?;
            return Ok(obj.clone());
        }

        Err(LoxError::RuntimeError(
            expr.name.clone(),
            "Only instances have properties.".to_string(),
        ))
    }

    fn visit_set_expr(&mut self, expr: &ExprSet) -> Result<Object, LoxError> {
        let object = self.evaluate(&expr.object)?;

        if !matches!(object, Object::Instance(_)) {
            return Err(LoxError::RuntimeError(
                expr.name.clone(),
                "Only instances have fields.".to_string(),
            ));
        }

        let value = self.evaluate(&expr.value)?;

        if let Object::Instance(mut instance) = object {
            instance.set(expr.name.clone(), value.clone());
        }

        Ok(value)
    }

    fn visit_this_expr(&mut self, expr: &ExprThis) -> Result<Object, LoxError> {
        self.look_up_variable(&expr.keyword, &Expr::This(expr.clone()))
    }

    fn visit_super_expr(&mut self, expr: &ExprSuper) -> Result<Object, LoxError> {
        let distance = self.locals.get(&Expr::Super(expr.clone())).unwrap();

        let superclass = self
            .environment
            .as_ref()
            .borrow()
            .get_at(*distance, "super");

        let object = self
            .environment
            .as_ref()
            .borrow()
            .get_at(*distance - 1, "this");

        let method = if let Object::Callable(CallableKind::Class(ref lox_class)) = superclass {
            lox_class.find_method(&expr.method.lexeme)
        } else {
            panic!("'super' must be LoxClass.");
        };

        if let Some(method) = method {
            if let Object::Instance(instance) = object {
                return Ok(Object::Callable(CallableKind::Function(
                    method.bind(instance),
                )));
            } else {
                panic!("This object must be LoxInstance");
            }
        }

        Err(LoxError::RuntimeError(
            expr.method.clone(),
            format!("Undefined property '{}'.", expr.method.lexeme),
        ))
    }
}

fn check_number_operand(operator: &Token, operand: &Object) -> Result<(), LoxError> {
    if operand.is_num() {
        return Ok(());
    }

    Err(LoxError::RuntimeError(
        operator.clone(),
        "Operand must be a number.".into(),
    ))
}

fn check_number_operands(operator: &Token, left: &Object, right: &Object) -> Result<(), LoxError> {
    if left.is_num() && right.is_num() {
        return Ok(());
    }

    Err(LoxError::RuntimeError(
        operator.clone(),
        "Operands must be numbers.".into(),
    ))
}

impl StmtVisitor<Result<(), LoxError>> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &StmtExpression) -> Result<(), LoxError> {
        self.evaluate(&stmt.expression)?;

        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &StmtPrint) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);

        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &StmtVar) -> Result<(), LoxError> {
        let mut value = Object::Null;
        if let Some(ref initializer) = stmt.initializer {
            value = self.evaluate(initializer)?;
        }

        self.environment
            .as_ref()
            .borrow_mut()
            .define(stmt.name.lexeme.to_string(), value);

        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &StmtBlock) -> Result<(), LoxError> {
        let environment = Rc::new(RefCell::new(Environment::new(Some(
            self.environment.clone(),
        ))));
        self.execute_block(&stmt.statements, environment)?;

        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &StmtIf) -> Result<(), LoxError> {
        if self.evaluate(&stmt.condition)?.is_truthy() {
            self.execute(&stmt.then_branch)?;
        } else if let Some(ref else_branch) = stmt.else_branch {
            self.execute(else_branch)?;
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &StmtWhile) -> Result<(), LoxError> {
        while self.evaluate(&stmt.condition)?.is_truthy() {
            self.execute(&stmt.body)?;
        }

        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: &StmtFunction) -> Result<(), LoxError> {
        let function = LoxFunction::new(stmt.clone(), self.environment.clone(), false);
        self.environment.as_ref().borrow_mut().define(
            stmt.name.lexeme.clone(),
            Object::Callable(CallableKind::Function(function)),
        );

        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &StmtReturn) -> Result<(), LoxError> {
        let mut value = Object::Null;

        if let Some(ref expr) = stmt.value {
            value = self.evaluate(expr)?;
        }

        Err(LoxError::Return(value))
    }

    fn visit_class_stmt(&mut self, stmt: &StmtClass) -> Result<(), LoxError> {
        let mut superclass = Object::Null;

        if let Some(ref stmt_superclass) = stmt.superclass {
            superclass = self.evaluate(stmt_superclass)?;

            if !matches!(superclass, Object::Callable(CallableKind::Class(_))) {
                if let Some(Expr::Variable(ref expr_variable)) = stmt.superclass {
                    return Err(LoxError::RuntimeError(
                        expr_variable.name.clone(),
                        "Superclass must be a class.".to_string(),
                    ));
                }
            }
        }

        self.environment
            .as_ref()
            .borrow_mut()
            .define(stmt.name.lexeme.to_string(), Object::Null);

        if stmt.superclass.is_some() {
            self.environment = Rc::new(RefCell::new(Environment::new(Some(
                self.environment.clone(),
            ))));

            self.environment
                .as_ref()
                .borrow_mut()
                .define("super".to_string(), superclass.clone());
        }

        let mut methods = HashMap::<String, LoxFunction>::new();

        for method in &stmt.methods {
            let stmt_function = match method {
                Stmt::Function(stmt_function) => stmt_function,
                _ => panic!("StmtClass.methods must contain StmtFunction only."),
            };

            let function = LoxFunction::new(
                stmt_function.clone(),
                self.environment.clone(),
                stmt_function.name.lexeme == "init",
            );

            methods.insert(stmt_function.name.lexeme.to_string(), function);
        }

        let klass = {
            let superclass =
                if let Object::Callable(CallableKind::Class(lox_class)) = superclass.clone() {
                    Some(Rc::new(lox_class))
                } else {
                    None
                };

            LoxClass::new(stmt.name.lexeme.to_string(), superclass, methods)
        };

        if !superclass.is_null() {
            let temp = self
                .environment
                .as_ref()
                .borrow()
                .enclosing
                .clone()
                .unwrap();

            self.environment = temp;
        }

        self.environment
            .as_ref()
            .borrow_mut()
            .assign(&stmt.name, Object::Callable(CallableKind::Class(klass)))?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{parser::Parser, scanner::Scanner};

    use super::*;

    fn run(source: &str, interpreter: &mut Interpreter) -> Result<Object, LoxError> {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;

        let mut parser = Parser::new(tokens);
        let statements = parser.parse_one_expr()?;

        interpreter.evaluate(&statements)
    }

    #[test]
    fn interpret_unary_expr() {
        let mut interpreter = Interpreter::new();

        assert_eq!(run("!true", &mut interpreter), Ok(Object::Bool(false)));
        assert_eq!(run("!false", &mut interpreter), Ok(Object::Bool(true)));
        assert_eq!(run("!123", &mut interpreter), Ok(Object::Bool(false)));
        assert_eq!(run("!\"hello\"", &mut interpreter), Ok(Object::Bool(false)));
        assert_eq!(run("!nil", &mut interpreter), Ok(Object::Bool(true)));

        assert_eq!(run("-123", &mut interpreter), Ok(Object::Num(-123f64)));
        assert!(matches!(
            run("-true", &mut interpreter),
            Err(LoxError::RuntimeError(..))
        ));
        assert!(matches!(
            run("-\"hello\"", &mut interpreter),
            Err(LoxError::RuntimeError(..))
        ));
        assert!(matches!(
            run("-nil", &mut interpreter),
            Err(LoxError::RuntimeError(..))
        ));
    }

    #[test]
    fn interpret_binary_expr() {
        let mut interpreter = Interpreter::new();

        assert_eq!(run("1 <= 2", &mut interpreter), Ok(Object::Bool(true)));
        assert_eq!(run("1 <  2", &mut interpreter), Ok(Object::Bool(true)));
        assert_eq!(run("1 >= 2", &mut interpreter), Ok(Object::Bool(false)));
        assert_eq!(run("1 >  2", &mut interpreter), Ok(Object::Bool(false)));

        assert_eq!(run("1 != 2", &mut interpreter), Ok(Object::Bool(true)));
        assert_eq!(run("1 == 2", &mut interpreter), Ok(Object::Bool(false)));

        assert_eq!(run("4 + 2", &mut interpreter), Ok(Object::Num(6f64)));
        assert_eq!(run("4 - 2", &mut interpreter), Ok(Object::Num(2f64)));
        assert_eq!(run("4 * 2", &mut interpreter), Ok(Object::Num(8f64)));
        assert_eq!(run("4 / 2", &mut interpreter), Ok(Object::Num(2f64)));
    }

    #[test]
    fn interpret_grouping_expr() {
        let mut interpreter = Interpreter::new();

        assert_eq!(run("!(!true)", &mut interpreter), Ok(Object::Bool(true)));
        assert_eq!(run("(1 + 2) * 3", &mut interpreter), Ok(Object::Num(9f64)));
        assert_eq!(run("1 + (2 * 3)", &mut interpreter), Ok(Object::Num(7f64)));
        assert_eq!(
            run("(1 + 2) * (3 - 4)", &mut interpreter),
            Ok(Object::Num(-3f64))
        );
    }
}
