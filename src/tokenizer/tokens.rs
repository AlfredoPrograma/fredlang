#![allow(dead_code)]

#[derive(Debug, PartialEq)]
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
}

#[derive(Debug, PartialEq)]
/// Represents the tokenized structure for given gramar.
pub struct Token {
    lexeme: String,
    kind: TokenKind,
}

impl Token {
    pub fn new(lexeme: String, kind: TokenKind) -> Self {
        Self { lexeme, kind }
    }
}
