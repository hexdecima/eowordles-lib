{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-25.05";
    parts.url = "github:hercules-ci/flake-parts";
    rust = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs@{ nixpkgs, parts, rust, ... }:
    parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" ];

      perSystem = { system, ... }:
        let
          overlays = [ (import rust) ];
          pkgs = import nixpkgs { inherit system overlays; };
          toolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile
            ./rust-toolchain.toml;
        in {
          devShells.default =
            pkgs.mkShell { packages = with pkgs; [ toolchain just bacon nil nixfmt-classic taplo ]; };
        };
    };
}
