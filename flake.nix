{
  inputs = {
    naersk = {
      url = "github:nix-community/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    fenix,
    ...
  } @ inputs:
    utils.lib.eachDefaultSystem (system: let
      defaultBinName = "lupin_cli";
      pkgs = import nixpkgs {inherit system;};
      toolchain = with fenix.packages.${system};
        fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-jvoZwAQuaeLQbrd77FCVwHP57XC27RQ9yWMxF/Pc0XY=";
        };

      naersk = pkgs.callPackage inputs.naersk {
        cargo = toolchain;
        rustc = toolchain;
      };
    in {
      defaultPackage = naersk.buildPackage {
        pname = defaultBinName;
        src = ./.;
      };

      devShell = pkgs.mkShell {
        nativeBuildInputs = [toolchain];
      };
    });
}
