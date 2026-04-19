# 0003: Development Environment Setup

## Context
Developing in a consistent and reproducible environment is crucial for ensuring quality and reducing time spent on configuration issues. This document summarizes the tools and configuration used in this project.

## Tools and Setup

### Nix Flakes
Nix Flakes provides a mechanism for defining project dependencies and build specifications in a declarative manner. The `flake.nix` in the repo root defines the dev shell, including the Rust toolchain, mold linker, bacon, and cargo-nextest.

### direnv
`direnv` automatically loads the Nix dev shell when entering the project directory. The `.envrc` file in the project root enables this. Run `direnv allow` once after cloning.

### Mold Linker
For faster incremental build times, the mold linker is configured via `RUSTFLAGS` in the Nix dev shell:

```bash
RUSTFLAGS="-C linker=mold -C target-cpu=native"
```

This is set in `flake.nix` via `shellHook` rather than in `.cargo/config.toml`, so it applies only within the Nix dev environment and does not affect other users or CI unless they use the same shell.

### Cargo Nextest
`cargo-nextest` is the project's test runner. It runs tests in parallel and provides cleaner output than `cargo test`. It is included in the Nix dev shell. Run tests with:

```bash
cargo nextest run
```

### Bacon
`bacon` is a background build/test watcher. It is included in the Nix dev shell and provides instant feedback during development, which pairs well with a red-green-refactor workflow.

### Clippy Pedantic Lints
Clippy lints are configured once in the workspace `Cargo.toml` using the `[workspace.lints.clippy]` table, available since Rust 1.73. Each crate inherits them via `workspace = true` in its own `[lints]` table.

```toml
# Root Cargo.toml
[workspace.lints.clippy]
pedantic = "warn"

# Each crate's Cargo.toml
[lints]
workspace = true
```

## Conclusion
The dev environment is fully declarative via Nix Flakes and direnv. All tools are pinned and reproducible. Lint configuration lives in the workspace `Cargo.toml` and is inherited by all crates.
