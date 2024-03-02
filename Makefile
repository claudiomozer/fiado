.PHONY: setup
setup:
	cargo install sqlx-cli samply

.PHONY: lint
lint:
	cargo clippy --all-targets --all-features

.PHONY: build
build:
	cargo build --release

.PHONY: profiling
profiling:
	samply record cargo run 
