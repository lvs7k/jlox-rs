use crate::{expr::Expr, token::Token};

pub trait StmtVisitor<R> {
    fn visit_expression_stmt(&mut self, stmt: &StmtExpression) -> R;
    fn visit_print_stmt(&mut self, stmt: &StmtPrint) -> R;
    fn visit_var_stmt(&mut self, stmt: &StmtVar) -> R;
    fn visit_block_stmt(&mut self, stmt: &StmtBlock) -> R;
    fn visit_if_stmt(&mut self, stmt: &StmtIf) -> R;
    fn visit_while_stmt(&mut self, stmt: &StmtWhile) -> R;
    fn visit_function_stmt(&mut self, stmt: &StmtFunction) -> R;
}

#[derive(Debug)]
pub enum Stmt {
    Expression(StmtExpression),
    Print(StmtPrint),
    Var(StmtVar),
    Block(StmtBlock),
    If(StmtIf),
    While(StmtWhile),
    Function(StmtFunction),
}

impl Stmt {
    pub fn accept<V, R>(&self, visitor: &mut V) -> R
    where
        V: StmtVisitor<R>,
    {
        match self {
            Stmt::Expression(ref stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Print(ref stmt) => visitor.visit_print_stmt(stmt),
            Stmt::Var(ref stmt) => visitor.visit_var_stmt(stmt),
            Stmt::Block(ref stmt) => visitor.visit_block_stmt(stmt),
            Stmt::If(ref stmt) => visitor.visit_if_stmt(stmt),
            Stmt::While(ref stmt) => visitor.visit_while_stmt(stmt),
            Stmt::Function(ref stmt) => visitor.visit_function_stmt(stmt),
        }
    }

    pub fn new_expression(expression: Expr) -> Self {
        Self::Expression(StmtExpression { expression })
    }

    pub fn new_print(expression: Expr) -> Self {
        Self::Print(StmtPrint { expression })
    }

    pub fn new_var(name: Token, initializer: Option<Expr>) -> Self {
        Self::Var(StmtVar { name, initializer })
    }

    pub fn new_block(statements: Vec<Stmt>) -> Self {
        Self::Block(StmtBlock { statements })
    }

    pub fn new_if(condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>>) -> Self {
        Self::If(StmtIf {
            condition,
            then_branch,
            else_branch,
        })
    }

    pub fn new_while(condition: Expr, body: Box<Stmt>) -> Self {
        Self::While(StmtWhile { condition, body })
    }

    pub fn new_function(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Self::Function(StmtFunction { name, params, body })
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

#[derive(Debug)]
pub struct StmtBlock {
    pub statements: Vec<Stmt>,
}

#[derive(Debug)]
pub struct StmtIf {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug)]
pub struct StmtWhile {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

#[derive(Debug)]
pub struct StmtFunction {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}
