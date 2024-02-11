
rustup target add x86_64-pc-windows-gnu 2> /dev/null

cargo build --release --target x86_64-pc-windows-gnu

cp target/x86_64-pc-windows-gnu/release/capti.exe builds/capti-windows-x64.exe

