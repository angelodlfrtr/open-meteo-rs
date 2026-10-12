.PHONY: clippy
clippy:
	cargo clippy --workspace --all-features --all-targets
	cargo clippy --workspace --all-targets

.PHONY: test
test:
	cargo test --workspace --all-features

.PHONY: test-live
test-live:
	cargo test -p open-meteo-rs --all-features --test live -- --ignored --test-threads=2

.PHONY: check-no-std
check-no-std:
	cargo build -p open-meteo-core --target wasm32v1-none

.PHONY: fmt
fmt:
	cargo fmt --all
	dprint fmt

.PHONY: fmt-check
fmt-check:
	cargo fmt --all --check
	dprint check

.PHONY: coverage
coverage:
	cargo llvm-cov --workspace --all-features --ignore-filename-regex '(tests|examples)/' --fail-under-lines 100
