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

    if x > 0 && x < 10 {
        println!("1: {} > 0 && {} < 10", x, x);
    } else {
        println!("2: {} > 0 || {} < 10", x, x);
    }

    let y = if x > 10 { x } else { 0 };
    println!("y: {}", y);
}
