use std::fmt::Debug;

use crate::prelude::Result;

/// `TokenKind` is the type used for describe the kind of the extracted valid lexeme of the source code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    LeftParentheses,
    RightParentheses,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Equal,
    DoubleEqual,
    Bang,
    BangEqual,
    Great,
    GreatEqual,
    Less,
    LessEqual,

    String,
    Number,

    And,
    Else,
    False,
    For,
    Function,
    If,
    Null,
    Or,
    Print,
    Return,
    True,
    Var,
    While,

    Identifier,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Holds data about the extracted lexeme.
pub struct Token {
    /// Tags the token with a `TokenKind`.
    pub kind: TokenKind,
    /// Allocates the extracted lexeme as `String`.
    pub lexeme: String,
    /// Indicates the line number where token was found.
    line: u32,
}

impl Token {
    /// Constructs a new token instance.
    pub fn new(lexeme: String, kind: TokenKind, line: u32) -> Self {
        Self { lexeme, kind, line }
    }
}

/// `Tokenizer` trait provides methods signatures for building tokens.
pub trait Tokenizer {
    /// Builds a token from a single char lexeme.
    fn build_single_char_token(
        &mut self,
        char_lexeme: char,
        kind: TokenKind,
        line: u32,
    ) -> Result<Token>;

    /// Builds a token from two chars lexeme.
    fn build_pair_chars_token(
        &mut self,
        left_char: char,
        right_char: char,
        kind: TokenKind,
        line: u32,
    ) -> Result<Token>;

    /// Builds a token from string lexeme.
    fn build_string_token(&mut self) -> Result<Token>;

    /// Builds a token from number lexeme.
    fn build_number_token(&mut self, first_char: char) -> Result<Token>;

    /// Builds a token from keyword or identifier lexeme.
    /// See [`TokenKind`] to check the available keywords kinds.
    ///
    /// In the other hand, **identifiers** are built using only ascii alphanumeric characters and underscores
    /// *(a...z | a..Z | 0..9 | _)*.
    fn build_keyword_or_identifier_token(&mut self, first_char: char) -> Result<Token>;
}
