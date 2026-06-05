use std::io;
use std::process::Command;

pub fn run(){
    let mut comment = String::new();

    println!("Commit changes:"); 

    io::stdin().read_line(&mut comment);

    Command::new("git")
        .args(["add", "."])
        .status();

    Command::new("git")
        .args(["commit", "-m", comment.trim()])
        .status();

    Command::new("git")
        .arg("push")
        .status();
}
