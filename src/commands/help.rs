use crate::{commands, core::assets::parser};

pub fn run(args: &[String]) {
    let data = parser();

    match args.len() {
        0 => { println!("{}", data.description); }
        1 => {
            let command = &args[0];
            match data.commands.get(command){
                Some(cmd) => { println!("{}", cmd.description); }
                None => { println!("Unknown"); }
            }
        }
        _ => { println!("Too many arguments"); }
    }
}
