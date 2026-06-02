pub mod ip;

pub fn run(cmd: &str) {
    match cmd {
        "ip" => ip::run(),
        _ => println!("Unknown command: {}", cmd),
    }
}
