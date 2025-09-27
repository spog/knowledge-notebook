{
  description = "Knowledge Notebook Dev Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.defaultSystems (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlay ];
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            # Rust toolchain
            (pkgs.rust-bin.stable.latest.default)

            # Frontend toolchains
            pkgs.nodejs_20   # Node.js LTS
            pkgs.deno        # Deno runtime

            # Database tooling
            pkgs.postgresql_15

            # Docker (if you want local infra with containers)
            pkgs.docker
            pkgs.docker-compose
          ];

          shellHook = ''
            echo "🚀 Welcome to Knowledge Notebook Dev Environment"
            echo "Rust: $(rustc --version)"
            echo "Node: $(node --version)"
            echo "Deno: $(deno --version)"
            echo "psql: $(psql --version)"
          '';
        };
      });
}
