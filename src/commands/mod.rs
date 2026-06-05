use core::str;
use std::env::args;

pub mod ip;
pub mod push;

pub fn run(args: &[String]) {
    let cmd = &args[0];

    match cmd.as_str() {
        "ip" => ip::run(&args[1..]),
        "push" => push::run(&args[1..]),
        _ => println!("Unknown command: {}", cmd),
    }
}
