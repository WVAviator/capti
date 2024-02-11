#!/bin/bash

OS=$(uname -s)
ARCH=$(uname -m)

if [ "$OS" != "Linux" ]; then
  echo "This script is only for Linux"
  exit 1
fi

if [ "$ARCH" != "x86_64" ]; then
  echo "This script is only for x86_64"
  exit 1
fi

rustup target add x86_64-unknown-linux-gnu 2> /dev/null
rustup target add aarch64-unknown-linux-gnu 2> /dev/null

dpkg -l grep -qw build-essential || sudo apt-get install build-essential
dpkg -l grep -qw gcc-aarch64-linux-gnu || sudo apt-get install gcc-aarch64-linux-gnu

PACKAGES=(gcc-aarch64-linux-gnu g++-aarch64-linux-gnu gcc g++ make cmake build-essential)

for package in "${PACKAGES[@]}"; do
  dpkg -l grep -qw $package || sudo apt-get install $package
done

cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu 

cp target/x86_64-unknown-linux-gnu/release/capti builds/capti-linux-x64
cp target/aarch64-unknown-linux-gnu/release/capti builds/capti-linux-arm64

