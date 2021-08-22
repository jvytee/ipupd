let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "nix-shell_ipupd";
    buildInputs = [
      nixpkgs.latest.rustChannels.stable.rust
      cacert
      openssl
    ];
    nativeBuildInputs = [
      pkgconfig
      rust-analyzer
    ];
  }
