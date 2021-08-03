with import <nixpkgs> {};

mkShell {
  nativeBuildInputs = [
    pkgconfig
    rust-analyzer
    rustup
  ];

  buildInputs = [
    cacert
    openssl
  ];
}
