use crate::{expr::Expr, token::Token};

pub trait StmtVisitor<R> {
    fn visit_expression_stmt(&self, stmt: &StmtExpression) -> R;
    fn visit_print_stmt(&self, stmt: &StmtPrint) -> R;
    fn visit_var_stmt(&self, stmt: &StmtVar) -> R;
}

#[derive(Debug)]
pub enum Stmt {
    Expression(StmtExpression),
    Print(StmtPrint),
    Var(StmtVar),
}

impl Stmt {
    pub fn accept<V, R>(&self, visitor: &V) -> R
    where
        V: StmtVisitor<R>,
    {
        match *self {
            Stmt::Expression(ref stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Print(ref stmt) => visitor.visit_print_stmt(stmt),
            Stmt::Var(ref stmt) => visitor.visit_var_stmt(stmt),
        }
    }

    pub fn expression(expression: Expr) -> Self {
        Self::Expression(StmtExpression { expression })
    }

    pub fn print(expression: Expr) -> Self {
        Self::Print(StmtPrint { expression })
    }

    pub fn var(name: Token, initializer: Option<Expr>) -> Self {
        Self::Var(StmtVar { name, initializer })
    }
}

#[derive(Debug)]
pub struct StmtExpression {
    pub expression: Expr,
}

#[derive(Debug)]
pub struct StmtPrint {
    pub expression: Expr,
}

#[derive(Debug)]
pub struct StmtVar {
    pub name: Token,
    pub initializer: Option<Expr>,
}
