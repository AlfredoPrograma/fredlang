#![allow(dead_code)]

use std::{error::Error, fmt};

use crate::tokenizer::{
    parse::parse_token,
    tokens::{Token, TokenKind},
};

/// Contains information about error occurred during the scanning proccess.
#[derive(Debug)]
pub struct ScannerError {
    message: &'static str,
    source: Option<Box<dyn Error>>,
}

impl ScannerError {
    pub fn new(message: &'static str, source: Option<Box<dyn Error>>) -> Self {
        Self { message, source }
    }
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[SCANNER ERROR] {}", self.message)
    }
}

/// Keeps track of the position of the parsing proccess.
#[derive(Debug)]
pub struct Location {
    /// Current line.
    line: usize,

    /// Current offset for the current line.
    line_offset: usize,

    /// Global offset with beggining.
    cursor: usize,
}

impl Location {
    pub fn new() -> Self {
        Location {
            line: 1,
            line_offset: 0,
            cursor: 0,
        }
    }
}

impl Location {
    /// Increases line counter and resets line offset.
    pub fn update_line(&mut self) {
        self.line += 1;
        self.line_offset = 0;
    }

    /// Increases global cursor and current line offset by a characters consumed amount
    pub fn update_line_offset(&mut self, consumed: usize) {
        self.line_offset += consumed
    }

    /// Increases global cursor offset by a characters consumend amount
    pub fn update_cursor(&mut self, consumed: usize) {
        self.cursor += consumed;
    }
}

/// Encapsulates scanning states and performs scanning operations over source code.
///
/// Basically, the `Scanner` is the core element of the source code transformation into
/// meaningful structures for later interpreting.
///
/// `Scanner` parses, tokenizes and register errors during the tokenization proccess.
#[derive(Debug)]
pub struct Scanner<'a> {
    /// Source code to scan.
    source: &'a str,

    /// Parsing location.
    location: Location,

    /// Generated tokens.
    tokens: Vec<Token>,

    /// Registered errors.
    errors: Vec<ScannerError>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            location: Location::new(),
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }
}

/// Helper methods for handle internal scanner processes.
impl<'a> Scanner<'a> {
    fn is_end(&self) -> bool {
        self.location.cursor >= self.source.len()
    }
}

impl<'a> Scanner<'a> {
    /// Runs available parsers for grammar, generates a token and registers it in scanner state.
    fn register_token(&mut self, source: &str) {
        // tries to parse input as some token
        match parse_token(source) {
            Ok((token, consumed)) => match token.kind {
                // For whitespaces just update the global cursor and the line offset
                TokenKind::Whitespace => {
                    self.location.update_cursor(consumed);
                    self.location.update_line_offset(consumed);
                }

                // For end of lines update line counter and global cursor
                TokenKind::EOF => {
                    self.location.update_line();
                    self.location.update_cursor(consumed);
                }

                // For the rest (and meaningful) tokens, update line offset, global cursor and append them into the tokens register
                _ => {
                    self.location.update_line_offset(consumed);
                    self.location.update_cursor(consumed);

                    self.tokens.push(token);
                }
            },
            Err(err) => {
                const END_OF_LINE: &'static str = "\n";
                let scanner_err = ScannerError::new("cannot register token", Some(Box::new(err)));

                // takes current input line until next `\n` character and compute the consumed characters amount
                match source.split_once(END_OF_LINE) {
                    Some((before, _)) => {
                        // compute `consumed` value adding up the length of the line and the delimiter character size
                        let consumed = before.len() + END_OF_LINE.len();
                        self.location.update_line();
                        self.location.update_cursor(consumed);
                    }

                    // if none `before` exists is because we are at last line, so just update cursor with current source length
                    None => self.location.update_cursor(source.len()),
                }

                // always register error
                self.errors.push(scanner_err)
            }
        }
    }

