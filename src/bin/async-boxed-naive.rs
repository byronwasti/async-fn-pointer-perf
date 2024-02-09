use std::hint::black_box;
use std::future::Future;
use std::pin::Pin;

#[tokio::main]
async fn main() {
    let foo = Foo {
        func: bar,
    };

    for i in 0..250_000_000 {
        let _res = (foo.func)(i).await;
    }
}

type BoxedFut = Pin<Box<dyn Future<Output = i32> + Send>>;

struct Foo {
    func: fn(i32) -> BoxedFut,
}

fn bar(arg: i32) -> BoxedFut {
    Box::pin(async move {
        arg * 2
    })
}
