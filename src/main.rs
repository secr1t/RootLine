mod commands;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: rl <command> [subcommand] [args...]");
        return;
    }

    commands::run(&args[1..]);
}

// Useless comment
