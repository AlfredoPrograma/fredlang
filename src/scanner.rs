use std::{error::Error, iter::Peekable, ops::Deref, str::Chars};

use crate::{
    prelude::Result,
    tokens::{Token, TokenKind, Tokenizer},
};

/// Holds all the state of the tokenization process. It uses the given source code and inspects it
/// to emit the corresponding tokens based on the encountered lexemes.
///
/// Also keeps track of the current location of the scanning mechanism.
pub struct Scanner<'a> {
    /// Source code that is going to be transformed into tokens.
    source: Peekable<Chars<'a>>,
    /// Line number of the current scanning execution.
    pub line: u32,
    /// Holds the produced tokens. See [`crate::Token`] and [`crate::TokenKind`] for extra info.
    pub tokens: Vec<Token>,
    /// Holds the produced errors on scanning tokens process.
    errors: Vec<Box<dyn Error>>,
}

impl<'a> Scanner<'a> {
    /// Constructs a new scanner instance.
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            tokens: Vec::new(),
            errors: Vec::new(),
            line: 1,
        }
    }

    /// Iterates over `self.source`, identifies valid lexemes and builds the corresponding
    /// tokens instances for them.
    ///
    /// If found string fragment doesn't match with any valid lexeme, then report a new error.
    pub fn scan_tokens(&mut self) {
        while let Some(c) = self.source.next() {
            // Consumes line break character and updates line state on scanner.
            if c == '\n' {
                self.update_line();
                continue;
            }

            // Ignore whitespaces characters (`' ' | '\t' | '\r'`)
            // Notice Rust considers line break (`\n`) as whitespace character, but we handled this case
            // previously because line break has to update current scanner line to.
            // Warn: the whitespaces characters skipping always should come after the line break case handling in order
            // to preserve the line updating logic.
            if c.is_whitespace() {
                continue;
            }

            // Any line which starts with `#` character is considered as a single line comment, so we should ignore their
            // characters until find a line break character.
            if c == '#' {
                self.skip_single_line_comment();
                continue;
            }

            // TODO: maybe refactor this bunch of matching patterns in some way.
            let token = match c {
                // Single char tokens
                '(' => self.build_single_char_token(c, TokenKind::LeftParentheses, self.line),
                ')' => self.build_single_char_token(c, TokenKind::RightParentheses, self.line),
                '{' => self.build_single_char_token(c, TokenKind::LeftBrace, self.line),
                '}' => self.build_single_char_token(c, TokenKind::RightBrace, self.line),
                ',' => self.build_single_char_token(c, TokenKind::Comma, self.line),
                '.' => self.build_single_char_token(c, TokenKind::Dot, self.line),
                '-' => self.build_single_char_token(c, TokenKind::Minus, self.line),
                '+' => self.build_single_char_token(c, TokenKind::Plus, self.line),
                ';' => self.build_single_char_token(c, TokenKind::Semicolon, self.line),
                '/' => self.build_single_char_token(c, TokenKind::Slash, self.line),
                '*' => self.build_single_char_token(c, TokenKind::Star, self.line),

                // Either pair chars tokens or single char tokens depending on the next character.
                '=' => {
                    let target = '=';

                    if self.match_incoming(target) {
                        self.build_pair_chars_token(c, target, TokenKind::DoubleEqual, self.line)
                    } else {
                        self.build_single_char_token(c, TokenKind::Equal, self.line)
                    }
                }
                '!' => {
                    let target = '=';

                    if self.match_incoming(target) {
                        self.build_pair_chars_token(c, target, TokenKind::BangEqual, self.line)
                    } else {
                        self.build_single_char_token(c, TokenKind::Bang, self.line)
                    }
                }
                '>' => {
                    let target = '=';

                    if self.match_incoming(target) {
                        self.build_pair_chars_token(c, target, TokenKind::GreatEqual, self.line)
                    } else {
                        self.build_single_char_token(c, TokenKind::Great, self.line)
                    }
                }
                '<' => {
                    let target = '=';

                    if self.match_incoming(target) {
                        self.build_pair_chars_token(c, target, TokenKind::LessEqual, self.line)
                    } else {
                        self.build_single_char_token(c, TokenKind::Less, self.line)
                    }
                }

                // String tokens
                '"' => self.build_string_token(),
                // Number tokens
                c if c.is_ascii_digit() => self.build_number_token(c),
                // Keyword or identifier tokens
                c if c.is_ascii_alphabetic() => self.build_keyword_or_identifier_token(c),

                _ => Err("unexpected token".into()),
            };

            // Pushes the built [`Token`] or the captured error into the corresponding vector tracker.
            match token {
                Ok(t) => self.tokens.push(t),
                Err(e) => self.errors.push(e),
            };
        }
    }

    /// Skip characters within single line comment from comment until end of line
    fn skip_single_line_comment(&mut self) {
        while let Some(c) = self.source.next() {
            if c == '\n' {
                break;
            }
        }
    }

    /// Checks if next character matches with the given target character.
    fn match_incoming(&mut self, target: char) -> bool {
        if let Some(c) = self.source.peek() {
            return *c == target;
        }

        false
    }

    /// Updates the line number by 1.
    fn update_line(&mut self) {
        self.line += 1
    }
}

