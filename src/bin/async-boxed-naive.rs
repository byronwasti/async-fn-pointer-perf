use std::hint::black_box;
use std::future::Future;
use std::pin::Pin;

#[tokio::main]
async fn main() {
    load_test(foo).await;
}

async fn load_test(func: fn(i32) -> Pin<Box<dyn Future<Output=i32>>>) {
    for i in 0..250_000_000 {
        let _res = func(i).await;
    }
}

fn foo(arg: i32) -> Pin<Box<dyn Future<Output=i32>>> {
    Box::pin(async move {
        black_box(arg * 2)
    })
}
