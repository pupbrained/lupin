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
      defaultBinName = "cli";

      pkgs = import nixpkgs {
        inherit system;
      };

      toolchain = fenix.packages.${system}.stable;

      naersk = pkgs.callPackage inputs.naersk {
        cargo = toolchain.cargo;
        rustc = toolchain.rustc;
      };
    in {
      defaultPackage = naersk.buildPackage {
        pname = defaultBinName;
        src = ./.;
      };

      devShell = pkgs.mkShell {
        nativeBuildInputs = [ toolchain.completeToolchain ];
      };
    });
}
