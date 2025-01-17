{
  description = "Rust environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { crane, fenix, flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-lMLAupxng4Fd9F1oDw8gx+qA0RuF7ou7xhNU8wgs0PU=";
        };
        pkgs = nixpkgs.legacyPackages.${system};
        ghc = pkgs.haskell.compiler.ghc910;
        haskellPackages = pkgs.haskell.packages.ghc910.override {
          overrides = final: prev: {
            # distribution-nixpkgs's tests are broken on ghc-9.10
            distribution-nixpkgs = pkgs.haskell.lib.dontCheck prev.distribution-nixpkgs;
          };
        };
      in
      {
        packages.default =
          let craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
          in craneLib.buildPackage {
            src = craneLib.cleanCargoSource ./.;
            doCheck = true;
          };
        formatter = pkgs.nixpkgs-fmt;
        devShells.default = pkgs.stdenv.mkDerivation {
          name = "develop environment";
          nativeBuildInputs = [
            toolchain
            haskellPackages.cabal-install
            haskellPackages.cabal2nix
            haskellPackages.haskell-language-server
          ];
        };
      }
    );
}
