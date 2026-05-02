#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::fs;
use is_executable::is_executable;
use std::process::Command;
use std::path::PathBuf;

fn find_path(command: &str) -> Option<PathBuf> {
    let key = "PATH";
    let paths = env::var_os(key)?;
    
    for path in env::split_paths(&paths) {
        let test_path = path.join(command);
        if test_path.exists() && is_executable(&test_path) {
            return Some(test_path);
        }
    }
    None
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
        let recognized_com = ["echo", "type", "exit", "pwd"];

        match command {
            "exit" => std::process::exit(0),
            "echo" => println!("{}", args.join(" ")),
            "type" => {
                let cmd_type = &args[0];
                if recognized_com.contains(cmd_type) {
                    println!("{cmd_type} is a shell builtin");
                } else if let Some(path) = find_path(cmd_type) {
                    println!("{cmd_type} is {}", path.display());
                } else {
                    println!("{cmd_type}: not found");
                }
            }
            "pwd" => {
                let curr_dir = env::current_dir()
                    .expect("Failed to get current directory");
                    println!("{}", curr_dir.display());
            }
            _ => {
                if let Some(path) = find_path(command) {
                    let status = Command::new(command)
                        .args(args)
                        .status()
                        .expect("Failed to execute command");
                } else {
                    println!("{command}: command not found");
                }
            }
        }
    }
}
