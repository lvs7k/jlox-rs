use crate::{object::Object, token::Token};

pub trait ExprVisitor<R> {
    fn visit_literal_expr(&self, expr: &ExprLiteral) -> R;
    fn visit_unary_expr(&self, expr: &ExprUnary) -> R;
    fn visit_binary_expr(&self, expr: &ExprBinary) -> R;
    fn visit_grouping_expr(&self, expr: &ExprGrouping) -> R;
    fn visit_variable_expr(&self, expr: &ExprVariable) -> R;
    fn visit_assign_expr(&self, expr: &ExprAssign) -> R;
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(ExprLiteral),
    Unary(ExprUnary),
    Binary(ExprBinary),
    Grouping(ExprGrouping),
    Variable(ExprVariable),
    Assign(ExprAssign),
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
            Expr::Variable(ref expr) => visitor.visit_variable_expr(expr),
            Expr::Assign(ref expr) => visitor.visit_assign_expr(expr),
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

    pub fn variable(name: Token) -> Self {
        Self::Variable(ExprVariable { name })
    }

    pub fn assign(name: Token, value: Expr) -> Self {
        Self::Assign(ExprAssign {
            name,
            value: Box::new(value),
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

#[derive(Debug, PartialEq)]
pub struct ExprVariable {
    pub name: Token,
}

#[derive(Debug, PartialEq)]
pub struct ExprAssign {
    pub name: Token,
    pub value: Box<Expr>,
}
