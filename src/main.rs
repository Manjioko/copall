use cpa::parser_args;
use std::env;

fn main() {
    let mut args = Vec::new();

    for arg in env::args() {
        args.push(arg);
    }

    if let Err(e) = parser_args(args) {
        eprintln!("Application Error: {}", e)
    }
}
