.PHONY: lint
lint:
	cargo clippy --all-targets --all-features

.PHONY: build
build:
	cargo build --release