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

#[cfg(test)]
mod test_module {
    #[test]
    #[should_panic(expected = "flag is true")]
    fn test_maybe_panic() {
        // maybe_panic(false);
        super::maybe_panic(true);
    }

    #[test]
    fn test_calc_add() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn test_calc_diff() {
        assert_eq!(1 - 1, 0);
    }
}

pub fn main() {}
