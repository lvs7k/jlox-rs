use crate::{object::Object, token::Token};

pub trait ExprVisitor<R> {
    fn visit_literal_expr(&self, expr: &ExprLiteral) -> R;
    fn visit_unary_expr(&self, expr: &ExprUnary) -> R;
    fn visit_binary_expr(&self, expr: &ExprBinary) -> R;
    fn visit_grouping_expr(&self, expr: &ExprGrouping) -> R;
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(ExprLiteral),
    Unary(ExprUnary),
    Binary(ExprBinary),
    Grouping(ExprGrouping),
}

impl Expr {
    pub fn accept<V, R>(&self, visitor: &V) -> R
    where
        V: ExprVisitor<R>,
    {
        match *self {
            Expr::Literal(ref expr) => visitor.visit_literal_expr(expr),
            Expr::Unary(ref expr) => visitor.visit_unary_expr(expr),
            Expr::Binary(ref expr) => visitor.visit_binary_expr(expr),
            Expr::Grouping(ref expr) => visitor.visit_grouping_expr(expr),
        }
    }

    pub fn literal(value: Object) -> Self {
        Self::Literal(ExprLiteral { value })
    }

    pub fn unary(operator: Token, right: Expr) -> Self {
        Self::Unary(ExprUnary {
            operator,
            right: Box::new(right),
        })
    }

    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Binary(ExprBinary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn grouping(expression: Expr) -> Self {
        Self::Grouping(ExprGrouping {
            expression: Box::new(expression),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct ExprLiteral {
    pub value: Object,
}

#[derive(Debug, PartialEq)]
pub struct ExprUnary {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct ExprBinary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct ExprGrouping {
    pub expression: Box<Expr>,
}
