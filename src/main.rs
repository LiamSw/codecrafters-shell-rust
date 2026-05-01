#[allow(unused_imports)]
use std::io::{self, Write};

fn not_found(cmd: &str) {
    println!("{cmd}: command not found");
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let split = input.split_whitespace().collect::<Vec<_>>();
        if (split.is_empty()) {continue;}

        let command = split[0];
        let args = &split[1..];
        let recognized_com = ["echo", "type", "exit"];

        match command {
            "exit" => std::process::exit(0),
            "echo" => println!("{}", args.join(" ")),
            "type" => {
                if (args.len > 1) {std::process::exit(0);} 

                if (recognized_com.contains(&args[0])) {
                    println!("{command} is a shell builtin");
                } else {
                    not_found(command);
                }
            }
            _ => not_found(command),
        }
    }
}
