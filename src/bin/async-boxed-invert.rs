use std::hint::black_box;
use std::future::Future;
use std::pin::Pin;

#[tokio::main]
async fn main() {
    let foo = Foo {
        func: Box::pin(benchmark(bar)),
    };

    foo.func.await;
}

type BoxedFut = Pin<Box<dyn Future<Output = ()> + Send>>;

struct Foo {
    func: BoxedFut,
}

async fn bar(arg: i32) -> i32 {
    black_box(arg * 2)
}

async fn benchmark<T: Fn(i32) -> F, F: Future<Output=i32>>(func: T) {
    for i in 0..250_000_000 {
        func(i).await;
    }
}
