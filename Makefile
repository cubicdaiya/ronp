build: src/*.rs
	cargo build --example strdiff

check: build
	cargo test

strdiff:
	cargo run --example $@ $(ARGS) # give such as ARGS="abc abd"

fmt:
	cargo fmt

clean:
	cargo clean
