use std::{env, fs, io::Read};

use prelude::Result;

mod interpreter;
mod parser;
mod prelude;
mod repl;
mod scanner;
mod tokens;

// TODO: refactor and move this function to somewhere else
fn read_source() -> Result<String> {
    let input_path = env::args()
        // Skips binary location default arg
        .skip(1)
        .next()
        .ok_or("need valid source code input path")?;

    let mut content = String::new();
    fs::File::open(input_path)?.read_to_string(&mut content)?;

    Ok(content)
}

fn main() -> Result<()> {
    repl::Repl::run();
    Ok(())
}
