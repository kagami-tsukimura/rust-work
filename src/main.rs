mod modules {
    // pub mod basic;
    // pub mod flow;
    // pub mod fizz_buzz;
    // pub mod ownership;
    // pub mod refs;
    // pub mod smart_pointer;
    // pub mod structs;
    // pub mod enums;
    // pub mod options;
    // pub mod crates;
    // pub mod mods;
    // pub mod sub_struct;
    // pub mod binaries;
    // pub mod traits;
    // pub mod defaults;
    // pub mod lib_args;
    // pub mod derive;
    // pub mod genericses;
    // pub mod more_generics;
    // pub mod panics;
}

fn main() {
    // modules::basic::main();
    // modules::flow::main();
    // modules::fizz_buzz::main(20);
    // modules::ownership::main();
    // modules::refs::main();
    // modules::smart_pointer::main();
    // modules::structs::main();
    // modules::enums::main();
    // modules::options::main();
    // modules::crates::main();
    // modules::mods::main();
    // modules::sub_struct::main();
    // modules::binaries::main();
    // modules::traits::main();
    // modules::defaults::main();
    // modules::lib_args::main();
    // modules::derive::main();
    // modules::genericses::main();
    // modules::more_generics::main();
    // modules::panics::main();

    match_modules();
    is_modules();
    transfer();
}

fn need_even(a: i32) -> Result<i32, String> {
    if a % 2 == 0 {
        Ok(a)
    } else {
        Err(String::from("引数は偶数にしてください。"))
    }
}

fn match_modules() {
    println!("{:?}", need_even(10));
    println!("{:?}", need_even(5));
    let x = match need_even(10) {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            panic!()
        }
    };
    println!("{}", x);
    println!("-----");
}

fn is_modules() {
    let s = need_even(1);
    println!("{}", s.is_ok());
    println!("{}", s.is_err());

    // 所有権の移動あり
    // println!("{:?}", s.ok());
    // println!("{:?}", s.err());

    // println!("{:?}", s.unwrap_or(0));
    // println!("{:?}", s.unwrap());

    // println!("{:?}", s.expect("expectから発生"));
    println!("-----");
}

fn transfer() {
    match double_even(4) {
        Ok(val) => println!("{}", val),
        Err(err) => {
            println!("mainでハンドリング");
            println!("{}", err);
        }
    }
    // println!("{:?}", double_even(2));
    // println!("{:?}", double_even(1));
}

fn double_even(b: i32) -> Result<i32, String> {
    // match need_even(b) {
    //     Ok(val) => Ok(val * 2),
    //     Err(err) => Err(err),
    // }

    // ?演算子: 委譲
    let x = need_even(b)?;
    Ok(x * 2)
}
