use std::io;

pub fn main() {
    inputs();
}

fn inputs() {
    println!("Hello, inputs!");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("You typed: {}", input);
}
