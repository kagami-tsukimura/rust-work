use std::env;
pub fn main() {
    args();
}

fn args() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
