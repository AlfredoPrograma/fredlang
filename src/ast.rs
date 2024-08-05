use std::{fmt, iter::Peekable};

use crate::tokens::{Token, TokenKind};

#[derive(Debug, PartialEq)]
/// Represents the expressions available for the AST.
enum Expression {
    Unary(Token, Box<Expression>),
    Binary(Box<Expression>, Token, Box<Expression>),
    Group(Box<Expression>),
    Number(f32),
    String(String),
    Boolean(bool),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::String(s) => write!(f, "{s}"),
            Expression::Number(n) => write!(f, "{n}"),
            Expression::Boolean(b) => write!(f, "{b}"),
            Expression::Group(expr) => writeln!(f, "({})", expr),
            Expression::Unary(op, target) => write!(f, " ({}{})", op.lexeme, target),
            Expression::Binary(left, op, right) => write!(f, "({}{}{}) ", left, op.lexeme, right),
        }
    }
}

impl Expression {
    fn as_box(self) -> Box<Self> {
        Box::new(self)
    }
}

/// Provides the required methods for parse the AST expressions.
trait Parser {
    /// Parses the top level expression for AST.
    fn parse_expression(&mut self) -> Expression;
    /// Parses equality expressions.
    ///
    /// equality -> comparison (("!=" | "==") comparison)*;
    fn parse_equality(&mut self) -> Expression;
    /// Parses comparison expressions.
    ///
    /// comparison -> term ((">=" | ">" | "<=" | "<") term)*;
    fn parse_comparison(&mut self) -> Expression;
    /// Parses term expressions.
    ///
    /// term -> factor (("+" | "-") factor)*;
    fn parse_term(&mut self) -> Expression;
    /// Parses factor expressions.
    ///
    /// factor -> unary (("*" | "/") factor)*;
    fn parse_factor(&mut self) -> Expression;
    /// Parses unary expressions.
    ///
    /// unary -> ("!" | "-") unary | primary;
    fn parse_unary(&mut self) -> Expression;
    /// Parses literal expressions.
    ///
    /// primary -> NUMBER | STRING | "true" | "false" | "null" | "(" expression ")";
    fn parse_primary(&mut self) -> Expression;
}

/// Holds the Abstract Syntax Tree (AST) expressions built from given Tokens.
pub struct AST<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
}

impl<I: Iterator<Item = Token>> AST<I> {
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }
}

