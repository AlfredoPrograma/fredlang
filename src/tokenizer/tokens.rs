#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
/// Represents the available token kinds for language grammar
pub enum TokenKind {
    // Single character tokens
    OpeningParentheses,
    ClosingParentheses,
    OpeningCurlyBrace,
    ClosingCurlyBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    EOF,

    // Possible pair tokens,
    Bang,
    BangEqual,
    Equal,
    DoubleEqual,
    Greater,
    GreaterEqual,
    Lesser,
    LesserEqual,

    // Primitives
    Literal,
    Number,
    Identifier,

    // Keywords
    And,
    Or,
    If,
    Else,
    For,
    While,
    True,
    False,
    Func,
    Return,
    Null,
    Print,
    Var,
}

#[derive(Debug, Clone, PartialEq)]
/// Represents the tokenized structure for given gramar.
pub struct Token {
    pub lexeme: String,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(lexeme: String, kind: TokenKind) -> Self {
        Self { lexeme, kind }
    }
}
