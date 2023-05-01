{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-22.11";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      toolchain = fenix: system: target:
        with fenix.packages.${system};
        if system == target
        then stable.toolchain
        else
          combine [
            stable.cargo
            stable.rustc
            targets.${target}.stable.rust-std
          ];

      fenixRustPlatform = nixpkgs: fenix: system: target:
        let fenixToolchain = toolchain fenix system target;
        in
        nixpkgs.makeRustPlatform { cargo = fenixToolchain; rustc = fenixToolchain; };

      ipupdDevShell = nixpkgs: fenix: system: target: nixpkgs.mkShell {
        nativeBuildInputs = [ (toolchain fenix system target) ];
        CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${nixpkgs.stdenv.cc.targetPrefix}cc";
      };

      ipupdPackage = nixpkgs: fenix: system: target:
        (fenixRustPlatform nixpkgs fenix system target).buildRustPackage {
          pname = "ipupd";
          version = "0.3.0";

          src = nixpkgs.fetchgit {
            url = "https://github.com/jvytee/ipupd.git";
            rev = "e5b846416eabd43899b68f73880a694aca29f7fc";
            sha256 = "sha256-s+zccrbBUatKT9Zbf/rKAZEtdVc7h99vZvQqm9Ri73Q=";
          };

          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with nixpkgs.pkgsBuildHost; [
            stdenv.cc
          ];

          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${nixpkgs.stdenv.cc.targetPrefix}cc";
        };
    in
    {
      devShells = nixpkgs.lib.genAttrs systems (
        system: {
          default = ipupdDevShell (import nixpkgs { inherit system; }) fenix system system;
          ipupd-aarch64 = ipupdDevShell (import nixpkgs { inherit system; crossSystem.config = "aarch64-unknown-linux-gnu"; }) fenix system "aarch64-unknown-linux-gnu";
        }
      );

      formatter = nixpkgs.lib.genAttrs systems (
        system: nixpkgs.legacyPackages.${system}.nixpkgs-fmt
      );

      packages = nixpkgs.lib.genAttrs systems (
        system: {
          default = ipupdPackage (import nixpkgs { inherit system; }) fenix system system;
          ipupd-aarch64 = ipupdPackage (import nixpkgs { inherit system; crossSystem.config = "aarch64-unknown-linux-gnu"; }) fenix system "aarch64-unknown-linux-gnu";
        }
      );
    };
}
