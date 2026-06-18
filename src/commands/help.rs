use crate::core::assets::parser;

pub fn run(args: &[String]) {
    let data = parser();

    let ext = match args.get(0) {
        Some(v) => v,
        None => {
            println!("{}", data.description);
            return;
        }
    };

    match ext.as_str() {
        "git" => println!("{}", data.description),
        _ => println!("Unknown command: {}", ext),
    }
}
