{
  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    crossSystem = { config = "aarch64-unknown-linux-gnu"; };
    nixpkgsCross = import nixpkgs { inherit system crossSystem; };
  in
  with nixpkgsCross;
  {
    devShells.${system}.default = mkShell {
      nativeBuildInputs = with pkgsBuildHost; [
        cargo
        rust-analyzer
        rustc
        rustfmt
        stdenv.cc
      ];

      CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";
    };
  };
}
