prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p liquidity_guard --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/liquidity_guard.wasm 2>/dev/null | true

build-test-contract:
	cargo build --release -p test --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/liquidity_guard_test.wasm 2>/dev/null | true

test-only:
	cargo test -p liquidity_guard_tests

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm liquidity_guard_tests/wasm

test: build-contract build-test-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf liquidity_guard_tests/wasm/*.wasm