impl<'a> Tokenizer for Scanner<'a> {
    fn build_single_char_token(
        &mut self,
        char_lexeme: char,
        kind: TokenKind,
        line: u32,
    ) -> Result<Token> {
        Ok(Token::new(char_lexeme.to_string(), kind, line))
    }

    fn build_pair_chars_token(
        &mut self,
        left_char: char,
        right_char: char,
        kind: TokenKind,
        line: u32,
    ) -> Result<Token> {
        self.source.next();

        let chars_lexeme = format!("{left_char}{right_char}");
        Ok(Token::new(chars_lexeme, kind, line))
    }

    fn build_string_token(&mut self) -> Result<Token> {
        let mut string_lexeme = String::new();

        // Go through string content characters
        while let Some(c) = self.source.next() {
            if c == '"' {
                break;
            }

            string_lexeme.push(c);
        }

        // Skips the string character closing delimiter `"`
        self.source.next();
        Ok(Token::new(string_lexeme, TokenKind::String, self.line))
    }

    fn build_number_token(&mut self, first_char: char) -> Result<Token> {
        let mut number_extreme = String::from(first_char);

        while let Some(c) = self.source.next() {
            if !c.is_ascii_digit() {
                break;
            }

            number_extreme.push(c);
        }

        Ok(Token::new(number_extreme, TokenKind::Number, self.line))
    }

    fn build_keyword_or_identifier_token(&mut self, first_char: char) -> Result<Token> {
        let mut keyword_lexeme = String::from(first_char);

        while let Some(c) = self
            .source
            .next_if(|next| next.is_ascii_alphanumeric() || *next == '_')
        {
            keyword_lexeme.push(c)
        }

        let token_kind = match keyword_lexeme.deref() {
            "and" => Ok(TokenKind::And),
            "else" => Ok(TokenKind::Else),
            "false" => Ok(TokenKind::False),
            "for" => Ok(TokenKind::For),
            "function" => Ok(TokenKind::Function),
            "if" => Ok(TokenKind::If),
            "null" => Ok(TokenKind::Null),
            "or" => Ok(TokenKind::Or),
            "print" => Ok(TokenKind::Print),
            "return" => Ok(TokenKind::Return),
            "true" => Ok(TokenKind::True),
            "var" => Ok(TokenKind::Var),
            "while" => Ok(TokenKind::While),
            _ => Ok(TokenKind::Identifier),
        };

        token_kind.map(|kind| Token::new(keyword_lexeme, kind, self.line))
    }
}

#[cfg(test)]
mod scanner_tests {
    use crate::tokens::{Token, TokenKind};

    use super::Scanner;

    #[test]
    /// Runs `scan_tokens` over a source with only single chars valid lexemes.
    fn scan_over_single_char_tokens() {
        let source = "{},.-+;/*!><()=";
        let expected_tokens = vec![
            Token::new("{".to_string(), TokenKind::LeftBrace, 1),
            Token::new("}".to_string(), TokenKind::RightBrace, 1),
            Token::new(",".to_string(), TokenKind::Comma, 1),
            Token::new(".".to_string(), TokenKind::Dot, 1),
            Token::new("-".to_string(), TokenKind::Minus, 1),
            Token::new("+".to_string(), TokenKind::Plus, 1),
            Token::new(";".to_string(), TokenKind::Semicolon, 1),
            Token::new("/".to_string(), TokenKind::Slash, 1),
            Token::new("*".to_string(), TokenKind::Star, 1),
            Token::new("!".to_string(), TokenKind::Bang, 1),
            Token::new(">".to_string(), TokenKind::Great, 1),
            Token::new("<".to_string(), TokenKind::Less, 1),
            Token::new("(".to_string(), TokenKind::LeftParentheses, 1),
            Token::new(")".to_string(), TokenKind::RightParentheses, 1),
            Token::new("=".to_string(), TokenKind::Equal, 1),
        ];

        let mut scanner = Scanner::new(&source);
        scanner.scan_tokens();

        assert_eq!(
            scanner.tokens, expected_tokens,
            "scanned tokens for single char lexemes should be same as the expected tokens"
        )
    }

