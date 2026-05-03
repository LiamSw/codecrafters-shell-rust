#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
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

fn parse(input: &str) -> Vec<String>{
    let values = input.trim().chars();
    let mut vector = Vec::new();
    let mut single_quote = false;
    let mut temp_string = String::new();

    for c in values {
        match c {
            '\'' => single_quote = !single_quote,
            ' ' if !single_quote => {
                    if !temp_string.is_empty() {
                        vector.push(std::mem::take(&mut temp_string));
                    }
                }
            _ => temp_string.push(c),
        }
    }

    if !temp_string.is_empty() {vector.push(temp_string);}

    vector
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let split = parse(&input);
        if split.is_empty() {continue;}

        let command = split[0].as_str();
        let args = &split[1..];
        let recognized_com = ["echo", "type", "exit", "pwd"];

        match command {
            "exit" => std::process::exit(0),
            "echo" => println!("{}", args.join(" ")),
            "type" => {
                if args.is_empty() {continue;}
                let cmd_type = &args[0].as_str();
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
            "cd" => {
                if args.is_empty() {continue;}
                let mut path = args[0].to_string();

                if path == "~" {
                    if let Ok(home) = env::var("HOME") {path=home;}
                }

                let change_dir = env::set_current_dir(&path).is_ok();
                if !change_dir {
                    println!("cd: {}: No such file or directory", args[0]);
                }
            }
            _ => {
                if let Some(_path) = find_path(command) {
                    let _status = Command::new(command)
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
