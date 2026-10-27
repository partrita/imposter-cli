# AGENTS.md

## Project

`imposter` is a Rust CLI that fakes an AI coding session. Satire only: it never
modifies files and never calls a model.

## Build & verify

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
cargo build --release --no-default-features   # tiny, zero-dependency core
```

## Architecture

- `src/main.rs` — module wiring, style dispatch, first-run setup, deadline handling.
- `src/transcript.rs` — transcript engine: pacing, turn plans, content decks.
- `src/style/mod.rs` — `Style` enum, `Skin` data, the `Renderer` trait (with
  generic defaults), `patch()`, and shared box helpers.
- `src/style/<name>.rs` — one file per AI CLI: its `Skin` plus only the layout
  hooks that differ from the default. `claude` is the reference (no overrides).
- `src/classic.rs` — `classic` style: legacy system-log renderer.
- `src/tui.rs` — ratatui setup wizard, gated behind the `tui` feature (on by default).
- `src/content.rs`, `src/jargon.rs` — static copy pools, consumed via `Deck`.
- `src/rng.rs` — xorshift64* PRNG and shuffled `Deck` (no repeats per cycle).
- `src/ansi.rs` — ANSI rendering, spinner, wrapping, Windows VT enablement.
- `src/cli.rs`, `src/config.rs`, `src/setup.rs`, `src/types.rs` — arguments, config, prompts, enums.

## Conventions

- Keep the core zero-dependency: no `clap`, `serde`, `rand`, `tokio`, etc.
  New runtime crates must be optional and feature-gated.
- Performance matters: reuse buffers, one `write_all` per line, no threads,
  no alternate screen in normal output mode.
- Any new generated text must come from a `Deck` so output does not repeat
  within a deck cycle. Add new copy to the existing pools in `content.rs`.
- Add a style in three steps: a `Style` variant (plus `ALL`, `as_str`, `parse`,
  `renderer`), a new `src/style/<name>.rs` with a `Skin` and `impl Renderer`,
  then declare the module. Start by overriding only `skin()`; every other hook
  has a working default. `Style` is the single selector; there is no `--mode`.
- Maintain Linux, macOS, and Windows support; avoid platform-specific APIs
  without a `cfg` fallback.
- CI runs on all three OSes; release artifacts are built by
  `.github/workflows/release.yml` on `v*` tags.

## Documentation

Update `README.md` for user-facing changes. Keep the original author credit to
[giacomo-b/rust-stakeholder](https://github.com/giacomo-b/rust-stakeholder).
