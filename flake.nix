{
  description = "Configurable Rust + Deno Dev Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # Import nixpkgs with rust-overlay
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        # 🔧 Config section: change these to bump versions
        rustVersion = "1.90.0";  # stable Rust pinned
        denoPackage = pkgs.deno_2_5_1;  # pinned Deno (from nixpkgs)

      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            # Rust pinned via overlay
            (pkgs.rust-bin.stable."${rustVersion}".default)

            # Deno pinned via nixpkgs
            denoPackage
          ];

          shellHook = ''
            echo "🚀 Dev Environment Ready!"
            rustc --version
            deno --version
          '';
        };
      });
}
