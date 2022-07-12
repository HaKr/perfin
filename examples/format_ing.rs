use std::{collections::HashMap, fs::File};

use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Aliases {
    aliases: HashMap<String, String>,
}

fn main() {
    let cfg = File::open("data/formats/ing.yaml").unwrap();
    let aliases: Aliases = serde_yaml::from_reader(cfg).unwrap();
    let aliases: HashMap<String, Regex> = aliases
        .aliases
        .iter()
        .map(|(key, re)| (key.to_string(), Regex::new(re).unwrap()))
        .collect();
    println!("Aliases: {:?}", aliases)
}