impl<I: Iterator<Item = Token>> Parser for AST<I> {
    fn parse_expression(&mut self) -> Expression {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Expression {
        let left_expr = self.parse_comparison();

        while let Some(operator) = self.tokens.next_if(|next| {
            next.kind == TokenKind::DoubleEqual || next.kind == TokenKind::BangEqual
        }) {
            let right_expr = self.parse_comparison();
            return Expression::Binary(left_expr.as_box(), operator, right_expr.as_box());
        }

        left_expr
    }

    fn parse_comparison(&mut self) -> Expression {
        let left_expr = self.parse_term();

        while let Some(operator) = self.tokens.next_if(|next| {
            next.kind == TokenKind::Great
                || next.kind == TokenKind::GreatEqual
                || next.kind == TokenKind::Less
                || next.kind == TokenKind::LessEqual
        }) {
            let right_expr = self.parse_term();
            return Expression::Binary(left_expr.as_box(), operator, right_expr.as_box());
        }

        left_expr
    }

    fn parse_term(&mut self) -> Expression {
        let left_expr = self.parse_factor();

        while let Some(operator) = self
            .tokens
            .next_if(|next| next.kind == TokenKind::Plus || next.kind == TokenKind::Minus)
        {
            let right_expr = self.parse_factor();
            return Expression::Binary(left_expr.as_box(), operator, right_expr.as_box());
        }

        left_expr
    }

    fn parse_factor(&mut self) -> Expression {
        let left_expr = self.parse_unary();

        while let Some(operator) = self
            .tokens
            .next_if(|next| next.kind == TokenKind::Star || next.kind == TokenKind::Slash)
        {
            let right_expr = self.parse_unary();
            return Expression::Binary(left_expr.as_box(), operator, right_expr.as_box());
        }

        left_expr
    }

    fn parse_unary(&mut self) -> Expression {
        if let Some(operator) = self
            .tokens
            .next_if(|next| next.kind == TokenKind::Bang || next.kind == TokenKind::Minus)
        {
            let target_expr = self.parse_unary();

            return Expression::Unary(operator, target_expr.as_box());
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Expression {
        if let Some(token) = self.tokens.next() {
            let expr = match token.kind {
                TokenKind::Number => Expression::Number(
                    token
                        .lexeme
                        .parse::<f32>()
                        .expect("cannot parse token lexeme as float"),
                ),
                TokenKind::String => Expression::String(token.lexeme),
                TokenKind::True => Expression::Boolean(
                    token
                        .lexeme
                        .parse::<bool>()
                        .expect("cannot parse token lexeme as boolean"),
                ),
                TokenKind::False => Expression::Boolean(
                    token
                        .lexeme
                        .parse::<bool>()
                        .expect("cannot parse token lexeme as boolean"),
                ),
                TokenKind::LeftParentheses => {
                    let grouped_expr = self.parse_expression();
                    self.tokens.next();
                    Expression::Group(grouped_expr.as_box())
                }

                TokenKind::Null => todo!("implement null token"),
                _ => todo!(
                    "implement what happens if token does match with anything (some error I think)"
                ),
            };

            return expr;
        }

        todo!("implement what happens when it ends")
    }
}

#[cfg(test)]
mod ast_tests {
    // TODO: there is a lot of repetition for testing code. Try to refactor it to reuse some tokens definitions
    use crate::tokens::{Token, TokenKind};

    use super::{Expression, Parser, AST};

    #[test]
    /// Checks the expression building for single token expressions.
    fn parse_primary_for_single_token_expressions() {
        let tokens = vec![
            // Single character primary tokens
            Token::new("this is a string".to_string(), TokenKind::String, 1),
            Token::new("10".to_string(), TokenKind::Number, 1),
            Token::new("true".to_string(), TokenKind::True, 1),
            Token::new("false".to_string(), TokenKind::False, 1),
        ];
        let expected_expressions = vec![
            Expression::String("this is a string".to_string()),
            Expression::Number(10.0),
            Expression::Boolean(true),
            Expression::Boolean(false),
        ];

        let mut ast = AST::new(tokens.into_iter());

        for expected_expr in expected_expressions {
            assert_eq!(
                ast.parse_primary(),
                expected_expr,
                "should parse primary single token based expression"
            )
        }
    }

    #[test]
    /// Checks the expression building for grouping expressions.
    fn parse_primary_for_grouping_expressions() {
        let tokens = vec![
            Token::new("(".to_string(), TokenKind::LeftParentheses, 1),
            Token::new("inside group".to_string(), TokenKind::String, 1),
            Token::new(")".to_string(), TokenKind::RightParentheses, 1),
        ];
        let expected_expression =
            Expression::Group(Expression::String("inside group".to_string()).as_box());

        let mut ast = AST::new(tokens.into_iter());
        assert_eq!(
            ast.parse_primary(),
            expected_expression,
            "should parse primary grouping expression"
        )
    }

    #[test]
    /// Checks the expression building for unary expressions.
    fn parse_unary_expressions() {
        let bang_token = Token::new("!".to_string(), TokenKind::Bang, 1);
        let minus_token = Token::new("-".to_string(), TokenKind::Minus, 1);

        let token_combinations = vec![
            vec![
                bang_token.clone(),
                Token::new("false".to_string(), TokenKind::False, 1),
            ],
            vec![
                minus_token.clone(),
                Token::new("500".to_string(), TokenKind::Number, 1),
            ],
        ];

        let expected_expressions = vec![
            Expression::Unary(bang_token, Expression::Boolean(false).as_box()),
            Expression::Unary(minus_token, Expression::Number(500.0).as_box()),
        ];

        for (i, tokens) in token_combinations.into_iter().enumerate() {
            let mut ast = AST::new(tokens.into_iter());

            assert_eq!(
                ast.parse_unary(),
                expected_expressions[i],
                "should parse unary expressions"
            )
        }
    }

    #[test]
    /// Checks the expression building for factor expressions.
    fn parse_factor_expressions() {
        let star_token = Token::new("*".to_string(), TokenKind::Star, 1);
        let slash_token = Token::new("/".to_string(), TokenKind::Slash, 1);

        let token_combinations = vec![
            vec![
                Token::new("10".to_string(), TokenKind::Number, 1),
                star_token.clone(),
                Token::new("5".to_string(), TokenKind::Number, 1),
            ],
            vec![
                Token::new("10".to_string(), TokenKind::Number, 1),
                slash_token.clone(),
                Token::new("5".to_string(), TokenKind::Number, 1),
            ],
        ];

        let expected_expressions = vec![
            Expression::Binary(
                Expression::Number(10.0).as_box(),
                star_token,
                Expression::Number(5.0).as_box(),
            ),
            Expression::Binary(
                Expression::Number(10.0).as_box(),
                slash_token,
                Expression::Number(5.0).as_box(),
            ),
        ];

        for (i, tokens) in token_combinations.into_iter().enumerate() {
            let mut ast = AST::new(tokens.into_iter());
            assert_eq!(
                ast.parse_factor(),
                expected_expressions[i],
                "should parse factor expressions"
            )
        }
    }

    #[test]
    /// Checks the expression building for term expressions.
    fn parse_term_expressions() {
        let plus_token = Token::new("+".to_string(), TokenKind::Plus, 1);
        let minus_token = Token::new("-".to_string(), TokenKind::Minus, 1);

        let token_combinations = vec![
            vec![
                Token::new("10".to_string(), TokenKind::Number, 1),
                plus_token.clone(),
                Token::new("5".to_string(), TokenKind::Number, 1),
            ],
            vec![
                Token::new("10".to_string(), TokenKind::Number, 1),
                minus_token.clone(),
                Token::new("5".to_string(), TokenKind::Number, 1),
            ],
        ];

        let expected_expressions = vec![
            Expression::Binary(
                Expression::Number(10.0).as_box(),
                plus_token,
                Expression::Number(5.0).as_box(),
            ),
            Expression::Binary(
                Expression::Number(10.0).as_box(),
                minus_token,
                Expression::Number(5.0).as_box(),
            ),
        ];

        for (i, tokens) in token_combinations.into_iter().enumerate() {
            let mut ast = AST::new(tokens.into_iter());
            assert_eq!(
                ast.parse_term(),
                expected_expressions[i],
                "should parse term expressions"
            )
        }
    }

    #[test]
    /// Checks the expression building for comparison expressions.
    fn parse_comparison_expressions() {
        let great_token = Token::new(">".to_string(), TokenKind::Great, 1);
        let great_equal_token = Token::new(">=".to_string(), TokenKind::GreatEqual, 1);
        let less_token = Token::new("<".to_string(), TokenKind::Less, 1);
        let less_equal_token = Token::new("<=".to_string(), TokenKind::LessEqual, 1);

        let token_combinations = vec![
            vec![
                Token::new("10".to_string(), TokenKind::Number, 1),
                great_token.clone(),
                Token::new("5".to_string(), TokenKind::Number, 1),
            ],
            vec![
                Token::new("10".to_string(), TokenKind::Number, 1),
                great_equal_token.clone(),
                Token::new("5".to_string(), TokenKind::Number, 1),
            ],
            vec![
                Token::new("5".to_string(), TokenKind::Number, 1),
                less_token.clone(),
                Token::new("10".to_string(), TokenKind::Number, 1),
            ],
            vec![
                Token::new("5".to_string(), TokenKind::Number, 1),
                less_equal_token.clone(),
                Token::new("10".to_string(), TokenKind::Number, 1),
            ],
        ];

        let expected_expressions = vec![
            Expression::Binary(
                Expression::Number(10.0).as_box(),
                great_token,
                Expression::Number(5.0).as_box(),
            ),
            Expression::Binary(
                Expression::Number(10.0).as_box(),
                great_equal_token,
                Expression::Number(5.0).as_box(),
            ),
            Expression::Binary(
                Expression::Number(5.0).as_box(),
                less_token,
                Expression::Number(10.0).as_box(),
            ),
            Expression::Binary(
                Expression::Number(5.0).as_box(),
                less_equal_token,
                Expression::Number(10.0).as_box(),
            ),
        ];

        for (i, tokens) in token_combinations.into_iter().enumerate() {
            let mut ast = AST::new(tokens.into_iter());
            assert_eq!(
                ast.parse_comparison(),
                expected_expressions[i],
                "should parse comparison expressions"
            )
        }
    }

    #[test]
    /// Checks the expression building for equality expressions.
    fn parse_equality_expressions() {
        let double_equal_token = Token::new("==".to_string(), TokenKind::DoubleEqual, 1);
        let bang_equal_token = Token::new("!=".to_string(), TokenKind::BangEqual, 1);

        let token_combinations = vec![
            vec![
                Token::new("5".to_string(), TokenKind::Number, 1),
                double_equal_token.clone(),
                Token::new("5".to_string(), TokenKind::Number, 1),
            ],
            vec![
                Token::new("5".to_string(), TokenKind::Number, 1),
                bang_equal_token.clone(),
                Token::new("5".to_string(), TokenKind::Number, 1),
            ],
        ];

        let expected_expressions = vec![
            Expression::Binary(
                Expression::Number(5.0).as_box(),
                double_equal_token,
                Expression::Number(5.0).as_box(),
            ),
            Expression::Binary(
                Expression::Number(5.0).as_box(),
                bang_equal_token,
                Expression::Number(5.0).as_box(),
            ),
        ];

        for (i, tokens) in token_combinations.into_iter().enumerate() {
            let mut ast = AST::new(tokens.into_iter());
            assert_eq!(
                ast.parse_equality(),
                expected_expressions[i],
                "should parse term expressions"
            )
        }
    }
}
