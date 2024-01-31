use std::hint::black_box;
use std::future::Future;
use std::pin::Pin;
use tokio::runtime::Handle;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    let foo = Foo {
        func: bar,
    };

    // This is _super_ slow
    //for i in 0..250_000_000 {
    for i in 0..100_000 {
        let _res = (foo.func)(i).await;
    }
}

struct Foo {
    func: fn(i32) -> JoinHandle<i32>,
}

fn bar(arg: i32) -> JoinHandle<i32> {
    let handle = Handle::current();
    handle.spawn(async move {
        arg * 2
    })
}
