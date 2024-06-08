#[test]
// cargo test main.rs
fn test_eq() {
    let a = 1 + 1;
    let b = 2;

    assert_eq!(a, b);
}
#[test]
fn test_ne() {
    let a = 1 + 1;
    let c = 2 + 2;

    assert_ne!(a, c);
}
#[test]
fn test_vanilla() {
    let c = 2 + 2;

    assert!(c == 4);
}

// #[test]
// fn test_err() {
//     let c = 2 + 2;

//     assert!(c == 2);
// }

pub fn main() {}
