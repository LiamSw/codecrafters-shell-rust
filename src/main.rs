#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");

    let _ = String::new();
    io::stdin().read_line(&mut input);

    print!("{input}: command not found");

    io::stdout().flush().unwrap();
}
