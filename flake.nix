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
      toolchain = { system, rustTarget }:
        with fenix.packages.${system};
        if rustTarget == system
        then combine [
          stable.cargo
          stable.rustc
        ]
        else combine [
          stable.cargo
          stable.rustc
          targets.${rustTarget}.stable.rust-std
        ];

      devToolchain = system: fenix.packages.${system}.stable.toolchain;

      rustPlatform = { pkgs, system, rustTarget }:
        let
          fenixToolchain = toolchain { inherit system rustTarget; };
          configuredStdenv = pkgs.stdenv.override (prev: { hostPlatform = prev.hostPlatform // { rustc.config = rustTarget; }; });
        in
          if rustTarget == system
          then pkgs.makeRustPlatform { cargo = fenixToolchain; rustc = fenixToolchain; }
          else pkgs.makeRustPlatform { cargo = fenixToolchain; rustc = fenixToolchain; stdenv = configuredStdenv; };

      ipupdDevShell = { system, rustTarget ? system }:
        let
          pkgs = import nixpkgs { inherit system; };
          fenixToolchain = devToolchain system;
        in pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            fenixToolchain
            gh
            yaml-language-server
          ];
        };

      ipupdPackage = { system, rustTarget ? system }:
        let
          pkgs = import nixpkgs { inherit system; };
          fenixRustPlatform = rustPlatform { inherit pkgs system rustTarget; };
        in fenixRustPlatform.buildRustPackage {
          pname = "ipupd";
          version = "0.3.0";
          src = self;

          cargoLock.lockFile = ./Cargo.lock;
        };
    in {
      devShells = {
        x86_64-linux.default = ipupdDevShell { system = "x86_64-linux"; rustTarget = "x86_64-unknown-linux-musl"; };
        aarch64-linux.default = ipupdDevShell { system = "aarch64-linux"; rustTarget = "aarch64-unknown-linux-musl"; };
      };

      formatter.x86_64-linux = nixpkgs.legacyPackages.x86_64-linux.nixpkgs-fmt;

      packages = {
        x86_64-linux.default = ipupdPackage { system = "x86_64-linux"; };
        x86_64-linux.static = ipupdPackage { system = "x86_64-linux"; rustTarget = "x86_64-unknown-linux-musl"; };
        aarch64-linux.default = ipupdPackage { system = "aarch64-linux"; };
        aarch64-linux.static = ipupdPackage { system = "aarch64-linux"; rustTarget = "aarch64-unknown-linux-musl"; };
      };
    };
}
