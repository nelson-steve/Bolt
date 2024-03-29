#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod scanner;
mod expr;
mod parser;
mod interpreter;
mod stmt;
mod resolver;
mod tests;
mod environment;
use crate::scanner::*;
use crate::parser::*;
use crate::interpreter::*;

use core::num;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process::exit;
use std::result;

fn run_file(path: &str) -> Result<(), String> {
    let mut interpreter = Interpreter::new(); 
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&mut interpreter, &contents),
    }
}

pub fn run(interpreter: &mut Interpreter, contents: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let stmts = parser.parse()?;
    interpreter.interpret(stmts.iter().map(|b| b).collect())?;
    return Ok(());
}

fn run_prompt() -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    loop {
        print!("> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Could not flush stdout".to_string()),
        }
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(n) => {
                dbg!(n);
                if n <= 2 {
                    return Ok(());
                }
            }
            Err(_) => return Err("Couldn't read line".to_string()),
        }
        println!("ECHO: {}", buffer);
        match run(&mut interpreter, &buffer) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        // println!("Usage: bolt[script]");
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR: {}", msg);
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR:{}", msg);
                exit(1);
            }
        }
    }
}
