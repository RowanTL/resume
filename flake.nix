{
  description = "A flake for a Kile and LaTeX development environment";

  inputs = {
    nixpkgs.url = "github:Nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        overlays = [ (import rust-overlay) ];
      in
      {
        devShells.default = pkgs.mkShell {
          name = "latex-env";

          buildInputs = with pkgs; [
            kile
            texlive.combined.scheme-full
            openssl
            pkg-config
            (rust-bin.stable.latest.default.override {
              targets = [ "wasm32-unknown-unknown" ];
            })
            trunk
          ];
        };
      }
    );
}

