{pkgs ? import <nixpkgs> {}}: let
  overrides = builtins.fromTOML (builtins.readFile ./rust-toolchain.toml);
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      clang
      evcxr
      llvmPackages.bintools
      rust-analyzer
      rustup
      sccache
      trunk
    ];
    # nativeBuildInputs = with pkgs; [];

    RUSTC_VERSION = overrides.toolchain.channel;
    LIBCLANG_PATH =
      pkgs.lib.makeLibraryPath [pkgs.llvmPackages_latest.libclang.lib];
    shellHook = ''
      export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
      export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
    '';
  }
