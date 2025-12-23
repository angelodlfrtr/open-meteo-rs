.PHONY: clippy
clippy:
	cargo clippy --all-targets --all-features

.PHONY: test
test:
	cargo test

.PHONY: fmt
fmt:
	dprint fmt
