{
  description = "A flake for creating a resume with latex and kile all locally";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { nixpkgs, ... }:
  let system = "x86_64-linux"; in
  {
    devShells."${system}".default =
    let
      pkgs = nixpkgs.legacyPackages."${system}";
    in
      pkgs.mkShellNoCC {
        buildInputs = [ pkgs.bashInteractive ];
        packages = with pkgs; [
          texliveFull
          kile
        ];
        shellHook = ''
          export SHELL=${pkgs.lib.getExe pkgs.bashInteractive}
        '';
      };
  };
}
