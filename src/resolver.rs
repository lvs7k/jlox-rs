use std::collections::HashMap;

use crate::{
    error::{self, LoxError},
    expr::*,
    interpreter::Interpreter,
    stmt::*,
    token::Token,
};

#[derive(Debug)]
pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    had_error: bool,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: vec![],
            had_error: false,
        }
    }

    pub fn resolve(&mut self, statements: &[Stmt]) {
        self.had_error = false;

        for statement in statements {
            self.resolve_stmt(statement);
        }
    }

    fn resolve_stmt(&mut self, statement: &Stmt) {
        statement.accept(self)
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        expr.accept(self)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.scopes.last_mut().unwrap();
        scope.insert(name.lexeme.to_string(), false);
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.lexeme.to_string(), true);
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for (i, map) in self.scopes.iter().rev().enumerate() {
            if map.contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, i);
                return;
            }
        }
    }

    fn resolve_function(&mut self, function: &StmtFunction) {
        self.begin_scope();

        for param in &function.params {
            self.declare(param);
            self.define(param);
        }

        self.resolve(&function.body);
        self.end_scope();
    }
}

impl<'a> StmtVisitor<()> for Resolver<'a> {
    fn visit_expression_stmt(&mut self, stmt: &StmtExpression) {
        todo!();
    }

    fn visit_print_stmt(&mut self, stmt: &StmtPrint) {
        todo!();
    }

    fn visit_var_stmt(&mut self, stmt: &StmtVar) {
        self.declare(&stmt.name);

        if let Some(ref initializer) = stmt.initializer {
            self.resolve_expr(initializer);
        }

        self.define(&stmt.name);
    }

    fn visit_block_stmt(&mut self, stmt: &StmtBlock) {
        self.begin_scope();
        self.resolve(&stmt.statements);
        self.end_scope();
    }

    fn visit_if_stmt(&mut self, stmt: &StmtIf) {
        todo!();
    }

    fn visit_while_stmt(&mut self, stmt: &StmtWhile) {
        todo!();
    }

    fn visit_function_stmt(&mut self, stmt: &StmtFunction) {
        self.declare(&stmt.name);
        self.define(&stmt.name);

        self.resolve_function(stmt);
    }

    fn visit_return_stmt(&mut self, stmt: &StmtReturn) {
        todo!();
    }
}

impl<'a> ExprVisitor<()> for Resolver<'a> {
    fn visit_literal_expr(&mut self, expr: &ExprLiteral) {
        todo!();
    }

    fn visit_unary_expr(&mut self, expr: &ExprUnary) {
        todo!();
    }

    fn visit_binary_expr(&mut self, expr: &ExprBinary) {
        todo!();
    }

    fn visit_grouping_expr(&mut self, expr: &ExprGrouping) {
        todo!();
    }

    fn visit_variable_expr(&mut self, expr: &ExprVariable) {
        if !self.scopes.is_empty()
            && matches!(
                self.scopes.last_mut().unwrap().get(&expr.name.lexeme),
                None | Some(false)
            )
        {
            error::lox_error_token(
                &expr.name,
                "Can't read local variable in its own initializer.",
            );
            self.had_error = true;
        }

        self.resolve_local(&Expr::Variable(expr.clone()), &expr.name);
    }

    fn visit_assign_expr(&mut self, expr: &ExprAssign) {
        self.resolve_expr(&expr.value);
        self.resolve_local(&Expr::Assign(expr.clone()), &expr.name);
    }

    fn visit_logical_expr(&mut self, expr: &ExprLogical) {
        todo!();
    }

    fn visit_call_expr(&mut self, expr: &ExprCall) {
        todo!();
    }
}
