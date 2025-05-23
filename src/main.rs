use std::{
    env::{self, args},
    fs::read_to_string,
    io::BufReader,
    process::exit,
};

use scanner::Scanner;
use token::Token;
mod scanner;
mod token;
mod tokentype;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);


    if args.len() > 2 {
        println!("Using : jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        run_file(args[1].clone());
    } else {
        println!("pass the file path");
    }
}

fn run_file(path: String) {
    let data = read_to_string(path);
    run(data.unwrap());
}

fn run(bytes: String) {
    let scanner: Scanner = Scanner::new(bytes);
    let tokens: Vec<Token> = scanner.scanTokens();
    println!("{:#?}",tokens);
}
