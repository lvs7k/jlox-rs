use std::ops::Deref;

use crate::expr::{Binary, Expr, Grouping, Literal, Unary, Visitor};

#[derive(Debug)]
pub(crate) struct AstPrinter;

impl AstPrinter {
    pub(crate) fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize<E>(&self, name: &str, exprs: &[E]) -> String
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

impl Visitor<String> for AstPrinter {
    fn visit_literal_expr(&self, expr: &Literal) -> String {
        expr.value.to_string()
    }

    fn visit_unary_expr(&self, expr: &Unary) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&*expr.right])
    }

    fn visit_binary_expr(&self, expr: &Binary) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&*expr.left, &*expr.right])
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> String {
        self.parenthesize("group", &[&*expr.expression])
    }
}

#[cfg(test)]
mod test {
    use crate::{object::Object, token::Token, token_type::TokenType::*};

    use super::*;

    #[test]
    fn astprinter_books_example() {
        let left = Expr::Unary(Unary::new(
            Token::new(Minus, "-".into(), Object::Nil, 1),
            Expr::Literal(Literal::new(Object::Num(123f64))),
        ));
        let op = Token::new(Star, "*".into(), Object::Nil, 1);
        let right = Expr::Grouping(Grouping::new(Expr::Literal(Literal::new(Object::Num(
            45.67f64,
        )))));

        let expression = Expr::Binary(Binary::new(left, op, right));

        assert_eq!(
            "(* (- 123) (group 45.67))".to_string(),
            AstPrinter.print(&expression)
        );
    }
}
