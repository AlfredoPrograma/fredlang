use std::any;

use crate::{parser::expressions::Expression, prelude::Result, tokens::TokenKind};

/// It is responsible of take the built AST and execute its operations.
struct Interpreter {
    root: Expression,
}

impl Interpreter {
    pub fn new(root: Expression) -> Self {
        Self { root }
    }
}

impl Interpreter {
    pub fn evaluate(self) -> Result<Box<dyn any::Any>> {
        Self::evaluate_recursive(self.root)
    }

    fn evaluate_recursive(expr: Expression) -> Result<Box<dyn any::Any>> {
        match expr {
            Expression::Number(n) => Ok(Box::new(n)),
            Expression::String(s) => Ok(Box::new(s)),
            Expression::Boolean(b) => Ok(Box::new(b)),
            Expression::Null(opt) => Ok(Box::new(opt)),
            Expression::Unary(operator, expr) => {
                let value = Self::evaluate_recursive(*expr)?;

                match operator.kind {
                    TokenKind::Minus => {
                        let number = value
                            .downcast_ref::<f32>()
                            .ok_or("cannot evaluate expression as number")?;

                        Ok(Box::new(-number))
                    }
                    TokenKind::Bang => {
                        let boolean = value
                            .downcast_ref::<bool>()
                            .ok_or("negation operation over non boolean values is invalid")?;

                        Ok(Box::new(!*boolean))
                    }
                    _ => Err("invalid unary operator".into()),
                }
            }
            Expression::Binary(left_expr, operator, right_expr) => {
                let left_expr = Self::evaluate_recursive(*left_expr)?;
                let left_value = left_expr
                    .downcast_ref::<f32>()
                    .ok_or("cannot evaluate expression as number")?;

                let right_expr = Self::evaluate_recursive(*right_expr)?;
                let right_value = right_expr
                    .downcast_ref::<f32>()
                    .ok_or("cannot evaluate expression as number")?;

                match operator.kind {
                    // Arithmetic binary operations
                    TokenKind::Plus => Ok(Box::new(left_value + right_value)),
                    TokenKind::Minus => Ok(Box::new(left_value - right_value)),
                    TokenKind::Star => Ok(Box::new(left_value * right_value)),
                    TokenKind::Slash => {
                        if *right_value == 0.0 {
                            return Err("cannot divide by zero".into());
                        }

                        Ok(Box::new(left_value / right_value))
                    }

                    // Comparison binary operations
                    TokenKind::Great => Ok(Box::new(left_value > right_value)),
                    TokenKind::GreatEqual => Ok(Box::new(left_value >= right_value)),
                    TokenKind::Less => Ok(Box::new(left_value < right_value)),
                    TokenKind::LessEqual => Ok(Box::new(left_value <= right_value)),

                    // Equality binary operations
                    TokenKind::DoubleEqual => Ok(Box::new(left_value == right_value)),
                    TokenKind::BangEqual => Ok(Box::new(left_value != right_value)),

                    _ => Err("invalid binary operator".into()),
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
            Interpreter::new(number_expr)
                .evaluate()
                .is_ok_and(|value| value.downcast::<f32>().is_ok()),
            "should extract number value from number expression"
        )
    }

    #[test]
    fn evaluate_string_expr() {
        let string_expr = Expression::String("Hello world".to_string());

        assert!(
            Interpreter::new(string_expr)
                .evaluate()
                .is_ok_and(|value| value.downcast::<String>().is_ok()),
            "should extract string value from string expression"
        )
    }

    #[test]
    fn evaluate_bool_expr() {
        let true_expr = Expression::Boolean(true);
        let false_expr = Expression::Boolean(false);

        assert!(Interpreter::new(true_expr)
            .evaluate()
            .is_ok_and(|value| value.downcast::<bool>().is_ok()));

        assert!(Interpreter::new(false_expr)
            .evaluate()
            .is_ok_and(|value| value.downcast::<bool>().is_ok()));
    }

    #[test]
    fn evaluate_null_expr() {
        let null_expr = Expression::Null(None);

        assert!(Interpreter::new(null_expr)
            .evaluate()
            .is_ok_and(|value| value.downcast::<Option<Box<Expression>>>().is_ok()))
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
            let interpreter = Interpreter::new(expr);

            assert!(interpreter.evaluate().is_ok_and(|value| value
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
            let interpreter = Interpreter::new(expr);

            assert!(interpreter.evaluate().is_ok_and(|value| value
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
            let interpreter = Interpreter::new(expr);

            assert!(interpreter.evaluate().is_ok_and(|value| value
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
            let interpreter = Interpreter::new(expr);

            assert!(interpreter.evaluate().is_ok_and(|value| value
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
            let interpreter = Interpreter::new(expr);
            assert!(interpreter.evaluate().is_ok_and(|value| value
                .downcast::<bool>()
                .is_ok_and(|boolean| *boolean == true)))
        }
    }
}
