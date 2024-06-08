use std::fs::File;
use std::io::prelude::*;

pub fn main() {
    outfiles();
}

fn outfiles() {
    let path1 = "output_sample1.txt";
    let mut f1 = File::create(path1).unwrap();
    let bytes = b"write example!\n";
    println!("{:?}", bytes);

    f1.write(bytes).unwrap();

    println!("-----");
}
