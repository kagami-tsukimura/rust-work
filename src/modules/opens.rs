use std::fs::OpenOptions;

pub fn main() {
    opens();
}

fn opens() {
    let file_path1 = "sample1.txt";
    let f1 = OpenOptions::new()
        // 追記
        .append(true)
        .open(file_path1)
        .unwrap();

    let file_path2 = "sample2.txt";
    let f2 = OpenOptions::new()
        // 上書き
        .write(true)
        // ファイルが存在すればエラー
        .create_new(true)
        .open(file_path2)
        .unwrap();
}
