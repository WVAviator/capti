#!/bin/bash

OS=$(uname -s)
ARCH=$(uname -m)

if [ "$OS" != "Windows" ]; then
  echo "This script is only for Windows"
  exit 1
fi

if [ "$ARCH" != "x86_64" ]; then
  echo "This script is only for x86_64"
  exit 1
fi

cargo build --release

cp target/release/capti.exe builds/capti-win32-x64.exe

