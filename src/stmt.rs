use crate::expr::Expr;

pub trait StmtVisitor<R> {
    fn visit_expression_stmt(&self, stmt: &StmtExpression) -> R;
    fn visit_print_stmt(&self, stmt: &StmtPrint) -> R;
}

#[derive(Debug)]
pub enum Stmt {
    Expression(StmtExpression),
    Print(StmtPrint),
}

impl Stmt {
    pub fn accept<V, R>(&self, visitor: &V) -> R
    where
        V: StmtVisitor<R>,
    {
        match *self {
            Stmt::Expression(ref stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Print(ref stmt) => visitor.visit_print_stmt(stmt),
        }
    }

    pub fn expression(expression: Expr) -> Self {
        Self::Expression(StmtExpression { expression })
    }

    pub fn print(expression: Expr) -> Self {
        Self::Print(StmtPrint { expression })
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
