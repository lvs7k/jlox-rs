#![allow(dead_code)]

use crate::expr::*;

#[derive(Debug)]
pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
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

    fn visit_variable_expr(&mut self, _expr: &ExprVariable) -> String {
        unimplemented!();
    }

    fn visit_assign_expr(&mut self, _expr: &ExprAssign) -> String {
        unimplemented!();
    }

    fn visit_logical_expr(&mut self, _expr: &ExprLogical) -> String {
        unimplemented!();
    }

    fn visit_call_expr(&mut self, _expr: &ExprCall) -> String {
        unimplemented!();
    }

    fn visit_get_expr(&mut self, _expr: &ExprGet) -> String {
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use crate::{object::Object, token::Token, token_type::TokenType::*};

    use super::*;

    #[test]
    fn astprinter_books_example() {
        let left = Expr::new_unary(
            Token::new(Minus, "-".into(), Object::Null, 1),
            Expr::new_literal(Object::Num(123f64)),
        );
        let op = Token::new(Star, "*".into(), Object::Null, 1);
        let right = Expr::new_grouping(Expr::new_literal(Object::Num(45.67f64)));

        let expression = Expr::new_binary(left, op, right);

        assert_eq!(
            "(* (- 123) (group 45.67))".to_string(),
            AstPrinter.print(&expression)
        );
    }
}
