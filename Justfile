all: normal boxed async boxed-async

build:
    cargo build --release

normal: build
    hyperfine target/release/normal-func-pointer

boxed: build
    hyperfine target/release/boxed-func-pointer

async: build
    hyperfine target/release/async-func


boxed-async: boxed-async-naive boxed-async-invert

boxed-async-naive: build
    hyperfine target/release/naive

# Identical to naive
boxed-async-closure: build
    hyperfine target/release/closure

# Not worth even running; crazy slow
boxed-async-spawn: build
    hyperfine target/release/spawn

boxed-async-invert: build
    hyperfine target/release/invert
