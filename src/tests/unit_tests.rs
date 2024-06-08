#[cfg(test)]
#[test]
// cargo test main.rs
fn test_eq() {
    let a = 1 + 1;
    let b = 2;

    assert_eq!(a, b);
}
#[cfg(test)]
#[test]
fn test_ne() {
    let a = 1 + 1;
    let c = 2 + 2;

    assert_ne!(a, c);
}
#[cfg(test)]
#[test]
fn test_vanilla() {
    let c = 2 + 2;

    assert!(c == 4);
}

#[cfg(test)]
// #[test]
// fn test_err() {
//     let c = 2 + 2;

//     assert!(c == 2);
// }

pub fn main() {}
