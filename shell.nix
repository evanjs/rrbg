  with import <nixpkgs> {};

  stdenv.mkDerivation {
    name = "rust-env";
    buildInputs = [
        # Additional Dependencies
        # nightly-rust-with-extensions -> https://github.com/evanjs/nixos_cfg/blob/ed183e9f25524fba0828b9317ad807c88cfa85b6/overlays/evanjs/pkgs/nightly-rust/default.nix
        nightly-rust-with-extensions.base pkgconfig openssl stdenv bashInteractive xorg.libX11 xorg.libXrandr SDL2 feh
    ];

    # Environment Variables
    RUST_BACKTRACE=1;
    PATH="$PATH:$HOME/.cargo/bin";
  }
