use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let writer = BufWriter::new(stdout.lock());
    say(&message, width, writer).unwrap();
}
