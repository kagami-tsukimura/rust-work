pub fn main() {
    closures();
}

fn closures() {
    let c1 = |x: i32| x + 1;
    println!("{}", c1(10));

    let m = 10;
    // x + mの結果を固定
    let c2 = |x: i32| x + m;
    println!("{}", c2(10));

    let m = 20;
    // シャドーイングしても固定時から結果が変わらない
    println!("{}", c2(10));

    let v = vec![1, 2, 3];
    // move ||: vの所有権をclosureに移動
    // let c3 = move || {
    let c3 = || {
        println!("v: {:?}", v);
    };
    c3();
    println!("{:?}", v);
}
