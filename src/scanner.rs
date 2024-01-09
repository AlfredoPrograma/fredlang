#![allow(dead_code)]

use std::{error::Error, fmt};

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
    pub fn update_offset(&mut self, consumed: usize) {
        self.cursor += consumed;
        self.line_offset += consumed
    }
}

/// Encapsulates scanning states and performs scanning operations over source code.
///
/// Basically, the `Scanner` is the core element of the source code transformation into
/// meaningful structures for later interpreting.
///
/// `Scanner` parses, tokenizes and register errors during the tokenization proccess.
pub struct Scanner<'a> {
    /// Source code to scan.
    source: &'a str,

    /// Parsing location.
    location: Location,

    /// Generated tokens.
    tokens: Vec<String>,

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
    fn register_token() {
        todo!()
    }

    /// Loops over the source code and tokenizes it until ends.
    ///
    /// If tokenizing fails, keeps looping and registering errors in scanner state
    fn tokenize() {
        todo!()
    }

    pub fn run(&mut self) {
        println!("{}", self.source)
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

        location.update_offset(RND_OFFSET);

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
}
