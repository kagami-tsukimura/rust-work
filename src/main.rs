mod modules {
    // pub mod basic;
    // pub mod flow;
    // pub mod fizz_buzz;
    // pub mod ownership;
    // pub mod refs;
    pub mod lifetimes;
}

fn main() {
    // modules::basic::main();
    // modules::flow::main();
    // modules::fizz_buzz::main(20);
    // modules::ownership::main();
    // modules::refs::main();
    modules::lifetimes::main();
}
