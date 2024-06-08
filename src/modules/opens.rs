use std::fs::OpenOptions;

pub fn main() {
    opens();
}

fn opens() {
    let file_path = "sample1.txt";

    let f = OpenOptions::new()
        // 追記
        .append(true)
        .open(file_path)
        .unwrap();
}
