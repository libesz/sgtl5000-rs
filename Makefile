build:
	cargo build --verbose

build-examples:
	cargo build --release --examples --features=real-hal --target=thumbv7em-none-eabihf

test:
	cargo test --features=mock-hal --verbose

clippy:
	cargo clippy
	cargo clippy --examples --features=real-hal --target=thumbv7em-none-eabihf