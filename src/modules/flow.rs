pub fn main() {
    sentence();
}

fn sentence() {
    println!("-----");
    {
        let x = 1;
        println!("{}", x);
    }
    println!("2");
    // // error: x is not in scope
    // println!("{}", x);
}
