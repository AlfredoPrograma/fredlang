use scanner::Scanner;

mod read;
mod scanner;
mod tokenizer;

fn main() {
    match read::from_file() {
        Ok(source) => {
            let mut scanner = Scanner::new(source.as_str());

            scanner.run()
        }
        Err(err) => eprintln!("{err}"),
    }
}
