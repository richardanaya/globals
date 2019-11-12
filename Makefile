build:
	@cargo build --target wasm32-unknown-unknown
lint:
	@cargo fmt
test:
	@cargo test -- --test-threads=1
