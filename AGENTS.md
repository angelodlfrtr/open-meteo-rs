# AGENTS.md

Guidance for coding agents working in this repository. Humans are welcome to read it too.

## What this is

A Rust client for the [Open-Meteo](https://open-meteo.com) weather API, as a cargo workspace:

| Crate             | Path                     | Role                                                                                 |
| ----------------- | ------------------------ | ------------------------------------------------------------------------------------ |
| `open-meteo-core` | `crates/open-meteo-core` | Request building and response parsing. No I/O. `#![no_std]` + `alloc`.               |
| `open-meteo-rs`   | `crates/open-meteo-rs`   | The `reqwest` client (async, plus `blocking` behind a feature). Re-exports the core. |

The core must stay free of `std` and of any HTTP dependency. Everything that opens a socket
belongs in `open-meteo-rs`.

## Commands

Run from the repository root. The nix dev shell (`nix develop`, or direnv) provides the toolchain.

```sh
make clippy        # pedantic lints, all features and all targets; must be clean
make test          # offline tests only; must pass before every commit
make test-live     # ignored smoke tests against the public API; run after touching a client or an endpoint
make check-no-std  # builds the core for wasm32v1-none; run after touching the core or its dependencies
make fmt           # cargo fmt + dprint (TOML and Markdown)
make fmt-check
```

CI runs `fmt-check`, `clippy`, `test` and `check-no-std`. It has no network access to the API.

## Layout of the core

- `request.rs`: the `Request` trait (`query()` + `parse()`) every options type implements, and `url()`.
- `query.rs`: the `Query` builder. All query strings go through it; do not format URLs by hand.
- `response.rs`: `RawResponse`, the envelope shared by all time-series endpoints, and the
  `TimeStep`/`DateStep`/`Measurement` records it pivots into. One parser for every endpoint.
- `endpoints.rs`: default base URL of every host, as consts and as the `Endpoints` struct.
- `units.rs`, `model.rs`: option enums with `Display` and `TryFrom<&str>`.
- One module per endpoint (`forecast.rs`, `marine.rs`, ...): an `Options` struct, a result
  struct, and the `Request` impl. Nothing else.

## Adding or changing an endpoint

1. Read the endpoint's page under <https://open-meteo.com/en/docs>. Use the host and path from
   that page, not from the overview; the overview lists wrong hosts.
2. Add a const and a field to `Endpoints`.
3. Write `crates/open-meteo-core/src/<name>.rs` with only the parameters that page documents.
   Required parameters are plain fields, optional ones are `Option`, lists are `Vec`, booleans
   are `bool` sent through `Query::flag`. Always send `timeformat=unixtime` for time-series
   endpoints; the parser depends on it.
4. Register the module in `lib.rs`, add a method to both `Client`s in `open-meteo-rs`
   (`src/client.rs` and `src/blocking.rs`; keep them identical apart from `async`/`.await`).
5. Record a fixture: call the real API once with `timeformat=unixtime`, pretty-print with `jq .`
   into `crates/open-meteo-core/tests/fixtures/<name>.json`. Keep it small.
6. Add a parse test in `tests/parse.rs` asserting concrete values, an exact-URL test in
   `tests/query.rs`, an ignored live test in `crates/open-meteo-rs/tests/live.rs`, a row in the
   README endpoint table, and a changelog entry.

## Conventions

- Rust 2021, MSRV in the root `Cargo.toml` (`rust-version`). Dependency versions live in
  `[workspace.dependencies]`; crates opt into features there.
- Clippy runs with `pedantic` denied. Fix the lint rather than allowing it. When an allow is
  truly needed, put a comment above it saying why.
- Public items have doc comments. Fallible functions document `# Errors`.
- Results keep values as raw `serde_json::Value`: the API mixes numbers, strings and nulls per
  variable and the caller knows what they asked for. Do not add per-variable typed fields.
- `BTreeMap`, not `HashMap`: the core has no `std`, and ordered output keeps tests deterministic.
- Error types are `thiserror` enums marked `#[non_exhaustive]`. Never `Box<dyn Error>`.
- Tests that hit the network are `#[ignore = "talks to the public API"]`. Keep `make test` offline.
- Commits follow Conventional Commits: `feat(endpoints): ...`, `fix(core): ...`, `docs: ...`,
  `chore(workspace): ...`. One logical change per commit, each one green on its own.
- The two crates share one version and are released together. Publish `open-meteo-core`
  before `open-meteo-rs`. Record breaking changes in `CHANGELOG.md`.
