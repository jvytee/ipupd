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
      toolchain = fenix: system: rustTarget:
        with fenix.packages.${system};
        combine [
          stable.cargo
          stable.rustc
          targets.${rustTarget}.stable.rust-std
        ];

      importNixpkgs = system: target: rustTarget:
        if target == system
        then import nixpkgs { inherit system; }
        else import nixpkgs { inherit system; crossSystem.config = target; };

      fenixRustPlatform = pkgs: fenix: system: rustTarget:
        let
          fenixToolchain = toolchain fenix system rustTarget;
          configuredStdenv = pkgs.stdenv.override (previous: { hostPlatform = previous.hostPlatform // { rustc.config = rustTarget; }; });
        in pkgs.makeRustPlatform { cargo = fenixToolchain; rustc = fenixToolchain; stdenv = configuredStdenv; };

      ipupdDevShell = fenix: system: target: rustTarget:
        with importNixpkgs system target rustTarget;
        mkShell {
          nativeBuildInputs = [
            (toolchain fenix system rustTarget)
            gh
            rust-analyzer
            yaml-language-server
          ];
          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";
        };

      ipupdPackage = fenix: system: target: rustTarget:
        let pkgs = importNixpkgs system target rustTarget;
        in
        with pkgs;
        (fenixRustPlatform pkgs fenix system rustTarget).buildRustPackage {
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
          default = ipupdDevShell fenix system system "x86_64-unknown-linux-musl";
          ipupd-aarch64 = ipupdDevShell fenix system "aarch64-unknown-linux-gnu" "aarch64-unknown-linux-musl";
        }
      );

      formatter = nixpkgs.lib.genAttrs systems (
        system: nixpkgs.legacyPackages.${system}.nixpkgs-fmt
      );

      packages = nixpkgs.lib.genAttrs systems (
        system: {
          default = ipupdPackage fenix system system "x86_64-unknown-linux-musl";
          ipupd-aarch64 = ipupdPackage fenix system "aarch64-unknown-linux-gnu" "aarch64-unknown-linux-musl";
        }
      );
    };
}
