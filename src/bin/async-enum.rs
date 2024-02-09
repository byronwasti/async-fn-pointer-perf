use std::hint::black_box;

#[tokio::main]
async fn main() {
    load_test(Func::Foo).await;
}

async fn load_test(func: Func) {
    for i in 0..250_000_000 {
        func.run(i).await;
    }
}

async fn foo(arg: i32) -> i32 {
    black_box(arg * 2)
}

enum Func {
    Foo
}

impl Func {
    async fn run(&self, arg: i32) -> i32 {
        match self {
            Func::Foo => foo(arg).await,
        }
    }
}
