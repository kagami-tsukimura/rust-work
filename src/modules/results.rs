pub fn main() {
    results();
}

fn results() {
    let result = divide(10, 2);
    match result {
        Ok(x) => println!("Result: {}", x),
        Err(e) => println!("Error: {}", e),
    }
}
