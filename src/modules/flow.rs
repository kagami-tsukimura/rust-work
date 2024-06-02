pub fn main() {
    sentence();
    branch();
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

    let y = 10;
    println!("{}", y);
    {
        // シャドーイング
        let y = 20;
        println!("{}", y);
    }
    println!("{}", y);

    let z = { 100 };
    println!("{}", z);
}

fn branch() {
    println!("-----");
    let x = -1;

    if x > 0 {
        println!("{} > 0", x);
    } else {
        println!("{} <= 0", x);
    }
}
