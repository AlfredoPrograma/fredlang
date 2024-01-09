use crate::tokenizer::parse::parse_token;

mod read;
mod scanner;
mod tokenizer;

fn main() {
    match read::from_file() {
        Ok(source) => {
            println!("{:#?}", parse_token(&source));
        }
        Err(err) => eprintln!("{err}"),
    }
}
