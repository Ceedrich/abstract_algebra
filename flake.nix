{
  description = "rust flake";

  inputs = {
    nigpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        pkgs = import nixpkgs
          {
            inherit system;
            overlays = [
              (import inputs.rust-overlay)
              (final: prev: {
                my-rust = prev.rust-bin.stable.latest.default.override
                  {
                    inherit extensions;
                  };
                my-rust-nightly = prev.rust-bin.nightly.latest.default.override
                  {
                    inherit extensions;
                  };
              })
            ];
          };
      in
      {
        devShells.default = pkgs.mkShell rec {
          name = "rust";
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [
            # my-rust
            my-rust-nightly
            cargo-expand
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
      }
    );
}
