# Contributing

## Principles

This project follows domain-driven design and hexagonal architecture. The four crates map directly to the architecture:

- `domain` — domain model, value objects, aggregates. No dependencies on other crates.
- `application` — use cases and ports (traits). Depends only on `domain`.
- `infrastructure` — port implementations (database, token generation, etc.). Depends on `application` and `domain`.
- `web` — HTTP layer (Axum handlers). Depends on `application` and `domain`.

Keep these boundaries strict. Infrastructure and framework concerns must not leak into `domain` or `application`.

## Workflow

- All changes go through a pull request — do not push directly to `main`.
- CI must pass before merging. The CI pipeline runs `check`, `fmt`, `clippy`, `cargo-deny`, and `nextest`.
- Write tests before or alongside implementation (TDD). Do not open a PR with untested application logic.
- Each PR should have a clear, single purpose. Split unrelated changes into separate PRs.

## Running CI Locally

```bash
cargo check --workspace
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo deny check
cargo nextest run --workspace
```

## Architecture Decisions

Significant design decisions are recorded as ADRs in [`.github/adr`](.github/adr). If your PR makes a meaningful architectural choice, add or update an ADR.

## Commit Messages

Use the imperative mood and keep the subject line under 72 characters. Reference relevant issues or ADRs in the body where appropriate.