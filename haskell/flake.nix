{
  description = "haskell-experiment";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=24.05";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; with haskell; [
            ghc
            cabal-install
          ];
        };
      }
    );
}
