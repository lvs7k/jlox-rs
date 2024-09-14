use crate::{object::Object, token::Token};

pub(crate) trait Visitor<R> {
    fn visit_literal_expr(&self, expr: &Literal) -> R;
    fn visit_unary_expr(&self, expr: &Unary) -> R;
    fn visit_binary_expr(&self, expr: &Binary) -> R;
    fn visit_grouping_expr(&self, expr: &Grouping) -> R;
}

#[derive(Debug)]
pub(crate) enum Expr {
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
}

impl Expr {
    pub(crate) fn accept<V, R>(&self, visitor: &V) -> R
    where
        V: Visitor<R>,
    {
        match *self {
            Expr::Literal(ref expr) => visitor.visit_literal_expr(expr),
            Expr::Unary(ref expr) => visitor.visit_unary_expr(expr),
            Expr::Binary(ref expr) => visitor.visit_binary_expr(expr),
            Expr::Grouping(ref expr) => visitor.visit_grouping_expr(expr),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Literal {
    pub(crate) value: Object,
}

impl Literal {
    pub(crate) fn new(value: Object) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub(crate) struct Unary {
    pub(crate) operator: Token,
    pub(crate) right: Box<Expr>,
}

impl Unary {
    pub(crate) fn new(operator: Token, right: Expr) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Binary {
    pub(crate) left: Box<Expr>,
    pub(crate) operator: Token,
    pub(crate) right: Box<Expr>,
}

impl Binary {
    pub(crate) fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Grouping {
    pub(crate) expression: Box<Expr>,
}

impl Grouping {
    pub(crate) fn new(expression: Expr) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}
