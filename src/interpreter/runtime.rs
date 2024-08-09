use crate::{parser::expressions::Expression, prelude::Result, tokens::TokenKind};

use super::{evaluator::Evaluator, Output};

const INVALID_UNARY_OPERATOR: &'static str = "invalid unary operator";
const INVALID_BINARY_OPERATOR: &'static str = "invalid binary operator";

/// It is responsible of take the built AST and execute its operations.
pub struct Interpreter;

impl Interpreter {
    pub fn evaluate(expr: Expression) -> Result<Output> {
        match expr {
            Expression::Number(n) => Ok(Output(Box::new(n))),
            Expression::String(s) => Ok(Output(Box::new(s))),
            Expression::Boolean(b) => Ok(Output(Box::new(b))),
            Expression::Null(opt) => Ok(Output(Box::new(opt))),
            Expression::Unary(operator, expr) => {
                let value = Self::evaluate(*expr)?.0;
                match operator.kind {
                    TokenKind::Minus => Evaluator::negate_number(value),
                    TokenKind::Bang => Evaluator::negate_boolean(value),
                    _ => Err(INVALID_UNARY_OPERATOR.into()),
                }
            }
            Expression::Binary(left_expr, operator, right_expr) => {
                let left_expr = Self::evaluate(*left_expr)?.0;
                let right_expr = Self::evaluate(*right_expr)?.0;

                match operator.kind {
                    // Arithmetic operations
                    TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash => {
                        Evaluator::compute_arithmetic(left_expr, operator, right_expr)
                    }

                    // Comparison operations
                    TokenKind::Great
                    | TokenKind::GreatEqual
                    | TokenKind::Less
                    | TokenKind::LessEqual => {
                        Evaluator::compute_comparison(left_expr, operator, right_expr)
                    }

                    // Equality operations
                    TokenKind::DoubleEqual | TokenKind::BangEqual => {
                        Evaluator::compute_equality(left_expr, operator, right_expr)
                    }

                    _ => Err(INVALID_BINARY_OPERATOR.into()),
                }
            }
            _ => todo!("implement"),
        }
    }
}

#[cfg(test)]
mod interpreter_tests {
    use crate::{
        parser::expressions::Expression,
        tokens::{Token, TokenKind},
    };

    use super::Interpreter;

    #[test]
    fn evaluate_number_expr() {
        let number_expr = Expression::Number(10.0);
        assert!(
            Interpreter::evaluate(number_expr).is_ok_and(|value| value.0.downcast::<f32>().is_ok()),
            "should extract number value from number expression"
        )
    }

    #[test]
    fn evaluate_string_expr() {
        let string_expr = Expression::String("Hello world".to_string());

        assert!(
            Interpreter::evaluate(string_expr)
                .is_ok_and(|value| value.0.downcast::<String>().is_ok()),
            "should extract string value from string expression"
        )
    }

    #[test]
    fn evaluate_bool_expr() {
        let true_expr = Expression::Boolean(true);
        let false_expr = Expression::Boolean(false);

        assert!(
            Interpreter::evaluate(true_expr).is_ok_and(|value| value.0.downcast::<bool>().is_ok())
        );

        assert!(
            Interpreter::evaluate(false_expr).is_ok_and(|value| value.0.downcast::<bool>().is_ok())
        );
    }

    #[test]
    fn evaluate_null_expr() {
        let null_expr = Expression::Null(None);

        assert!(Interpreter::evaluate(null_expr)
            .is_ok_and(|value| value.0.downcast::<Option<Box<Expression>>>().is_ok()))
    }

    #[test]
    fn evaluate_number_unary_expr() {
        let number_exprs = vec![
            Expression::Unary(
                Token::new("-".to_string(), TokenKind::Minus, 1),
                Expression::Number(10.0).as_box(),
            ),
            Expression::Unary(
                Token::new("-".to_string(), TokenKind::Minus, 1),
                Expression::Number(-2.0).as_box(),
            ),
        ];

        let expected_values = vec![Box::new(-10.0), Box::new(2.0)];

        for (i, expr) in number_exprs.into_iter().enumerate() {
            assert!(Interpreter::evaluate(expr).is_ok_and(|value| value
                .0
                .downcast::<f32>()
                .is_ok_and(|number| number == expected_values[i])))
        }
    }

