use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub fn main() {
    infiles();
}

fn infiles() {
    let file_path = "./sample1.txt";

    let mut f = File::open(file_path).expect("file not found");

    let mut conetnts = String::new();
    f.read_to_string(&mut conetnts)
        .expect("something went wrong reading the file");
    println!("{}", conetnts);

    println!("-----");

    let contents2 = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    println!("{}", contents2);
}
