build:
    cargo build
run:
    ./target/debug/study
test:
    cargo test
test-with-output:
    cargo test -- --show-output
linting:
    cargo fmt
    cargo clippy
