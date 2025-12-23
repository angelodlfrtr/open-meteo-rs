.PHONY: clippy
clippy:
	cargo clippy

.PHONY: test
test:
	cargo test

.PHONY: fmt
fmt:
	dprint fmt

.PHONY: fmt-check
fmt-check:
	dprint check
