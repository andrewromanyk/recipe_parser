fmt:
	cargo fmt

clippy:
	cargo clippy

test:
	cargo test

build:
	cargo build

run:
	cargo run

all: fmt clippy test build