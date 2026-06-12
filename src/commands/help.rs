use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Help {
    usage: String,
    description: String,
}

pub fn run(args: &[String]){

   reader();
 


}

fn reader() {
    let text = match fs::read_to_string("src/assets/help.json") {
        Ok(text) => text,
        Err(err) => {
            println!("Failed to open help.json: {}", err);
            return;
        }
    };

    let help: Help = match serde_json::from_str(&text) {
        Ok(data) => data,
        Err(err) => {
            println!("Invalid JSON: {}", err);
            return;
        }
    };

    println!("Usage: {}", help.usage);
    println!("Description: {}", help.description);

}
