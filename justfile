run-dev:
    cargo run

test:
    cargo test



build-optimized: test
    cargo build --optimized