    #[test]
    fn evaluate_boolean_unary_expr() {
        let boolean_exprs = vec![
            Expression::Unary(
                Token::new("!".to_string(), TokenKind::Bang, 1),
                Expression::Boolean(true).as_box(),
            ),
            Expression::Unary(
                Token::new("!".to_string(), TokenKind::Bang, 1),
                Expression::Boolean(false).as_box(),
            ),
        ];

        let expected_values = vec![Box::new(false), Box::new(true)];

        for (i, expr) in boolean_exprs.into_iter().enumerate() {
            let value = Interpreter::evaluate(expr);

            assert!(value.is_ok_and(|value| value
                .0
                .downcast::<bool>()
                .is_ok_and(|boolean| boolean == expected_values[i])))
        }
    }

    #[test]
    fn evaluate_arithmetic_binary_expr() {
        let arithmethic_exprs = vec![
            Expression::Binary(
                Expression::Number(10.0).as_box(),
                Token::new("+".to_string(), TokenKind::Plus, 1),
                Expression::Number(5.0).as_box(),
            ),
            Expression::Binary(
                Expression::Number(10.0).as_box(),
                Token::new("-".to_string(), TokenKind::Minus, 1),
                Expression::Number(5.0).as_box(),
            ),
            Expression::Binary(
                Expression::Number(10.0).as_box(),
                Token::new("*".to_string(), TokenKind::Star, 1),
                Expression::Number(5.0).as_box(),
            ),
            Expression::Binary(
                Expression::Number(10.0).as_box(),
                Token::new("/".to_string(), TokenKind::Slash, 1),
                Expression::Number(5.0).as_box(),
            ),
        ];
        let expected_values = vec![Box::new(15.0), Box::new(5.0), Box::new(50.0), Box::new(2.0)];

        for (i, expr) in arithmethic_exprs.into_iter().enumerate() {
            assert!(Interpreter::evaluate(expr).is_ok_and(|value| value
                .0
                .downcast::<f32>()
                .is_ok_and(|result| result == expected_values[i])))
        }
    }

    #[test]
    fn evaluate_comparison_binary_expr() {
        let comparison_exprs = vec![
            Expression::Binary(
                Expression::Number(10.0).as_box(),
                Token::new(">".to_string(), TokenKind::Great, 1),
                Expression::Number(5.0).as_box(),
            ),
            Expression::Binary(
                Expression::Number(10.0).as_box(),
                Token::new(">=".to_string(), TokenKind::GreatEqual, 1),
                Expression::Number(5.0).as_box(),
            ),
            Expression::Binary(
                Expression::Number(10.0).as_box(),
                Token::new(">=".to_string(), TokenKind::GreatEqual, 1),
                Expression::Number(10.0).as_box(),
            ),
            Expression::Binary(
                Expression::Number(5.0).as_box(),
                Token::new("<".to_string(), TokenKind::Less, 1),
                Expression::Number(10.0).as_box(),
            ),
            Expression::Binary(
                Expression::Number(5.0).as_box(),
                Token::new("<=".to_string(), TokenKind::LessEqual, 1),
                Expression::Number(10.0).as_box(),
            ),
            Expression::Binary(
                Expression::Number(10.0).as_box(),
                Token::new("<=".to_string(), TokenKind::LessEqual, 1),
                Expression::Number(10.0).as_box(),
            ),
        ];

        for expr in comparison_exprs {
            assert!(Interpreter::evaluate(expr).is_ok_and(|value| value
                .0
                .downcast::<bool>()
                .is_ok_and(|boolean| *boolean == true)))
        }
    }

    #[test]
    fn evaluate_equality_binary_expr() {
        let equality_exprs = vec![
            Expression::Binary(
                Expression::Number(20.0).as_box(),
                Token::new("==".to_string(), TokenKind::DoubleEqual, 1),
                Expression::Number(20.0).as_box(),
            ),
            Expression::Binary(
                Expression::Number(20.0).as_box(),
                Token::new("!=".to_string(), TokenKind::BangEqual, 1),
                Expression::Number(3.0).as_box(),
            ),
        ];

        for expr in equality_exprs {
            assert!(Interpreter::evaluate(expr).is_ok_and(|value| value
                .0
                .downcast::<bool>()
                .is_ok_and(|boolean| *boolean == true)))
        }
    }
}
