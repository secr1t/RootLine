use std::io::{self, Write};
use std::process::Command;

pub fn run(args: &[String]) {
    let ext = &args[0];

    match ext.as_str(){
        "push" => push(),
         _ => println!("Unknown command: {}", ext),

    }
}
fn push(){

    print!("Commit message. In case if comment would be empty would be used default preset: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let comment = match input.trim() {
        "" => {
            println!("Empty commit usage. Using default preset");
            "Update project"
            }
        c => c,
    };
   
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
