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
        # cargo-llvm-cov needs llvm-cov and llvm-profdata from the same LLVM as rustc.
        llvm = pkgs.rustc.llvmPackages.llvm;
      in
      {
        formatter = pkgs.nixfmt;

        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            clippy
            rust-analyzer
            openssl
            pkg-config
            dprint
            gnumake
            cargo-llvm-cov
            # `make lint`
            actionlint
            zizmor
            typos
            cargo-deny
            cargo-machete
            nixfmt
          ];
          LLVM_COV = "${llvm}/bin/llvm-cov";
          LLVM_PROFDATA = "${llvm}/bin/llvm-profdata";
        };
      }
    );
}
