# AGENTS.md

Instructions for AI agents working on this project.

## Build Environment

**Always use `nix develop -c` as a prefix for every cargo, trunk, or pnpm command.** The Nix
flake provides all system dependencies (ffmpeg, alsa, libclang) needed to build
the project.

```bash
# Correct
nix develop -c cargo check --workspace
nix develop -c cargo build --release
nix develop -c cargo test --workspace

# Wrong — will fail on systems without all deps installed
cargo check --workspace
```

All quality checks should be run as:

```bash
nix develop -c cargo fmt --all --check
nix develop -c cargo check --workspace
nix develop -c cargo test --workspace
nix develop -c cargo clippy --all-targets --workspace -- -D warnings
```

## Workspace Layout

| Crate           | Type | Description                                           |
| --------------- | ---- | ----------------------------------------------------- |
| `artscii-core`  | lib  | Shared config, errors, dithering strategies           |
| `artscii-img`   | lib  | Image loading and ASCII conversion                    |
| `artscii-video` | lib  | Video decoding and ASCII encoding (needs ffmpeg/alsa) |
| `artscii-cli`   | bin  | CLI binary named `artscii` (not `artscii-cli`)        |
| `artscii-web`   | WASM | Leptos CSR SPA built with Trunk                       |

Rust edition is **2024** across all crates.

## Feature Flags

- `artscii-core` has a `cli` feature (adds `clap`). Enabled transitively by
  `artscii-img/cli`.
- `artscii-img/cli` enables `artscii-core/cli` + the `colored` crate for ANSI
  output.
- `artscii-cli` has a `video` feature (default on) that pulls in
  `artscii-video`.
- `artscii-web` uses `artscii-img` with `default-features = false` to skip the
  `colored`/ANSI dependency.

## Web Crate (artscii-web)

Built with Leptos (CSR mode) and Trunk. Requires the WASM target:

```bash
rustup target add wasm32-unknown-unknown
```

Dev server:

```bash
nix develop -c trunk serve --open
```

Only CLI crate tests and formatting apply to `artscii-web`. There are no WASM
tests.

## Gotchas

- `nix build` only sees **git-tracked** files. New files must be committed (or
  `git add`-ed) before they're visible to the Nix build.
- The video crate tests need ffmpeg/alsa at runtime; Nix provides these.
- `cargo test -p <crate>` runs tests for a single crate (useful when working on
  one library).
- There is no `rust-toolchain.toml`; CI uses `stable` via
  `dtolnay/rust-toolchain@stable`.
