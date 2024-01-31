use std::hint::black_box;

fn main() {
    let foo = Foo {
        func: Box::new(foo),
    };

    for i in 0..250_000_000 {
        let _res = black_box((foo.func)(i));
    }
}

struct Foo {
    func: Box<fn(i32) -> i32>,
}

fn foo(arg: i32) -> i32 {
    arg * 2
}
