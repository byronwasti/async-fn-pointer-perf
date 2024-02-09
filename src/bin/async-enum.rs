use std::hint::black_box;

#[tokio::main]
async fn main() {
    let foo = Foo::Bar;

    for i in 0..250_000_000 {
        foo.run(i).await;
    }
}

async fn bar(arg: i32) -> i32 {
    black_box(arg * 2)
}

enum Foo {
    Bar,
}

impl Foo {
    async fn run(&self, arg: i32) -> i32 {
        match self {
            Foo::Bar => bar(arg).await,
        }
    }
}
