mod commands;

use std::fs;
use std::fs::File;
use std::io::{self, Write, Read};
use std::env;
use std::path::PathBuf;

fn main() {
    let user = env::var("USER").or_else(|_| env::var("USERNAME"));
    let mut username = String::new();
    match user {
        Ok(usernamel) => username = usernamel,
        Err(_) => println!("User get failed"),
    }

    let mut hydrosh_source = String::new();
    let mut pkg_config = String::new();
    let hydrosh_src_file_path = &format!("/home/{}/.config/hydrosh/hydrosh.hsh", username);
    let pkg_file_path = &format!("/home/{}/.config/hydrosh/pkg.cfg", username);
    match read_file(hydrosh_src_file_path){
        Ok(contents) => {
            hydrosh_source = contents;
        },
        Err(e) => {
            eprintln!("Error reading hydrosh source: {}", e);
        }
    }
    match read_file(pkg_file_path){
        Ok(contents) => {
            pkg_config = contents;
        },
        Err(e) => {
            eprintln!("Error reading pkg config: {}", e)
        }
    }
    for (index, line) in hydrosh_source.lines().enumerate() {
        commands::execute_command(line, &[]);
    }

    loop {
        let user = env::var("USER").or_else(|_| env::var("USERNAME"));
        let mut username = String::new();
        match user {
            Ok(usernamel) => username = usernamel,
            Err(_) => println!("User not resolved"),
        }

        let mut current_dir = PathBuf::new();
        match env::current_dir() {
            Ok(path) => current_dir = path,
            Err(e) => eprintln!("Error getting current dir: {}", e),
        }

        print!("hydrosh [{}]> ", username/*, current_dir.display()*/);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();
        if command.is_empty(){
            continue;
        }

        if (command == "exit" || command == "quit"){
            println!("Exiting shell...");
            break;
        }

        let parts: Vec<&str> = command.split_whitespace().collect();
        let cmd = parts[0];
        let args = &parts[1..];

        commands::execute_command(cmd, args);
    }    
}

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}