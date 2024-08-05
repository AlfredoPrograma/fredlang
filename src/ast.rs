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
    use crate::tokens::{Token, TokenKind};

    use super::{Expression, Parser, AST};

    #[test]
    fn it_works() {}
}
