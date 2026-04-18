# 0003: Development Environment Setup

## Context
Developing in a consistent and reproducible environment is crucial for ensuring quality and reducing time spent on configuration issues. This document summarizes how to set up a development environment using various tools that streamline the process.

## Tools and Setup
### Nix Flakes
Nix Flakes provides a mechanism for defining project dependencies and build specifications in a declarative manner. To use Nix flakes, ensure that you have Nix installed, and then create a `flake.nix` file in the root of your project:

```nix
{# flake.nix #}
{
  description = "A flake for a Rust project using Axum and HTMX";

  inputs = {
    # Define your dependencies here
  };

  outputs = { self, nixpkgs }: {
    # Define your package outputs here
  };
}
```

### direnv
`direnv` allows you to automatically load environment variables depending on your directory. After installing `direnv`, create an `.envrc` file in your project root:

```bash
# .envrc
use nix
```

Run `direnv allow` to enable it.

### Mold Linker
For faster build times, configure your project to use the mold linker by adding the following to your `.cargo/config.toml`:

```toml
[target.x86_64-unknown-linux-gnu]
linker = "mold"
```

### Cargo Nextest
To speed up your testing process, integrate `cargo-nextest`, a test runner that can run tests in parallel and cache results. Install it using:

```bash
cargo install cargo-nextest
```

You can run your tests in parallel with:
```bash
cargo nextest run
```

### Bacon
`bacon` can be used to run your tests in a way that helps with visibility into test execution. Ensure you have set it up according to its documentation.

### Clippy Pedantic Lints
To maintain code quality, we recommend running Clippy with the pedantic lints flag. Add this configuration to your `Cargo.toml`:

```toml
[profile.dev]
lints = "pedantic"
```

## Conclusion
By following the above setup, you can achieve a robust and efficient development environment that enhances productivity and code quality.