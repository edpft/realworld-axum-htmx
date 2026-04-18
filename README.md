# realworld-axum-htmx

A [RealWorld](https://realworld-docs.netlify.app/) demo app built in Rust using [Axum](https://github.com/tokio-rs/axum) and [HTMX](https://htmx.org/). Built as a personal learning project following domain-driven design and hexagonal architecture principles.

## Prerequisites

- [Nix](https://nixos.org/download/) with flakes enabled
- [direnv](https://direnv.net/)

## Getting Started

Clone the repo, then allow direnv to load the Nix dev shell:

```bash
direnv allow
```

This will automatically install and configure all required tools.

## Running Tests

```bash
cargo nextest run
```

## References

- [RealWorld spec](https://realworld-docs.netlify.app/specifications/backend/introduction/)
- [Axum](https://docs.rs/axum/latest/axum/)
- [HTMX](https://htmx.org/docs/)
