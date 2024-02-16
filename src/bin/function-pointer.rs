//! Baseline of function pointer performance
use std::hint::black_box;

fn main() {
    load_test(black_box(foo));
}

fn load_test(func: fn(i32) -> i32) {
    for i in 0..250_000_000 {
        let _res = func(i);
    }
}

fn foo(arg: i32) -> i32 {
    black_box(arg * 2)
}
