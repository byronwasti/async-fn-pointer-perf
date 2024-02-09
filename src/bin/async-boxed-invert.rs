use std::hint::black_box;
use std::future::Future;
use std::pin::Pin;

#[tokio::main]
async fn main() {
    load_test_outer(Box::pin(load_test(foo))).await;
}

async fn load_test_outer(test: Pin<Box<dyn Future<Output=()>>>) {
    test.await;
}

async fn load_test<T, F>(func: T)
where T: Fn(i32) -> F,
      F: Future<Output=i32>,
{
    for i in 0..250_000_000 {
        let _res = func(i).await;
    }
}

async fn foo(arg: i32) -> i32 {
    black_box(arg * 2)
}
