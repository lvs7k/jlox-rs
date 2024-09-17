use crate::{
    error::{self, LoxError},
    expr::{Expr, ExprBinary, ExprGrouping, ExprLiteral, ExprUnary, ExprVisitor},
    object::Object,
    stmt::{Stmt, StmtExpression, StmtPrint, StmtVar, StmtVisitor},
    token::Token,
    token_type::TokenType,
};

#[derive(Debug)]
pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&self, statements: &Vec<Stmt>) -> Result<(), LoxError> {
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

    fn evaluate<E>(&self, expr: E) -> Result<Object, LoxError>
    where
        E: std::ops::Deref<Target = Expr>,
    {
        expr.accept(self)
    }

    fn execute<S>(&self, stmt: S) -> Result<(), LoxError>
    where
        S: std::ops::Deref<Target = Stmt>,
    {
        stmt.accept(self)
    }
}

impl ExprVisitor<Result<Object, LoxError>> for Interpreter {
    fn visit_literal_expr(&self, expr: &ExprLiteral) -> Result<Object, LoxError> {
        Ok(expr.value.clone())
    }

    fn visit_unary_expr(&self, expr: &ExprUnary) -> Result<Object, LoxError> {
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

    fn visit_binary_expr(&self, expr: &ExprBinary) -> Result<Object, LoxError> {
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

    fn visit_grouping_expr(&self, expr: &ExprGrouping) -> Result<Object, LoxError> {
        self.evaluate(&*expr.expression)
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
    fn visit_expression_stmt(&self, stmt: &StmtExpression) -> Result<(), LoxError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &StmtPrint) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &StmtVar) -> Result<(), LoxError> {
        todo!();
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
