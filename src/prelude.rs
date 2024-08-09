use std::error::Error;

use crate::{
    interpreter::{runtime::Interpreter, Output},
    parser::{ast::Parser, expressions::AST},
    scanner::Scanner,
};

/// Wrapper over [`std::error::Result`] enum to automatically inject dynamic Error trait.
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn execute_code(source: &str) -> Result<Output> {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();
    let mut ast = AST::new(scanner.tokens.into_iter());
    let expressions = ast.parse();

    Interpreter::evaluate(expressions.unwrap())
}
