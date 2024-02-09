use std::hint::black_box;
use tokio::runtime::Handle;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    load_test(foo).await;
}

async fn load_test(func: fn(i32) -> JoinHandle<i32>) {
    for i in 0..100_000 {
        let _res = func(i).await;
    }
}

fn foo(arg: i32) -> JoinHandle<i32> {
    let handle = Handle::current();
    handle.spawn(async move {
        black_box(arg * 2)
    })
}
