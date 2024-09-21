use uuid::Uuid;

use crate::{object::Object, token::Token};

pub trait ExprVisitor<R> {
    fn visit_literal_expr(&mut self, expr: &ExprLiteral) -> R;
    fn visit_unary_expr(&mut self, expr: &ExprUnary) -> R;
    fn visit_binary_expr(&mut self, expr: &ExprBinary) -> R;
    fn visit_grouping_expr(&mut self, expr: &ExprGrouping) -> R;
    fn visit_variable_expr(&mut self, expr: &ExprVariable) -> R;
    fn visit_assign_expr(&mut self, expr: &ExprAssign) -> R;
    fn visit_logical_expr(&mut self, expr: &ExprLogical) -> R;
    fn visit_call_expr(&mut self, expr: &ExprCall) -> R;
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(ExprLiteral),
    Unary(ExprUnary),
    Binary(ExprBinary),
    Grouping(ExprGrouping),
    Variable(ExprVariable),
    Assign(ExprAssign),
    Logical(ExprLogical),
    Call(ExprCall),
}

impl Expr {
    pub fn accept<V, R>(&self, visitor: &mut V) -> R
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
            Expr::Call(ref expr) => visitor.visit_call_expr(expr),
        }
    }

    pub fn new_literal(value: Object) -> Self {
        Self::Literal(ExprLiteral {
            id: Uuid::new_v4(),
            value,
        })
    }

    pub fn new_unary(operator: Token, right: Expr) -> Self {
        Self::Unary(ExprUnary {
            id: Uuid::new_v4(),
            operator,
            right: Box::new(right),
        })
    }

    pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Binary(ExprBinary {
            id: Uuid::new_v4(),
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn new_grouping(expression: Expr) -> Self {
        Self::Grouping(ExprGrouping {
            id: Uuid::new_v4(),
            expression: Box::new(expression),
        })
    }

    pub fn new_variable(name: Token) -> Self {
        Self::Variable(ExprVariable {
            id: Uuid::new_v4(),
            name,
        })
    }

    pub fn new_assign(name: Token, value: Expr) -> Self {
        Self::Assign(ExprAssign {
            id: Uuid::new_v4(),
            name,
            value: Box::new(value),
        })
    }

    pub fn new_logical(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Logical(ExprLogical {
            id: Uuid::new_v4(),
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn new_call(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Self {
        Self::Call(ExprCall {
            id: Uuid::new_v4(),
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Literal(l), Self::Literal(r)) => l.id == r.id,
            (Expr::Unary(l), Self::Unary(r)) => l.id == r.id,
            (Expr::Binary(l), Self::Binary(r)) => l.id == r.id,
            (Expr::Grouping(l), Self::Grouping(r)) => l.id == r.id,
            (Expr::Variable(l), Self::Variable(r)) => l.id == r.id,
            (Expr::Assign(l), Self::Assign(r)) => l.id == r.id,
            (Expr::Logical(l), Self::Logical(r)) => l.id == r.id,
            (Expr::Call(l), Self::Call(r)) => l.id == r.id,
            _ => false,
        }
    }
}

impl Eq for Expr {}

impl std::hash::Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Expr::Literal(e) => e.id.hash(state),
            Expr::Unary(e) => e.id.hash(state),
            Expr::Binary(e) => e.id.hash(state),
            Expr::Grouping(e) => e.id.hash(state),
            Expr::Variable(e) => e.id.hash(state),
            Expr::Assign(e) => e.id.hash(state),
            Expr::Logical(e) => e.id.hash(state),
            Expr::Call(e) => e.id.hash(state),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExprLiteral {
    id: Uuid,
    pub value: Object,
}

#[derive(Debug, Clone)]
pub struct ExprUnary {
    id: Uuid,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct ExprBinary {
    id: Uuid,
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct ExprGrouping {
    id: Uuid,
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct ExprVariable {
    id: Uuid,
    pub name: Token,
}

#[derive(Debug, Clone)]
pub struct ExprAssign {
    id: Uuid,
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct ExprLogical {
    id: Uuid,
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct ExprCall {
    id: Uuid,
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}
