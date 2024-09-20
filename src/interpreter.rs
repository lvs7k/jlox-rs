use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment,
    error::{self, LoxError},
    expr::*,
    object::Object,
    stmt::*,
    token::Token,
    token_type::TokenType,
};

#[derive(Debug)]
pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new(None))),
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

    fn evaluate<E>(&mut self, expr: E) -> Result<Object, LoxError>
    where
        E: std::ops::Deref<Target = Expr>,
    {
        expr.new_accept(self)
    }

    fn execute<S>(&mut self, stmt: S) -> Result<(), LoxError>
    where
        S: std::ops::Deref<Target = Stmt>,
    {
        stmt.accept(self)
    }

    fn execute_block<S>(
        &mut self,
        statements: S,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), LoxError>
    where
        S: std::ops::Deref<Target = Vec<Stmt>>,
    {
        let previous = self.environment.clone();
        self.environment = environment;

        for statement in &*statements {
            if let Err(e) = self.execute(statement) {
                self.environment = previous;
                return Err(e);
            }
        }
        self.environment = previous;

        Ok(())
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
        let right = self.evaluate(&*expr.right)?;

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
        let left = self.evaluate(&*expr.left)?;
        let right = self.evaluate(&*expr.right)?;

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
        self.evaluate(&*expr.expression)
    }

    fn visit_variable_expr(&mut self, expr: &ExprVariable) -> Result<Object, LoxError> {
        let value = self.environment.as_ref().borrow().get(&expr.name)?;
        Ok(value.clone())
    }

    fn visit_assign_expr(&mut self, expr: &ExprAssign) -> Result<Object, LoxError> {
        let value = self.evaluate(&*expr.value)?;
        self.environment
            .as_ref()
            .borrow_mut()
            .assign(&expr.name, value.clone())?;
        Ok(value)
    }

    fn visit_logical_expr(&mut self, expr: &ExprLogical) -> Result<Object, LoxError> {
        let left = self.evaluate(&*expr.left)?;

        if expr.operator.typ == TokenType::Or {
            if left.is_truthy() {
                return Ok(left);
            }
        } else if !left.is_truthy() {
            return Ok(left);
        }

        self.evaluate(&*expr.right)
    }

    fn visit_call_expr(&mut self, expr: &ExprCall) -> Result<Object, LoxError> {
        todo!();
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
            self.execute(&*stmt.then_branch)?;
        } else if let Some(ref else_branch) = stmt.else_branch {
            self.execute(&**else_branch)?;
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &StmtWhile) -> Result<(), LoxError> {
        while self.evaluate(&stmt.condition)?.is_truthy() {
            self.execute(&*stmt.body)?;
        }

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
