all: normal boxed async async-boxed-all
fast: normal boxed async-boxed-invert async-enum async

build:
    cargo build --release

normal: build
    hyperfine target/release/function-pointer

boxed: build
    hyperfine target/release/boxed-function-pointer

async: build
    hyperfine target/release/async-func


async-boxed-all: async-boxed-naive async-boxed-invert async-enum

async-boxed-naive: build
    hyperfine target/release/async-boxed-naive

# Identical to boxed-naive
async-boxed-closure: build
    hyperfine target/release/async-boxed-closure

# Not worth even running; crazy slow
async-boxed-spawn: build
    hyperfine target/release/async-boxed-spawn

async-boxed-invert: build
    hyperfine target/release/async-boxed-invert

async-enum: build
    hyperfine target/release/async-enum
