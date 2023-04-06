{
  description = "lupin: a programming language";

  inputs = {
    naersk = {
      url = "github:nix-community/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    rust-overlay,
    ...
  } @ inputs:
    utils.lib.eachDefaultSystem (system: let
      defaultBinName = "lupin_cli";

      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };

      toolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
        toolchain.default.override {
          extensions = [ "rust-analyzer" ];
          targets = [ ];
        });

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
        buildInputs = [ toolchain ];
      };
    });
}
