use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

pub fn main() {
    jsons();
}

fn jsons() {
    let p = Person {
        name: String::from("Kagami Tsukimura"),
        age: 28,
        phones: vec![String::from("080-1234-5678"), String::from("090-1234-5678")],
    };
    let json_date = serde_json::to_string_pretty(&p).unwrap();
    println!("{}", json_date);
}
