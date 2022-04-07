build: src/*.rs
	cargo build --example strdiff
	cargo build --example intdiff

check: build
	cargo test

strdiff:
	cargo run --example $@ $(ARGS) # give such as ARGS="abc abd"

intdiff:
	cargo run --example $@

fmt:
	cargo fmt

clean:
	cargo clean
