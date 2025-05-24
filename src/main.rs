use std::{
    env::{self, args},
    fs::read_to_string,
    io::{stdin, stdout, BufRead, BufReader, Write},
    process::exit,
};

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use token::Token;

mod interpreter;
mod scanner;
mod token;
mod tokentype;
mod expr;
mod parser;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    let interpreter = Interpreter::new();
    if args.len() > 2 {
        println!("Using : jlox [script]");
        exit(64);
    } else if args.len() == 2 {
  //      run_file(args[1].clone());
    } else {
        run_prompt();
    }
}
/*
fn run_file(path: String) {
    let data = read_to_string(path);
    run(data.unwrap());
}*/
fn run_prompt()->Result<(),String>{
    let mut interpreter = Interpreter::new();

    loop {
        print!(">> ");
        match stdout().flush() {
            Ok(_)=>(),
            Err(_)=> return Err("could not not flush stdout".to_string()),
        }
        let mut buffer = String::new();
        let stdin = stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(n)=>{
                if n<=1 {
                    return Ok(());
                }
            }
            Err(_)=> return Err("coudn't read line".to_string()),
        }
        print!("");
        match run(&mut interpreter,buffer) {
            Ok(_)=>(),
            Err(m)=>println!("{}",m),
        }
    }
}

fn run(interpreter: &mut Interpreter, bytes: String) ->Result<(),String>{
    let scanner: Scanner = Scanner::new(bytes);
    let tokens: Vec<Token> = scanner.scanTokens();
    let mut parser = Parser::new(tokens);
    let expr = parser.parse()?;
    let res = expr.eval()?;
    println!("{}",res.to_string());
    //println!("{:#?}",tokens);
    Ok(())
}
