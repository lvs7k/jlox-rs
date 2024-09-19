use std::ops::Deref;

use crate::expr::*;

#[derive(Debug)]
pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize<E>(&mut self, name: &str, exprs: &[E]) -> String
    where
        E: Deref<Target = Expr>,
    {
        let mut builder = String::new();

        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }
        builder.push(')');

        builder
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_literal_expr(&mut self, expr: &ExprLiteral) -> String {
        expr.value.to_string()
    }

    fn visit_unary_expr(&mut self, expr: &ExprUnary) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&*expr.right])
    }

    fn visit_binary_expr(&mut self, expr: &ExprBinary) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&*expr.left, &*expr.right])
    }

    fn visit_grouping_expr(&mut self, expr: &ExprGrouping) -> String {
        self.parenthesize("group", &[&*expr.expression])
    }

    fn visit_variable_expr(&mut self, expr: &ExprVariable) -> String {
        unimplemented!();
    }

    fn visit_assign_expr(&mut self, expr: &ExprAssign) -> String {
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use crate::{object::Object, token::Token, token_type::TokenType::*};

    use super::*;

    #[test]
    fn astprinter_books_example() {
        let left = Expr::unary(
            Token::new(Minus, "-".into(), Object::Null, 1),
            Expr::literal(Object::Num(123f64)),
        );
        let op = Token::new(Star, "*".into(), Object::Null, 1);
        let right = Expr::grouping(Expr::literal(Object::Num(45.67f64)));

        let expression = Expr::binary(left, op, right);

        assert_eq!(
            "(* (- 123) (group 45.67))".to_string(),
            AstPrinter.print(&expression)
        );
    }
}
