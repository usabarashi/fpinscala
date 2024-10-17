{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=24.05";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (final: prev: { sbt = prev.sbt.overrideAttrs (oldAttrs: { postPatch = ""; }); }) ];
        pkgs = import nixpkgs { inherit overlays system; };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            sbt
            scala-cli
            temurin-bin-17
          ];
        };
      }
    );
}
