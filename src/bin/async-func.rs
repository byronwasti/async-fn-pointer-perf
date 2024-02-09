//! Baseline of async performance
use std::hint::black_box;

#[tokio::main]
async fn main() {
    load_test().await;
}

async fn load_test() {
    for i in 0..250_000_000 {
        foo(i).await;
    }
}

async fn foo(arg: i32) -> i32 {
    black_box(arg * 2)
}
