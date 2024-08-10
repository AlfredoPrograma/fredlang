use std::io::{stdin, stdout, Write};

use crate::prelude::execute_code;

pub struct Repl;

impl Repl {
    pub fn run() {
        Self::print_welcome();

        let mut input_buffer = String::new();

        loop {
            Self::print_cursor();
            Self::read_from_line(&mut input_buffer);

            if let Ok(output) = execute_code(&input_buffer) {
                println!("{}", output)
            }

            input_buffer.clear()
        }
    }

    fn print_welcome() {
        println!(
            r#"
    ______              ____                     ____  __________  __ 
   / ____/_______  ____/ / /___ _____  ____ _   / __ \/ ____/ __ \/ / 
  / /_  / ___/ _ \/ __  / / __ `/ __ \/ __ `/  / /_/ / __/ / /_/ / /  
 / __/ / /  /  __/ /_/ / / /_/ / / / / /_/ /  / _, _/ /___/ ____/ /___
/_/   /_/   \___/\__,_/_/\__,_/_/ /_/\__, /  /_/ |_/_____/_/   /_____/
                                    /____/
    "#
        );
    }

    fn print_cursor() {
        print!("> ");
        stdout().flush().expect("cannot flush stdout");
    }

    fn read_from_line(buf: &mut String) {
        stdin().read_line(buf).expect("cannot read from stdin");
    }
}
