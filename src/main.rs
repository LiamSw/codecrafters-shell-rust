#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let split = input.split_whitespace().collect::<Vec<_>>();
        if (split.is_empty()) continue;

        let command = split[0];
        let args = &split[1..];

        match command {
            "exit" => std::process::exit(0),
            "echo" => println!("{}", args.join(" ")),
            _ => println!("{command}: command not found"),
        }
    }
}
