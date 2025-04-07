install-dev-tools:
	@cargo install cargo-llvm-cov
	@echo "> Rust tools initialized"

bench:
	cargo bench -- --show-output

test-cov:
	cargo +nightly llvm-cov --open