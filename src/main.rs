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
        if split.is_empty() {continue;}

        let command = split[0];
        let args = &split[1..];
        let recognized_com = ["echo", "type", "exit"];

        match command {
            "exit" => std::process::exit(0),
            "echo" => println!("{}", args.join(" ")),
            "type" => {
                let cmd_type = &args[0];
                if recognized_com.contains(cmd_type) {
                    println!("{cmd_type} is a shell builtin");
                } else {
                    not_found(cmd_type);
                }
            }
            _ => not_found(command),
        }
    }
}
