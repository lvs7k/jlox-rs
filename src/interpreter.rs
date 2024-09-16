use crate::{
    expr::{Expr, ExprBinary, ExprGrouping, ExprLiteral, ExprUnary, Visitor},
    object::Object,
    token_type::TokenType,
};

#[derive(Debug)]
pub struct Interpreter {}

impl Interpreter {
    pub fn evaluate<E>(&self, expr: &E) -> Object
    where
        E: std::ops::Deref<Target = Expr>,
    {
        expr.accept(self)
    }
}

impl Visitor<Object> for Interpreter {
    fn visit_literal_expr(&self, expr: &ExprLiteral) -> Object {
        expr.value.clone()
    }

    fn visit_unary_expr(&self, expr: &ExprUnary) -> Object {
        let right = self.evaluate(&expr.right);

        match expr.operator.typ {
            TokenType::Bang => !right,
            TokenType::Minus => -right,
            _ => unreachable!(),
        }
    }

    fn visit_binary_expr(&self, expr: &ExprBinary) -> Object {
        let left = self.evaluate(&expr.left);
        let right = self.evaluate(&expr.right);

        match expr.operator.typ {
            TokenType::Greater => Object::Bool(left > right),
            TokenType::GreaterEqual => Object::Bool(left >= right),
            TokenType::Less => Object::Bool(left < right),
            TokenType::LessEqual => Object::Bool(left <= right),
            TokenType::BangEqual => Object::Bool(left != right),
            TokenType::EqualEqual => Object::Bool(left == right),
            TokenType::Minus => left - right,
            TokenType::Plus => left + right,
            TokenType::Slash => left / right,
            TokenType::Star => left * right,
            _ => unreachable!(),
        }
    }

    fn visit_grouping_expr(&self, expr: &ExprGrouping) -> Object {
        self.evaluate(&expr.expression)
    }
}
