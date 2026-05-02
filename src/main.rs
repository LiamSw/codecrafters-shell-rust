#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::fs;
use is_executable::is_executable;

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
                    let key = "PATH";
                    match env::var_os(key) {
                        Some(paths) => {
                            'search: {
                                for path in env::split_paths(&paths) {
                                    let test_path = path.join(cmd_type);

                                    match fs::exists(&test_path) {
                                        Ok(true) => {
                                            if is_executable(&test_path) {
                                                println!("{cmd_type} is {}", test_path.display());
                                                break 'search;
                                            }
                                            else {continue;}
                                        }
                                        Ok(false) => continue,
                                        Err(e) => println!("Error checking path: {}", e),
                                    }
                                }
                            println!("{cmd_type}: not found");
                        }
                        }
                        None => println!("{key} is not defined in the environment.")
                    }
                }
            }
            _ => println!("{command}: command not found"),
        }
    }
}
