{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-23.05";
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
        if target == system
        then stable.toolchain
        else
          combine [
            stable.cargo
            stable.rustc
            targets.${target}.stable.rust-std
          ];

      importNixpkgs = nixpkgs: system: target:
        if target == system
        then import nixpkgs { inherit system; }
        else import nixpkgs { inherit system; crossSystem.config = target; };

      fenixRustPlatform = nixpkgs: fenix: system: target:
        let fenixToolchain = toolchain fenix system target;
        in nixpkgs.makeRustPlatform { cargo = fenixToolchain; rustc = fenixToolchain; };

      ipupdDevShell = nixpkgs: fenix: system: target:
        with importNixpkgs nixpkgs system target;
        mkShell {
          nativeBuildInputs = [ (toolchain fenix system target) ];
          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";
        };

      ipupdPackage = nixpkgs: fenix: system: target:
        let pkgs = importNixpkgs nixpkgs system target;
        in
        with pkgs;
        (fenixRustPlatform pkgs fenix system target).buildRustPackage {
          pname = "ipupd";
          version = "0.3.0";
          src = self;

          cargoLock.lockFile = ./Cargo.lock;
          depsBuildTarget = with pkgsBuildTarget; [ stdenv.cc ];

          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";
        };
    in
    {
      devShells = nixpkgs.lib.genAttrs systems (
        system: {
          default = ipupdDevShell nixpkgs fenix system system;
          ipupd-aarch64 = ipupdDevShell nixpkgs fenix system "aarch64-unknown-linux-gnu";
        }
      );

      formatter = nixpkgs.lib.genAttrs systems (
        system: nixpkgs.legacyPackages.${system}.nixpkgs-fmt
      );

      packages = nixpkgs.lib.genAttrs systems (
        system: {
          default = ipupdPackage nixpkgs fenix system system;
          ipupd-aarch64 = ipupdPackage nixpkgs fenix system "aarch64-unknown-linux-gnu";
        }
      );
    };
}
