{
  description = "Basic rust dev environment";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    unstable.url = "nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.utils.follows = "utils";
    };
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay, unstable }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          overlays = [ rust-overlay.overlays.default ];
          localSystem = { inherit system; };
        };
        unstablePkgs = import unstable {
          localSystem = { inherit system; };
        };
        naersk-lib = naersk.lib."${system}";

        rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
          extensions = [ "rust-src" ];
        };
      in
      rec {
        # nix build
        packages.default = naersk-lib.buildPackage {
          pname = "shared-memory";
          root = ./shared-memory;
        };

        # nix run
        apps.default = utils.lib.mkApp {
          drv = packages.default;
        };

        # nix develop
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            python310
            rustToolchain
            unstablePkgs.rust-analyzer
            wabt
            wasm-bindgen-cli
            wasm-pack
          ];
        };
      }
    );
}
