use std::io::{self, Write};

pub const if_help: &str = "\'if\' usage:\nif <comparator1> <comparator2> <command>\ne.g. if 1 1 echo hello\n";

pub fn show_help(help: &str){
    println!("{}", help);
    return;
}