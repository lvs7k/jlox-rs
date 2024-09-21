use std::collections::HashMap;

use crate::{error::LoxError, expr::*, interpreter::Interpreter, stmt::*};

#[derive(Debug)]
pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: vec![],
        }
    }

    pub fn resolve(&mut self, statements: &[Stmt]) -> Result<(), LoxError> {
        for statement in statements {
            self.resolve_stmt(statement);
        }

        Ok(())
    }

    fn resolve_stmt(&mut self, statement: &Stmt) -> Result<(), LoxError> {
        statement.accept(self)
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), LoxError> {
        expr.accept(self)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }
}

impl<'a> StmtVisitor<Result<(), LoxError>> for Resolver<'a> {
    fn visit_expression_stmt(&mut self, stmt: &StmtExpression) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_print_stmt(&mut self, stmt: &StmtPrint) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_var_stmt(&mut self, stmt: &StmtVar) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_block_stmt(&mut self, stmt: &StmtBlock) -> Result<(), LoxError> {
        self.begin_scope();
        self.resolve(&stmt.statements);
        self.end_scope();

        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &StmtIf) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_while_stmt(&mut self, stmt: &StmtWhile) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_function_stmt(&mut self, stmt: &StmtFunction) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_return_stmt(&mut self, stmt: &StmtReturn) -> Result<(), LoxError> {
        todo!();
    }
}

impl<'a> ExprVisitor<Result<(), LoxError>> for Resolver<'a> {
    fn visit_literal_expr(&mut self, expr: &ExprLiteral) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_unary_expr(&mut self, expr: &ExprUnary) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_binary_expr(&mut self, expr: &ExprBinary) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_grouping_expr(&mut self, expr: &ExprGrouping) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_variable_expr(&mut self, expr: &ExprVariable) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_assign_expr(&mut self, expr: &ExprAssign) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_logical_expr(&mut self, expr: &ExprLogical) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_call_expr(&mut self, expr: &ExprCall) -> Result<(), LoxError> {
        todo!();
    }
}
