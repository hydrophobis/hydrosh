mod help;

//use libc::{getpwnam, setuid, uid_t};
use std::env;
use std::process::{Command, Stdio};
use std::vec::Vec;

pub fn execute_command(cmd: &str, args: &[&str]){
    match cmd {
        "if" => if_cmd(args),
        "pkg" => pkg_cmd(args),
        "run" => src_cmd(args),
        "cal" => cal_cmd(args),
        "cd" => cd_cmd(args).unwrap(),
        "perm" => perm_cmd(args),
        // "su" => su_cmd(args),
        _ => run_ext_cmd(cmd, args),
    }
}

// fn su_cmd(args: &[&str]){
//     switch_user(args[0]);
// }

fn perm_cmd(args: &[&str]){
    let mut arg;
    match args[0] {
        "exec" => arg = "+x",
        "static" => arg = "-x",
        "write" => arg = "+w",
        "rx" => arg ="-w",
        "read" => arg = "+r",
        "blank" => arg = "-r",
        _ => arg = args[0],
    }
    execute_command("chmod", &[arg, args[1]])
}

fn if_cmd(args: &[&str]){
    if (args[0] == "-h"){
        help::show_help(help::if_help);
        return;
    }

    let condition = args[0] == args[1];
    if condition {
        let command = args[2];
        let cmd_args = &args[3..];

        execute_command(command, cmd_args);
    }
}

fn src_cmd(args: &[&str]){
    execute_command(&format!("./{}", args[0]), &[""]);
}

fn pkg_cmd(args: &[&str]){
    if args[0] == "install" {
        let mut cmd_args: Vec<&str> = Vec::new();
        cmd_args.push("pacman");
        cmd_args.push("-S");
        for arg in args {
            if arg != &"install"{
                cmd_args.push(arg);
            }
        }
        execute_command("sudo", &cmd_args);
    } else {
        for arg in args {
            run_ext_cmd("whereis", &[arg]);
        }
    }
}

fn cd_cmd(args: &[&str]) -> Result<(), String>{
    if args[0].is_empty() {
        let home_dir = env::var("HOME").unwrap_or_else(|_| String::from("/"));
        return env::set_current_dir(home_dir).map_err(|e| e.to_string());
    }
    env::set_current_dir(args[0]).map_err(|e| e.to_string())?;

    Ok(())
}

fn cal_cmd(args: &[&str]){
    cd_cmd(args);
    execute_command("ls", &[]);
}

fn run_ext_cmd(cmd: &str, args: &[&str]){
    let output = Command::new(cmd)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output();

    if let Err(e) = output {
        eprintln!("Error: {}", e)
    }
}

// fn switch_user(username: &str){
//     let c_username = CString::new(username).map_err(|_| "Invalid username".to_string())?;

//     unsafe {
//         let user_info = getpwnam(c_username.as_ptr());
//         if user_info.is_null(){
//             return Err(format!("User {} not found"))
//         }
//         let uid = (*user_info).pw_uid;
//         if setuid(uid) == 0 {
//             Ok(())
//         } else {
//             Err("User switch failed".to_string())
//         }
//     }
// }

fn check_len(args: &[&str], length: usize){
    if args.len() < length {
        eprintln!("missing arguments");
        return;
    }
}