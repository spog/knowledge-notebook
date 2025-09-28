{
  description = "Minimal Rust + Deno Dev Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            (pkgs.rust-bin.stable.latest.default)  # Rust toolchain
            pkgs.deno                              # Deno runtime
          ];

          shellHook = ''
            echo "🚀 Minimal Dev Environment Ready!"
            rustc --version
            deno --version
          '';
        };
      });
}
