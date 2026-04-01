{
  description = "astro-aski — astrological chart software, domain model in aski";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, fenix, crane, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      toolchain = fenix.packages.${system}.stable.toolchain;
      craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

      src = craneLib.cleanCargoSource ./.;

      commonArgs = {
        inherit src;
        pname = "astro-aski";
        version = "0.1.0";
        nativeBuildInputs = with pkgs; [ pkg-config ];
        buildInputs = with pkgs; [ sqlite ];
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      astro-aski = craneLib.buildPackage (commonArgs // {
        inherit cargoArtifacts;
      });
    in
    {
      packages.${system}.default = astro-aski;

      devShells.${system}.default = craneLib.devShell {
        packages = with pkgs; [
          rust-analyzer
          pkg-config
          sqlite
        ];
      };
    };
}
