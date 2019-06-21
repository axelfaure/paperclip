
all: fmt build test

clean:
	rm -rf Cargo.lock
	rm -rf target
	rm -rf openapi/tests/test_k8s
	git checkout openapi/tests/test_k8s
	rm -rf openapi/tests/test_pet

prepare:
	rustup override set nightly-2019-06-09
	rustup component add rustfmt
	rustup component add clippy

fmt:
	cargo fmt --all

doc:
	cargo doc --all --all-features --no-deps

build:
	cargo build
	cargo build --features default
	cargo build --all --all-features

test:
	cargo clippy --all -- -D clippy::all
	cargo test --all --all-features
	# Compile the code generated through tests.
	cd openapi/tests/test_pet && cargo build
	cd openapi/tests/test_k8s && cargo build
	cd openapi/tests/test_k8s/cli && CARGO_TARGET_DIR=../target cargo build
	# Test that the CLI runs successfully.
	./openapi/tests/test_k8s/target/debug/test-k8s-cli --help > /dev/null
