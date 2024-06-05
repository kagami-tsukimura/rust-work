pub fn main() {
    genericses();
}

fn genericses() {
    println!("{}", max(1, 2));
    println!("{}", max(1.0, 2.3));
    println!("{}", max("hoge", "huga"));
}

// generics境界: T: PartialOrdで引数が比較可能な型に制限する
fn max<T: PartialOrd>(a: T, b: T) -> T
// where T: PartialOrd
// NOTE: where T: PartialOrdは、<T: PartialOrd>と同様。
// NOTE: 単純な制約なら<>で、複数の制約をつけたい等で複雑な制約ならwhereを使う。
{
    if a >= b {
        a
    } else {
        b
    }
}
