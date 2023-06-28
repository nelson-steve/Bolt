#![allow(dead_code)]
#![allow(non_snake_case)]

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    dbg!(args);
}
