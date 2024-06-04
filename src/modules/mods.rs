mod test_module {
    pub mod sub_module1 {
        pub fn test_fn1() {
            println!("test_fn1-1");
        }

        fn test_fn2() {
            println!("test_fn1-2");
        }
    }

    mod sub_module2 {
        pub fn test_fn1() {
            println!("test_fn1-2");
        }

        fn test_fn2() {
            println!("test_fn2-2");
        }
    }
}

pub fn main() {
    mods();
}

fn mods() {
    // crate: main.rsからの絶対パスを指定
    // NOTE: main.rsから、modules/mods(.rs)の、test_moduleの、sub_module1の、test_fn1を呼び出す
    crate::modules::mods::test_module::sub_module1::test_fn1();
    // // 関数がprivateのためアクセス不可
    // crate::modules::mods::test_module::sub_module1::test_fn2();
    // // モジュールがprivateのためアクセス不可
    // crate::modules::mods::test_module::sub_module2::test_fn1();
    // // モジュール、関数がprivateのためアクセス不可
    // crate::modules::mods::test_module::sub_module2::test_fn2();
}
