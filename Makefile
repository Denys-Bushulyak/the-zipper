BASELINE=master

install-dev-tools:
	@cargo install cargo-llvm-cov
	@echo "> Rust tools initialized"

bench:
	git switch ${BASELINE}; \
	cargo bench -- --save-baseline ${BASELINE}; \

	git switch -; \
	cargo bench; \
	cargo bench -- --load-baseline new --baseline ${BASELINE} --show-output

test-cov:
	cargo +nightly llvm-cov --open
