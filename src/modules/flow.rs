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

    // シャドーイング
    let y = 10;
    println!("{}", y);
    {
        let y = 20;
        println!("{}", y);
    }
    println!("{}", y);
}
