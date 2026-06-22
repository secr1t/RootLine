use std::collections::HashMap;
use std::fs;
use std::process::Command;


use serde::Deserialize;


#[derive(Deserialize)]
pub struct HelpData{
   pub common: String,
   pub description: String,

   pub commands: HashMap<String, Commands>,
}


#[derive(Deserialize)]
pub struct Commands{
    pub usage: String,
    pub  description: String,

    pub args: Option<HashMap<String, String>>,
}

pub fn parser() -> HelpData{
    let text = fs::read_to_string("assets/help.json").unwrap();

    let info: HelpData = serde_json::from_str(&text).unwrap();

    info
}