    /// Loops over the source code and tokenizes it until ends.
    ///
    /// If tokenizing fails, keeps looping and registering errors in scanner state
    pub fn tokenize(&mut self) {
        let mut current_source = self.source;

        while !self.is_end() {
            self.register_token(current_source);
            current_source = &self.source[self.location.cursor..];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::tokens::{Token, TokenKind};

    use super::{Location, Scanner};

    const SOURCE: &'static str = "()\n-.\n";

    #[test]
    fn new_location() {
        let location = Location::new();

        assert_eq!(
            location.cursor, 0,
            "should be initialized with global cursor at position zero"
        );

        assert_eq!(location.line, 1, "should be initialized with line as one");

        assert_eq!(
            location.line_offset, 0,
            "should be initialized with line offset at position zero"
        )
    }

    #[test]
    fn update_line_location() {
        const RND_LINE: usize = 4;
        const RND_LINE_OFFSET: usize = 22;

        let mut location = Location::new();

        // check concrete line and line offset state
        location.line = RND_LINE;
        location.line_offset = RND_LINE_OFFSET;

        location.update_line();

        assert_eq!(
            location.line,
            RND_LINE + 1,
            "should update line counter by one"
        );
        assert_eq!(
            location.line_offset, 0,
            "should reset line offset after update line"
        );
    }

    #[test]
    fn update_offset_location() {
        const RND_OFFSET: usize = 25;
        let mut location = Location::new();

        location.update_cursor(RND_OFFSET);
        location.update_line_offset(RND_OFFSET);

        assert_eq!(
            location.line_offset, RND_OFFSET,
            "should update line offset by given consumed characters amount"
        );

        assert_eq!(
            location.cursor, RND_OFFSET,
            "should global cursor by given consumed characters amount"
        );
    }

    #[test]
    fn new_scanner() {
        let scanner = Scanner::new(SOURCE);

        assert_eq!(
            scanner.source, SOURCE,
            "should be initialized with given source code"
        );

        assert_eq!(
            scanner.tokens.len(),
            0,
            "should be initialized with empty tokens list"
        );

        assert_eq!(
            scanner.errors.len(),
            0,
            "should be initialized with empty errors list"
        )
    }

    #[test]
    fn scanner_is_end() {
        let mut scanner = Scanner::new(SOURCE);

        assert!(
            !scanner.is_end(),
            "should return false if global cursor is not at the end of the source code"
        );

        // set the global cursor at the end of the source code
        scanner.location.cursor = SOURCE.len();

        assert!(
            scanner.is_end(),
            "should return true if global cursor is  at the end of the source code"
        )
    }

    #[test]
    fn scanner_register_token() {
        let expected_tokens = vec![
            Token::new('('.to_string(), TokenKind::OpeningParentheses),
            Token::new(')'.to_string(), TokenKind::ClosingParentheses),
            Token::new('-'.to_string(), TokenKind::Minus),
            Token::new('.'.to_string(), TokenKind::Dot),
        ];

        let mut scanner = Scanner::new(SOURCE);

        for (i, _) in SOURCE.chars().enumerate() {
            scanner.register_token(&SOURCE[i..])
        }

        // check all tokens were registered successfully
        for (i, token) in scanner.tokens.clone().into_iter().enumerate() {
            assert_eq!(
                token, expected_tokens[i],
                "should register token in scanner state"
            )
        }

        // check location state was updated successfully
        assert_eq!(
            scanner.location.line, 3,
            "should update line counter on each end of line character"
        );

        assert_eq!(
            scanner.location.cursor,
            SOURCE.len(),
            "should end scanning proccess with the same count as source lenght"
        );

        assert_eq!(
            scanner.location.line_offset, 0,
            "should end with line offset counter as zero"
        )
    }

    #[test]
    fn scanner_tokenize() {
        const SOURCE: &'static str = "()\n.\n_invalid\n{+-*}";

        // just one error for `SOURCE` string ("invalid" fragment)
        const EXPECTED_ERRORS_AMOUNT: usize = 1;

        // line counter should be 4 after tokenization for `SOURCE` input
        const EXPECTED_LINE_AMOUNT: usize = 4;

        let expected_tokens = vec![
            Token::new('('.to_string(), TokenKind::OpeningParentheses),
            Token::new(')'.to_string(), TokenKind::ClosingParentheses),
            Token::new('.'.to_string(), TokenKind::Dot),
            Token::new('{'.to_string(), TokenKind::OpeningCurlyBrace),
            Token::new('+'.to_string(), TokenKind::Plus),
            Token::new('-'.to_string(), TokenKind::Minus),
            Token::new('*'.to_string(), TokenKind::Star),
            Token::new('}'.to_string(), TokenKind::ClosingCurlyBrace),
        ];

        let mut scanner = Scanner::new(SOURCE);
        scanner.tokenize();

        // check registered tokens
        for (i, token) in scanner.tokens.clone().into_iter().enumerate() {
            assert_eq!(
                token, expected_tokens[i],
                "should generate token's register based on current input"
            );
        }

        // check registered errors
        assert_eq!(
            scanner.errors.len(),
            EXPECTED_ERRORS_AMOUNT,
            "should register errors"
        );

        // check location state
        assert_eq!(
            scanner.location.cursor,
            SOURCE.len(),
            "should position global cursor at end of the source input after complete tokenization"
        );

        assert_eq!(
            scanner.location.line, EXPECTED_LINE_AMOUNT,
            "should contain the total amount of lines registered after complete tokenization"
        );
    }
}
