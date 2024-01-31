use std::hint::black_box;
use std::future::Future;
use std::pin::Pin;
use futures::future::{BoxFuture};

#[tokio::main]
async fn main() {
    let foo = Foo {
        func: |arg| Box::pin(bar(arg)),
    };

    for i in 0..250_000_000 {
        let _res = (foo.func)(i).await;
    }
}

struct Foo<'a, T> {
    func: fn(i32) -> BoxFuture<'a, T>,
}

async fn bar(arg: i32) -> i32 {
    arg * 2
}
