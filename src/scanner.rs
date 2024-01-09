#![allow(dead_code)]

use std::{error::Error, fmt};

/// Contains information about error occurred during the scanning proccess
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

/// Keeps track of the position of the parsing proccess
pub struct Location {
    /// current line
    line: usize,

    /// current offset for the current line
    line_offset: usize,

    /// global offset with beggining
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

/// Encapsulates scanning states and performs scanning operations over source code
/// Basically, the `Scanner` is the core element of the source code transformation into
/// meaningful structures for later interpreting.
///
/// `Scanner` parses, tokenizes and register errors during the tokenization proccess
pub struct Scanner<'a> {
    /// source code to scan
    source: &'a str,

    /// parsing location
    location: Location,

    /// generated tokens
    tokens: Vec<String>,

    /// registered errors
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

/// helper methods for handle internal scanner processes
impl<'a> Scanner<'a> {
    fn is_end(&self) -> bool {
        self.location.cursor >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{Location, Scanner};

    const SOURCE: &'static str = "hello world";

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
}
