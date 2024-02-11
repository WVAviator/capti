# SUPPORTED_PLATFORMS = ['linux', 'darwin', 'win32'];
# SUPPORTED_ARCHS = ['x64', 'arm64'];

rustup target add aarch64-apple-darwin 2> /dev/null
rustup target add x86_64-apple-darwin 2> /dev/null
# rustup target add x86_64-unknown-linux-gnu 2> /dev/null
# rustup target add aarch64-unknown-linux-gnu 2> /dev/null
# rustup target add x86_64-pc-windows-gnu 2> /dev/null


echo "Building for all targets"

cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
# cross build --release --target x86_64-unknown-linux-gnu
# cross build --release --target aarch64-unknown-linux-gnu 
# cross build --release --target x86_64-pc-windows-gnu

cp target/aarch64-apple-darwin/release/capti builds/capti-darwin-arm64
cp target/x86_64-apple-darwin/release/capti builds/capti-darwin-x64
# cp target/x86_64-unknown-linux-gnu/release/capti builds/capti-linux-x64
# cp target/aarch64-unknown-linux-gnu/release/capti builds/capti-linux-arm64
# cp target/x86_64-pc-windows-gnu/release/capti.exe builds/capti-windows-x64.exe
