# Development tasks for the open-meteo-rs workspace.
#
# Run `make` or `make help` to list the targets. Lines ending in `## text` are the help.
# Everything runs from the repository root inside the nix dev shell (`nix develop` or direnv).

CARGO ?= cargo
WORKSPACE := --workspace --all-features
NO_STD_TARGET := wasm32v1-none
COVERAGE_IGNORE := '(tests|examples)/'

.DEFAULT_GOAL := help

##@ Checks

.PHONY: ci
ci: lint fmt-check clippy test check-no-std coverage doc ## Everything CI runs, in order

.PHONY: check
check: fmt-check clippy test ## The quick pre-commit set: format, lints, offline tests

.PHONY: lint
lint: lint-workflows lint-nix lint-typos lint-deps ## Lint the CI workflow, the flake, spelling and the dependency tree

.PHONY: lint-workflows
lint-workflows: ## actionlint (syntax, expressions, shellcheck) and zizmor (security hygiene) on the workflows
	actionlint
	zizmor --min-severity low .github/workflows

.PHONY: lint-nix
lint-nix: ## Check flake.nix formatting and that the flake evaluates
	nixfmt --check flake.nix
	nix flake check --no-build

.PHONY: lint-typos
lint-typos: ## Spell-check sources, docs and configs
	typos

.PHONY: lint-deps
lint-deps: ## cargo-deny (advisories, licences, sources) and cargo-machete (unused dependencies)
	cargo deny check
	cargo machete

.PHONY: fmt-check
fmt-check: ## Verify formatting of Rust, TOML and Markdown without changing anything
	$(CARGO) fmt --all --check
	dprint check

.PHONY: clippy
clippy: ## Pedantic lints, with and without the `blocking` feature
	$(CARGO) clippy $(WORKSPACE) --all-targets
	$(CARGO) clippy --workspace --all-targets

.PHONY: test
test: ## Offline tests: recorded API answers, exact URLs, mock HTTP server
	$(CARGO) test $(WORKSPACE)

.PHONY: test-live
test-live: ## Smoke tests against the public API (ignored by default; two threads to avoid bursts)
	$(CARGO) test -p open-meteo-rs --all-features --test live -- --ignored --test-threads=2

.PHONY: check-no-std
check-no-std: ## Build the core for a target without a standard library
	$(CARGO) build -p open-meteo-core --target $(NO_STD_TARGET)

.PHONY: coverage
coverage: ## Line coverage with cargo-llvm-cov; fails below 100%
	$(CARGO) llvm-cov $(WORKSPACE) --ignore-filename-regex $(COVERAGE_IGNORE) --fail-under-lines 100

.PHONY: coverage-html
coverage-html: ## Coverage report as HTML, opened in the browser
	$(CARGO) llvm-cov $(WORKSPACE) --ignore-filename-regex $(COVERAGE_IGNORE) --html --open

.PHONY: doc
doc: ## Build the API docs with warnings denied, as docs.rs would
	RUSTDOCFLAGS="-D warnings" $(CARGO) doc $(WORKSPACE) --no-deps

##@ Development

.PHONY: fmt
fmt: ## Format Rust, TOML and Markdown
	$(CARGO) fmt --all
	dprint fmt

.PHONY: build
build: ## Build every crate, feature and target
	$(CARGO) build $(WORKSPACE) --all-targets

.PHONY: doc-open
doc-open: doc ## Build the API docs and open them in the browser
	$(CARGO) doc $(WORKSPACE) --no-deps --open

.PHONY: examples
examples: ## Run every example against the public API
	$(CARGO) run -p open-meteo-rs --example forecast
	$(CARGO) run -p open-meteo-rs --example archive
	$(CARGO) run -p open-meteo-rs --example blocking --features blocking

.PHONY: clean
clean: ## Remove build artifacts
	$(CARGO) clean

##@ Release

.PHONY: package
package: ## Dry-run packaging of both crates, as `cargo publish` would see them
	$(CARGO) package --workspace --allow-dirty

.PHONY: publish
publish: ci ## Publish both crates to crates.io in dependency order, after a full `make ci`
	$(CARGO) publish --workspace

##@ Help

.PHONY: help
help: ## Show this help
	@awk 'BEGIN { FS = ":.*## "; printf "Usage: make <target>\n" } \
	      /^##@/ { printf "\n%s\n", substr($$0, 5) } \
	      /^[a-zA-Z0-9_-]+:.*## / { printf "  %-16s %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
