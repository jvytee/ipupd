{
  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    # crossSystem = { config = "aarch64-unknown-linux-gnu"; };
    # crossSystem = { config = "aarch64-unknown-linux-musl"; };
    crossSystem = { config = "x86_64-unknown-linux-musl"; };
    nixpkgsCross = import nixpkgs { inherit system crossSystem; };
  in
  with nixpkgsCross;
  {
    devShells.${system}.ipupdAarch64 = mkShell {
      nativeBuildInputs = with pkgsBuildHost; [
        cargo
        rust-analyzer
        rustc
        rustfmt
        stdenv.cc
      ];

      CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";
      CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER = "${stdenv.cc.targetPrefix}cc";
    };
  };
}
