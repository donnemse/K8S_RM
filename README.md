export OPENSSL_DIR="$(brew --prefix openssl)"
cargo build --release --target=x86_64-unknown-linux-musl