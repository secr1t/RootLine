
pub mod ip;
pub mod git;

pub fn run(args: &[String]) {
    let cmd = &args[0];

    match cmd.as_str() {
        "ip" => ip::run(&args[1..]),
        "git" => git::run(&args[1..]),
        _ => println!("Unknown command: {}", cmd),
    }
}
