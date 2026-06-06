use std::io;
use std::process::Command;

pub fn run(args: &[String]) {
    let ext = &args[0];

    match ext.as_str(){
        "push" => push(),
         _ => println!("Unknown command: {}", ext),

    }
}

fn push(){
    let mut comment = String::new();

    print!("Commit changes"); 

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
