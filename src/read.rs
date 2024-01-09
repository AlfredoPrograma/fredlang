use std::{env, error::Error, fmt, fs, result};

/// Contains data about errors which may occur while trying read source code
#[derive(Debug)]
#[allow(dead_code)]
pub struct ReadError {
    message: &'static str,
    source: Option<Box<dyn Error>>,
}

impl ReadError {
    pub fn new(message: &'static str, source: Option<Box<dyn Error>>) -> Self {
        Self { message, source }
    }
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[READ ERROR] {}", self.message)
    }
}

/// Result type which should be returned for all read functions
type ReadResult = result::Result<String, ReadError>;

/// Reads the path given as first argument of the execution and tries to read content of file in given path
pub fn from_file() -> ReadResult {
    let path = env::args()
        .nth(1)
        .ok_or(ReadError::new("missing source file path", None))?;

    let content = fs::read_to_string(path)
        .map_err(|err| ReadError::new("cannot read source code file", Some(Box::new(err))))?;

    Ok(content)
}
