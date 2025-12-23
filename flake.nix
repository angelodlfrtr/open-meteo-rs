{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      crane,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.mkLib pkgs;
      in
      {
        formatter = pkgs.nixfmt-rfc-style;

        devShell = craneLib.devShell {
          packages = with pkgs; [
            clippy
            rust-analyzer
            openssl
            pkg-config
          ];
        };
      }
    );
}
