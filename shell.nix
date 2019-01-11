  with import <nixpkgs> {};

  stdenv.mkDerivation {
    name = "rust-env";
    buildInputs = [
        # Additional Dependencies
        rustup pkgconfig openssl stdenv bashInteractive xorg.libX11 xorg.libXrandr
    ];

    # Environment Variables
    RUST_BACKTRACE=1;
    PATH="$PATH:/home/evanjs/.cargo/bin";
  }
