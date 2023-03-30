{
  outputs = { self, nixpkgs }:
  let
    systems = ["x86_64-linux" "aarch64-linux"];
    crossSystemConfig = "aarch64-unknown-linux-musl";
    ipupdDevShell = nixpkgs: with nixpkgs; mkShell {
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
  in
  {
    devShells = nixpkgs.lib.genAttrs systems (system: 
    {
      default = ipupdDevShell (import nixpkgs { inherit system; });
      ipupdCross = ipupdDevShell (import nixpkgs { inherit system; crossSystem = { config = crossSystemConfig; }; });
    });
  };
}
