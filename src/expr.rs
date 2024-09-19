use crate::{object::Object, token::Token};

pub trait ExprVisitor<R> {
    fn visit_literal_expr(&mut self, expr: &ExprLiteral) -> R;
    fn visit_unary_expr(&mut self, expr: &ExprUnary) -> R;
    fn visit_binary_expr(&mut self, expr: &ExprBinary) -> R;
    fn visit_grouping_expr(&mut self, expr: &ExprGrouping) -> R;
    fn visit_variable_expr(&mut self, expr: &ExprVariable) -> R;
    fn visit_assign_expr(&mut self, expr: &ExprAssign) -> R;
    fn visit_logical_expr(&mut self, expr: &ExprLogical) -> R;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(ExprLiteral),
    Unary(ExprUnary),
    Binary(ExprBinary),
    Grouping(ExprGrouping),
    Variable(ExprVariable),
    Assign(ExprAssign),
    Logical(ExprLogical),
}

impl Expr {
    pub fn new_accept<V, R>(&self, visitor: &mut V) -> R
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
            Expr::Logical(ref expr) => visitor.visit_logical_expr(expr),
        }
    }

    pub fn new_literal(value: Object) -> Self {
        Self::Literal(ExprLiteral { value })
    }

    pub fn new_unary(operator: Token, right: Expr) -> Self {
        Self::Unary(ExprUnary {
            operator,
            right: Box::new(right),
        })
    }

    pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Binary(ExprBinary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn new_grouping(expression: Expr) -> Self {
        Self::Grouping(ExprGrouping {
            expression: Box::new(expression),
        })
    }

    pub fn new_variable(name: Token) -> Self {
        Self::Variable(ExprVariable { name })
    }

    pub fn new_assign(name: Token, value: Expr) -> Self {
        Self::Assign(ExprAssign {
            name,
            value: Box::new(value),
        })
    }

    pub fn new_logical(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Logical(ExprLogical {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprLiteral {
    pub value: Object,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprUnary {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprBinary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprGrouping {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprVariable {
    pub name: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprAssign {
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprLogical {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
