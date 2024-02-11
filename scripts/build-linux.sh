#!/bin/bash

rustup target add x86_64-unknown-linux-gnu 2> /dev/null
rustup target add aarch64-unknown-linux-gnu 2> /dev/null

cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu 

cp target/x86_64-unknown-linux-gnu/release/capti builds/capti-linux-x64
cp target/aarch64-unknown-linux-gnu/release/capti builds/capti-linux-arm64

