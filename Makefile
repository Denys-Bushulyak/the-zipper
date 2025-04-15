install-dev-tools:
	@cargo install cargo-llvm-cov
	@echo "> Rust tools initialized"

bench:
	@read -p "Enter baseline branch:" baseline; \
	if ! git rev-parse --verify $$baseline > /dev/null 2>&1; then \
		echo "Branch $$baseline does not exist"; \
		exit 1; \
	fi; \
	git switch ${baseline}; \
	cargo bench -- --save-baseline $$baseline

	git switch -
	cargo bench; \
	cargo bench -- --load-baseline new --baseline $$baseline --show-output

test-cov:
	cargo +nightly llvm-cov --open
