// cargo test -- --test-threads=1: シングルスレッド実行
// default: マルチスレッド実行

fn maybe_panic(flag: bool) {
    if flag {
        panic!("flag is true!!!");
    }
    println!("Safe!");
    // do something...
    panic!("unexpected!");
}

#[test]
#[should_panic(expected = "flag is true")]
fn test_maybe_panic() {
    // maybe_panic(false);
    maybe_panic(true);
}

pub fn main() {}
