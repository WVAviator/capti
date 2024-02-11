cargo install cross --git https://github.com/cross-rs/cross 2>/dev/null


echo "Building for all targets"

cross build --release --target aarch64-apple-darwin
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target aarch64-unknown-linux-gnu 
cross build --release --target x86_64-apple-darwin
cross build --release --target x86_64-pc-windows-gnu
