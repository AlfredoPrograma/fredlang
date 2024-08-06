use core::fmt;

use super::expressions::Expression;

/// Provides the required methods for parse the AST expressions.
pub trait Parser {
    /// Triggers the parsing mechanism
    fn parse(&mut self) -> Option<Expression> {
        todo!("implement when more statements were added")
    }

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

#[derive(Debug, PartialEq, Eq)]
/// Describes a parse error got during the expressions building.
pub enum ParseError {
    /// Sometimes the interpreter knows which would be the next token for complete the expression, like on
    /// Grouping expressions, were interpreter knows it should be closed with RightParentheses token.
    /// So this error describes this situation, where interpreter expects an specific token but gets another one.
    UnexpectedToken(String, String),

    /// It occurs when a token which doesn't can be parsed as any available expression as itself is given.
    /// An example could be starting an expression using Plus token ("+"). It is a valid token, but it cannot
    /// be parsed as any valid expression by itself.
    ExpectedExpression,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken(expected, got) => write!(
                f,
                "invalid token found, expected \"{expected}\" but got \"{got}\""
            ),
            Self::ExpectedExpression => write!(f, "expected expression"),
        }
    }
}
