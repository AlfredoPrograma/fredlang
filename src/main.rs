mod read;
mod scanner;

fn main() {
    match read::from_file() {
        Ok(source) => println!("{source}"),
        Err(err) => eprintln!("{err}"),
    }
}
