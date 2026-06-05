use std::io;
use std::process::Command;

pub fn run(_args: &[String]){
    let mut comment = String::new();

    println!("Commit changes:"); 

    io::stdin().read_line(&mut comment).unwrap();

    Command::new("git")
        .args(["add", "."])
        .status()
        .unwrap();

    Command::new("git")
        .args(["commit", "-m", comment.trim()])
        .status()
        .unwrap();

    Command::new("git")
        .arg("push")
        .status()
        .unwrap();
}
