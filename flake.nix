{
  description = "The typed-command-builder crate";
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-23.05";
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    crane = {
      url = "github:ipetkov/crane/v0.12.2";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    cargo-changelog = {
      url = "github:matthiasbeyer/cargo-changelog";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
        crane.follows = "crane";
        rust-overlay.follows = "rust-overlay";
      };
    };
  };

  outputs = { nixpkgs, crane, flake-utils, rust-overlay, cargo-changelog, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
              (import rust-overlay)
            ];
        };

        rustTarget = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (crane.mkLib pkgs).overrideToolchain rustTarget;

        tomlInfo = craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; };
        inherit (tomlInfo) version;

        pname = "typed-command-builder";
        src =
          let
            nixFilter = path: _type: !pkgs.lib.hasSuffix ".nix" path;
            extraFiles = path: _type: !(builtins.any (n: pkgs.lib.hasSuffix n path) [ ".github" ".sh" ]);
            filterPath = path: type: builtins.all (f: f path type) [
              nixFilter
              extraFiles
              pkgs.lib.cleanSourceFilter
            ];
          in
          pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = filterPath;
          };

        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src pname;
        };

        typed-command-builder = craneLib.buildPackage {
          inherit cargoArtifacts;
          inherit pname;
          inherit src;
          inherit version;

          cargoExtraArgs = "--all-features";
        };
      in
      rec {
        checks = {
          inherit typed-command-builder;

          typed-command-builder-clippy = craneLib.cargoClippy {
            inherit cargoArtifacts src pname;
            cargoClippyExtraArgs = "--benches --examples --tests --all-features -- --deny warnings";
          };

          typed-command-builder-fmt = craneLib.cargoFmt {
            inherit src pname;
          };

          typed-command-builder-tests = craneLib.cargoNextest {
            inherit cargoArtifacts src pname;
          };
        };

        packages = {
          default = packages.typed-command-builder;
          inherit typed-command-builder;
        };

        devShells.default = devShells.typed-command-builder;
        devShells.typed-command-builder = pkgs.mkShell {
          buildInputs = [];

          nativeBuildInputs = [
            rustTarget

            pkgs.cargo-deny
            pkgs.gitlint
            cargo-changelog.packages."${system}".changelog
            pkgs.statix
          ];
        };
      }
    );
}
