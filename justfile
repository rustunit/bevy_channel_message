check:
    cargo check
    cargo test
    cargo fmt --all -- --check
    cargo clippy -- -D warnings