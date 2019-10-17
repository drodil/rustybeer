extern crate getopts;
use getopts::Options;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    println!("{}", program);
}
