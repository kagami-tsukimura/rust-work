// cargo test main.rs
pub fn main() {
    test_sample();
}

#[test]
fn test_sample() {
    let a = 1 + 1;
    let b = 2;
    let c = 2 + 2;

    assert_eq!(a, b);
    assert_ne!(a, c);
    assert!(c == 4);
}
