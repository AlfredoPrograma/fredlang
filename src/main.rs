mod read;

fn main() {
    match read::from_file() {
        Ok(source) => println!("{source}"),
        Err(err) => eprintln!("{err}"),
    }
}
