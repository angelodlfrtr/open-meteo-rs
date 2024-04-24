{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.gcc
    pkgs.rustfmt
    pkgs.clippy
    pkgs.rust-analyzer
    pkgs.openssl
    pkgs.pkg-config

    # keep this line if you use bash
    pkgs.bashInteractive
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
