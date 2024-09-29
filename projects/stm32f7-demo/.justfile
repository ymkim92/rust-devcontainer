build:
    cargo build --target thumbv7em-none-eabihf
build-release:
    cargo build --release --target thumbv7em-none-eabihf
clean:
    cargo clean
convert:
    rust-objcopy -O binary target/thumbv7em-none-eabihf/debug/stm32f7-demo stm32f7-demo-debug.bin
convert-release:
    rust-objcopy -O binary target/thumbv7em-none-eabihf/release/stm32f7-demo stm32f7-demo-release.bin
    # arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/stm32f7-demo stm32f7-demo-debug.bin
run:
    ./target/debug/study
test:
    cargo test
test-with-output:
    cargo test -- --show-output
objdump:
    rust-objdump --section-headers target/thumbv7em-none-eabihf/debug/stm32f7-demo
linting:
    cargo fmt
    cargo clippy
