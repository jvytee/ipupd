{
  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];

      ipupdDevShell = nixpkgs: with nixpkgs; mkShell {
        nativeBuildInputs = with pkgsBuildHost; [
          cargo
          clippy
          nixpkgs-fmt
          rust-analyzer
          rustc
          rustfmt
          stdenv.cc
        ];

        CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";
        # CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER = "${stdenv.cc.targetPrefix}cc";
      };

      ipupdPackage = nixpkgs: with nixpkgs; rustPlatform.buildRustPackage {
        pname = "ipupd";
        version = "0.3.0";

        src = fetchgit {
          url = "https://github.com/jvytee/ipupd.git";
          rev = "e5b846416eabd43899b68f73880a694aca29f7fc";
          sha256 = "sha256-s+zccrbBUatKT9Zbf/rKAZEtdVc7h99vZvQqm9Ri73Q=";
        };

        cargoLock.lockFile = ./Cargo.lock;

        # nativeBuildInputs = with pkgsBuildHost; [
        #   rustc
        #   cargo
        #   stdenv.cc
        # ];

        CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";
      };
    in
    {
      devShells = nixpkgs.lib.genAttrs systems (
        system: {
          default = ipupdDevShell (import nixpkgs { inherit system; });
          ipupd-aarch64 = ipupdDevShell (import nixpkgs { inherit system; crossSystem.system = "aarch64-linux"; });
        }
      );

      formatter = nixpkgs.lib.genAttrs systems (
        system: nixpkgs.legacyPackages.${system}.nixpkgs-fmt
      );

      packages = nixpkgs.lib.genAttrs systems (
        system: {
          default = ipupdPackage (import nixpkgs { inherit system; });
          ipupd-aarch64 = ipupdPackage (import nixpkgs { inherit system; crossSystem.system = "aarch64-linux"; });
        }
      );
    };
}
