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
    current_function: FunctionType,
    had_error: bool,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: vec![],
            current_function: FunctionType::None,
            had_error: false,
        }
    }

    pub fn resolve(&mut self, statements: &[Stmt]) -> Result<(), LoxError> {
        self.had_error = false;

        self.resolve_stmts(statements);

        if self.had_error {
            return Err(LoxError::ResolveError);
        }

        Ok(())
    }

    fn resolve_stmts(&mut self, statements: &[Stmt]) {
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

        if scope.contains_key(&name.lexeme) {
            error::lox_error_token(name, "Already a variable with this name in this scope.");
            self.had_error = true;
        }

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

    fn resolve_function(&mut self, function: &StmtFunction, ftype: FunctionType) {
        let enclosing_function = self.current_function;
        self.current_function = ftype;

        self.begin_scope();

        for param in &function.params {
            self.declare(param);
            self.define(param);
        }

        self.resolve_stmts(&function.body);
        self.end_scope();

        self.current_function = enclosing_function;
    }
}

impl<'a> StmtVisitor<()> for Resolver<'a> {
    fn visit_expression_stmt(&mut self, stmt: &StmtExpression) {
        self.resolve_expr(&stmt.expression);
    }

    fn visit_print_stmt(&mut self, stmt: &StmtPrint) {
        self.resolve_expr(&stmt.expression);
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
        self.resolve_stmts(&stmt.statements);
        self.end_scope();
    }

    fn visit_if_stmt(&mut self, stmt: &StmtIf) {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.then_branch);
        if let Some(ref else_branch) = stmt.else_branch {
            self.resolve_stmt(else_branch);
        }
    }

    fn visit_while_stmt(&mut self, stmt: &StmtWhile) {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.body);
    }

    fn visit_function_stmt(&mut self, stmt: &StmtFunction) {
        self.declare(&stmt.name);
        self.define(&stmt.name);

        self.resolve_function(stmt, FunctionType::Function);
    }

    fn visit_return_stmt(&mut self, stmt: &StmtReturn) {
        if self.current_function == FunctionType::None {
            error::lox_error_token(&stmt.keyword, "Can't return from top-level code.");
            self.had_error = true;
        }

        if let Some(ref value) = stmt.value {
            self.resolve_expr(value);
        }
    }

    fn visit_class_stmt(&mut self, stmt: &StmtClass) {
        self.declare(&stmt.name);
        self.define(&stmt.name);
    }
}

impl<'a> ExprVisitor<()> for Resolver<'a> {
    fn visit_literal_expr(&mut self, _expr: &ExprLiteral) {}

    fn visit_unary_expr(&mut self, expr: &ExprUnary) {
        self.resolve_expr(&expr.right);
    }

    fn visit_binary_expr(&mut self, expr: &ExprBinary) {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
    }

    fn visit_grouping_expr(&mut self, expr: &ExprGrouping) {
        self.resolve_expr(&expr.expression);
    }

    fn visit_variable_expr(&mut self, expr: &ExprVariable) {
        if !self.scopes.is_empty()
            && matches!(
                self.scopes.last_mut().unwrap().get(&expr.name.lexeme),
                Some(&false)
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
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
    }

    fn visit_call_expr(&mut self, expr: &ExprCall) {
        self.resolve_expr(&expr.callee);

        for argument in &expr.arguments {
            self.resolve_expr(argument);
        }
    }

    fn visit_get_expr(&mut self, expr: &ExprGet) {
        self.resolve_expr(&expr.object);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FunctionType {
    None,
    Function,
}
