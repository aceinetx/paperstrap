{
  description = "paperstrap";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        rustToolchain = fenix.packages.${system}.complete.toolchain;
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "paperstrap";
          version = "1.0.0";

          src = self;

          cargoLock.lockFile = ./Cargo.lock;
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain
            cargo-expand
          ];
        };
      }
    );
}
