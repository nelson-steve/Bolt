#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::env;
use std::process::exit;
use std::fs;
use std::io::{self, BufRead, Write};

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path){
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&contents),
    }
}

fn run(_contents: &str) ->Result<(), String> {
    return Err("Not impl".to_string());
}

fn run_prompt() -> Result<(), String>{
    print!("> ");
    let mut buffer = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer){
        Ok(_) => (),
        Err(_) => return Err("Couldn't read line".to_string()),
    }
    println!("Your wrote: {}", buffer);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2{
        println!("Usage: bolt[script]");
    } else if args.len() == 2{
        match run_file(&args[1]){
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR:\n{}", msg);
                exit(1);
            }
        }
    } else {
        match run_prompt(){
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR:\n{}", msg);
                exit(1);
            }
        }
    }
}
