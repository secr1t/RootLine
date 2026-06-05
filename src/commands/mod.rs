pub mod ip;
pub mod push;

pub fn run(cmd: &str) {
    match cmd {
        "ip" => ip::run(),
        "push" => push::run(),
        _ => println!("Unknown command: {}", cmd),
    }
}
