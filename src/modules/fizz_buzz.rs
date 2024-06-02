pub fn main(count: i32) {
    fizzbuzz_for(count);
    println!("-----");
    fizbuzz_while(count);
    println!("-----");
    fizbuzz_match(count);
}

fn fizzbuzz_for(count: i32) {
    for i in 1..=count {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz")
        } else if i % 3 == 0 {
            println!("Fizz")
        } else if i % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", i)
        }
    }
}

fn fizbuzz_while(count: i32) {
    let mut i = 1;
    while count > i {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz")
        } else if i % 3 == 0 {
            println!("Fizz")
        } else if i % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", i)
        }
        i += 1;
    }
}

fn fizbuzz_match(count: i32) {
    for i in 1..=count {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", i),
        }
    }
}
