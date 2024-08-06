use core::fmt;

use super::expressions::Expression;

/// Provides the required methods for parse the AST expressions.
pub trait Parser {
    /// Runs the parser

    /// Parses the top level expression for AST.
    fn parse_expression(&mut self) -> Option<Expression>;
    /// Parses equality expressions.
    ///
    /// equality -> comparison (("!=" | "==") comparison)*;
    fn parse_equality(&mut self) -> Option<Expression>;
    /// Parses comparison expressions.
    ///
    /// comparison -> term ((">=" | ">" | "<=" | "<") term)*;
    fn parse_comparison(&mut self) -> Option<Expression>;
    /// Parses term expressions.
    ///
    /// term -> factor (("+" | "-") factor)*;
    fn parse_term(&mut self) -> Option<Expression>;
    /// Parses factor expressions.
    ///
    /// factor -> unary (("*" | "/") factor)*;
    fn parse_factor(&mut self) -> Option<Expression>;
    /// Parses unary expressions.
    ///
    /// unary -> ("!" | "-") unary | primary;
    fn parse_unary(&mut self) -> Option<Expression>;
    /// Parses literal expressions.
    ///
    /// primary -> NUMBER | STRING | "true" | "false" | "null" | "(" expression ")";
    fn parse_primary(&mut self) -> Option<Expression>;
    /// Synchronizes the state of the parser on panic mode.
    ///
    /// It search for the end of statement (usually a semicolon or some keyword token) and restarts the parser to start parsing from new statement.
    fn sync() {
        todo!("implement sync")
    }
}

#[derive(Debug)]
/// Describes a parse error due an invalid syntax and unexpected token.
pub struct ParseError {
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ParseError {
    pub fn unexpected_token(expected: String, got: String) -> Self {
        Self {
            message: format!("invalid token found, expected \"{expected}\" but got \"{got}\""),
        }
    }

    pub fn expected_expression() -> Self {
        Self {
            message: "expected expression".to_string(),
        }
    }
}
