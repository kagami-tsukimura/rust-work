use std::fs::File;
use std::io::prelude::*;

pub fn main() {
    infiles();
}

fn infiles() {
    let mut f = File::open("./sample1.txt").expect("file not found");

    let mut s = String::new();
    f.read_to_string(&mut s)
        .expect("something went wrong reading the file");
    println!("{}", s);
}
