#!/bin/bash


rustup target add aarch64-apple-darwin 2> /dev/null
rustup target add x86_64-apple-darwin 2> /dev/null

cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

cp target/aarch64-apple-darwin/release/capti builds/capti-darwin-arm64
cp target/x86_64-apple-darwin/release/capti builds/capti-darwin-x64

