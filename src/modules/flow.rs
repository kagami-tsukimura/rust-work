pub fn main() {
    sentences();
    branches();
    matches();
    loops();
}

fn sentences() {
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

fn branches() {
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

fn matches() {
    println!("-----");
    // let x = 1;
    let x = 2;
    // let x = 3;
    match x {
        1 => println!("1"),
        2 => {
            println!("2-1");
            println!("2-2");
        }
        _ => println!("other"),
    }
}

fn loops() {
    println!("-----");
    // loop: 無限ループ
    let mut cnt_loop = 0;
    loop {
        cnt_loop += 1;
        if cnt_loop > 10 {
            break;
        } else if cnt_loop % 2 == 0 {
            continue;
        }
        println!("Hello: {}", cnt_loop);
    }

    println!("-----");
    // while: 条件付きループ
    let mut cnt_while = 0;
    while cnt_while < 10 {
        cnt_while += 1;
        if cnt_while % 2 == 0 {
            continue;
        }
        println!("Hello: {}", cnt_while);
    }

    println!("-----");
    // for: ループ
    for cnt_for in 0..10 {
        if cnt_for % 2 == 0 {
            continue;
        }
        println!("Hello: {}", cnt_for);
    }
}
