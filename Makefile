default: build

test: build
	cargo test --all --tests

build:
	stellar contract build --package mock-sep-41-token --optimize
	cp -f target/wasm32v1-none/release/mock_sep_41_token.wasm sep-41/src/testutils/mock_sep_41_token.wasm
	cargo build -p sep-41-token
	cargo build -p sep-41-token --features testutils

fmt:
	cargo fmt --all

clean:
	cargo clean
