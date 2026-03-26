pub mod value;

use crate::parser::ast::{Expr, Stmt};
use crate::lexer::token::TokenKind;
use value::Value;


pub fn eval(expr: &Expr) -> Value {
    match expr {
        Expr::Literal(token) => match &token.kind {
            TokenKind::NumberLiteral(n) => {
                Value::Number(n.parse::<f64>().unwrap_or(0.0))
            }
            TokenKind::StringLiteral(s) => {
                Value::String(s.clone())
            }
            TokenKind::True => Value::Boolean(true),
            TokenKind::False => Value::Boolean(false),
            TokenKind::Nil => Value::Nil,
            _ => Value::Nil,
        },

        Expr::Grouping(inner) => eval(inner),

        Expr::Unary { operator, right } => {
            let right = eval(right);
            match operator.kind {
                TokenKind::Minus => match right {
                    Value::Number(n) => Value::Number(-n),
                    _ => Value::Nil,
                },
                TokenKind::Bang => match right {
                    Value::Boolean(b) => Value::Boolean(!b),
                    _ => Value::Boolean(false),
                },
                _ => Value::Nil,
            }
        }

        Expr::Binary { left, operator, right } => {
            let left = eval(left);
            let right = eval(right);

            match operator.kind {
                TokenKind::Plus => match (left, right) {
                    (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                    (Value::String(a), Value::String(b)) => Value::String(a + &b),
                    _ => Value::Nil,
                }

                TokenKind::Minus => match (left, right) {
                    (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
                    _ => Value::Nil,
                }

                TokenKind::Star => match (left, right) {
                    (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
                    _ => Value::Nil,
                }

                TokenKind::Slash => match (left, right) {
                    (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
                    _ => Value::Nil,
                }

                TokenKind::EqualEqual => Value::Boolean(left == right),
                TokenKind::BangEqual => Value::Boolean(left != right),

                TokenKind::Greater => match (left, right) {
                    (Value::Number(a), Value::Number(b)) => Value::Boolean(a > b),
                    _ => Value::Nil,
                }

                TokenKind::Less => match (left, right) {
                    (Value::Number(a), Value::Number(b)) => Value::Boolean(a < b),
                    _ => Value::Nil,
                }

                _ => Value::Nil,
            }
        }

        _ => Value::Nil,
    }
}
