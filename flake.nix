{
  description = "A flake for a Kile and LaTeX development environment";

  inputs = {
    nixpkgs.url = "github:Nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: 
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        pkgs.mkShellNoCC {
          buildInputs = [ pkgs.bashInteractive ];
          packages = with pkgs; [
            kile
            texlive.scheme-full
          ];
          shellHook = ''
            export SHELL=${pkgs.lib.getExe pkgs.bashInteractive}
          '';
        };
      };
    });
}

