{
  description = "RealWorld demo app in Rust using Axum and HTMX";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.mold
            pkgs.clang
            pkgs.bacon
            pkgs.cargo-nextest
            pkgs.pkg-config
            pkgs.postgresql
          ];

          shellHook = ''
            export RUSTFLAGS="-C linker=mold -C target-cpu=native"
          '';
        };
      }
    );
}