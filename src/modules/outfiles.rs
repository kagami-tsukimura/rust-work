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

    f1.write_all(bytes).unwrap();

    let path2 = "output_sample2.txt";
    let mut f2 = File::create(path2).unwrap();
    writeln!(f2, "Hello, {}", "Rust").unwrap();

    println!("-----");
}
