use core::range;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::os::linux::raw::stat;
use bytes::buf::Writer;
use is_executable::is_executable;
use std::process::Command;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::fs::File;

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
    let mut values = input.trim().chars().peekable();
    let mut vector = Vec::new();
    let mut single_quote = false;
    let mut double_quote = false;
    let mut temp_string = String::new();
    let db_quote_special = ['"', '\\', '$', '`', 'n'];

    while let Some(c) = values.next() {
        match c {
            '\\' => {
                if single_quote {
                    temp_string.push(c);
                    continue;
                } else if let Some(&c_next) = values.peek() {
                    let special = db_quote_special.contains(&c_next);
            
                    if !double_quote || (double_quote && special) {
                        temp_string.push(values.next().unwrap());
                    } else {
                        temp_string.push(c);
                    }
                } else {
                    temp_string.push(c);
                }
            }
            '\'' if !double_quote => single_quote = !single_quote,
            '"' if !single_quote => double_quote = !double_quote,
            ' ' if !single_quote && !double_quote => {
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

fn output(args: &mut Vec<String>) -> Option<File> {
    if let Some(i) = args.iter().position(|x| x == ">" || x == "1>" || x == "2>") {
        if let Some(file) = args.get(i+1) {
            let file =  File::create(file).expect("No such file or directory"); 
            args.drain(i..);
            return Some(file); 
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
        
        let mut split = parse(&input);
        if split.is_empty() {continue;}

        let out = output(&mut split);
        let command = split[0].as_str();
        let args = &split[1..];
        let recognized_com = ["echo", "type", "exit", "pwd"];

        match command {
            "echo" | "type" | "pwd" => {
                let mut writer: Box<dyn Write> = match out {
                    Some(ref f) => Box::new(f.try_clone().expect("failed clone")), 
                    None => Box::new(io::stdout()), 
                };

                if command == "echo" {writeln!(writer, "{}", args.join(" ")).expect("error");}
                if command == "type" {
                    if args.is_empty() {continue;}
                    let cmd_type = &args[0].as_str();
                    if recognized_com.contains(cmd_type) {
                        writeln!(writer, "{cmd_type} is a shell builtin").expect("error");
                    } else if let Some(path) = find_path(cmd_type) {
                        writeln!(writer, "{cmd_type} is {}", path.display()).expect("error");
                    } else {
                        writeln!(writer, "{cmd_type}: not found").expect("error");
                    }
                }
                if command == "pwd" {
                    let curr_dir = env::current_dir()
                        .expect("Failed to get current directory");
                        writeln!(writer, "{}", curr_dir.display()).expect("error");
                }

            }
            "exit" => std::process::exit(0),
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
                    let mut pth = Command::new(command);
                        pth.args(args);

                        if let Some(f) = out {
                            pth.stdout(f);
                        }

                        let _status = pth.status().expect("Failed to execute command");
                } else {
                    println!("{command}: command not found");
                }
            }
        }
    }
}