    #[test]
    /// Runs `scan tokens` over a source build from two chars valid lexemes.
    fn scan_over_two_subsequent_chars_tokens() {
        let source = "==!=>=<=";
        let expected_tokens = vec![
            Token::new("==".to_string(), TokenKind::DoubleEqual, 1),
            Token::new("!=".to_string(), TokenKind::BangEqual, 1),
            Token::new(">=".to_string(), TokenKind::GreatEqual, 1),
            Token::new("<=".to_string(), TokenKind::LessEqual, 1),
        ];

        let mut scanner = Scanner::new(&source);
        scanner.scan_tokens();

        assert_eq!(
            scanner.tokens, expected_tokens,
            "scanned tokens for two subsequent chars lexemes should be same as the expected tokens"
        )
    }

    #[test]
    /// Runs `scan_tokens` over a source built from a valid string pattern lexeme.
    fn scan_string() {
        let source = "\"Hello world\"";
        let expected_tokens = vec![Token::new("Hello world".to_string(), TokenKind::String, 1)];

        let mut scanner = Scanner::new(&source);
        scanner.scan_tokens();

        assert_eq!(
            scanner.tokens, expected_tokens,
            "scanned tokens for string lexemes should be same as expected tokens"
        )
    }

    #[test]
    /// Run `scan_tokens` over a source built from a valid integer number pattern lexeme.
    fn scan_number() {
        let source = "10 25 30";
        let expected_tokens = vec![
            Token::new("10".to_string(), TokenKind::Number, 1),
            Token::new("25".to_string(), TokenKind::Number, 1),
            Token::new("30".to_string(), TokenKind::Number, 1),
        ];

        let mut scanner = Scanner::new(&source);
        scanner.scan_tokens();

        assert_eq!(
            scanner.tokens, expected_tokens,
            "scanned tokens for number lexeme should be same as expected tokens"
        )
    }

    #[test]
    /// Runs `scan_tokens` over source built from keyword or identifier lexeme.
    fn scan_keyword_and_identifier() {
        let source =
            "and else false for function if null or print return true var while my_own_identifier";
        let expected_tokens = vec![
            Token::new("and".to_string(), TokenKind::And, 1),
            Token::new("else".to_string(), TokenKind::Else, 1),
            Token::new("false".to_string(), TokenKind::False, 1),
            Token::new("for".to_string(), TokenKind::For, 1),
            Token::new("function".to_string(), TokenKind::Function, 1),
            Token::new("if".to_string(), TokenKind::If, 1),
            Token::new("null".to_string(), TokenKind::Null, 1),
            Token::new("or".to_string(), TokenKind::Or, 1),
            Token::new("print".to_string(), TokenKind::Print, 1),
            Token::new("return".to_string(), TokenKind::Return, 1),
            Token::new("true".to_string(), TokenKind::True, 1),
            Token::new("var".to_string(), TokenKind::Var, 1),
            Token::new("while".to_string(), TokenKind::While, 1),
            Token::new("my_own_identifier".to_string(), TokenKind::Identifier, 1),
        ];

        let mut scanner = Scanner::new(&source);
        scanner.scan_tokens();

        assert_eq!(
            scanner.tokens, expected_tokens,
            "scanner tokens for keyword lexemes should be same as expected tokens"
        )
    }

    #[test]
    /// Runs `scan_tokens` over an invalid lexeme and capture the error.
    fn scan_fails_on_invalid_lexeme() {
        let invalid_source = "@";

        let mut scanner = Scanner::new(&invalid_source);
        scanner.scan_tokens();

        assert!(
            scanner.errors.len() == 1,
            "should track errors when invalid lexeme is found"
        )
    }

    #[test]
    /// Runs `update_line` and checks if it modifies the `line` field of the [`Scanner`]
    fn update_line() {
        let mut scanner = Scanner::new("");
        let expected_line = 10;

        for _ in 1..expected_line {
            scanner.update_line();
        }

        assert_eq!(
            scanner.line, expected_line,
            "scanner line should be updated once a new line character is found"
        )
    }

    #[test]
    /// Runs `skip_single_line_comment` and checks if comment characters are skipped.
    fn skip_single_line_comment() {
        let mut scanner = Scanner::new("# This is a comment\n");
        scanner.scan_tokens();

        assert_eq!(
            scanner.tokens.len(),
            0,
            "scanner should not emit any tokens after comment consuming"
        );
    }
}
