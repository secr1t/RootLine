mod commands;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: rl <command>");
        return;
    }

    let command = &args[1];

    commands::run(command);
}
