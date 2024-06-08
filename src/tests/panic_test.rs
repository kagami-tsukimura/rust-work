fn maybe_panic(flag: bool) {
    if flag {
        panic!("ハンドリング");
    }
    println!("Safe!");
    // do something...
    panic!("unexpected!");
}

#[test]
#[should_panic]
fn test_maybe_panic() {
    maybe_panic(false);
    // maybe_panic(true);
}

pub fn main() {}
