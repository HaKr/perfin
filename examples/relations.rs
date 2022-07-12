#![allow(dead_code)]

use std::{collections::HashMap, fs::File};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Relations {
    relations: HashMap<String, Relation>,
}

#[derive(Deserialize, Debug)]
struct Relation {
    #[serde(default)]
    iban: Vec<String>,

    #[serde(default)]
    names: Vec<String>,
}

fn main() {
    let cfg = File::open("data/organisations/cb09add43080499a90e7479543e750a9/2018/relations.yaml")
        .unwrap();

    let relations: Relations = serde_yaml::from_reader(cfg).unwrap();
    println!("Relations: {:?}", relations);
}
