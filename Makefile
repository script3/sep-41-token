default: build

test: build
	cargo test --all --tests

build:
	cargo rustc --manifest-path=mock-sep-41/Cargo.toml --crate-type=cdylib --target=wasm32v1-none --release
	stellar contract optimize \
		--wasm target/wasm32v1-none/release/mock_sep_41_token.wasm \
		--wasm-out sep-41/src/testutils/mock_sep_41_token.wasm
	cargo build -p sep-41-token
	cargo build -p sep-41-token --features testutils

fmt:
	cargo fmt --all

clean:
	cargo clean
