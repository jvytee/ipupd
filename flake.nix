{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-23.11";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix }:
    let
      toolchain = system:
        with fenix.packages.${system};
        combine [
          stable.cargo
          stable.rustc
        ];

      devToolchain = system: fenix.packages.${system}.stable.toolchain;

      rustPlatform = { pkgs, system }:
        let fenixToolchain = toolchain system;
        in pkgs.makeRustPlatform { cargo = fenixToolchain; rustc = fenixToolchain; };

      ipupdDevShell = system:
        let
          pkgs = import nixpkgs { inherit system; };
          fenixToolchain = devToolchain system;
        in
        pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            fenixToolchain
            gh
            yaml-language-server
          ];
        };

      ipupdPackage = system:
        let
          pkgs = import nixpkgs { inherit system; };
          fenixRustPlatform = rustPlatform { inherit pkgs system; };
        in
        fenixRustPlatform.buildRustPackage {
          pname = "ipupd";
          version = "0.3.0";
          src = self;

          cargoLock.lockFile = ./Cargo.lock;
        };
    in
    {
      devShells = {
        x86_64-linux.default = ipupdDevShell "x86_64-linux";
        aarch64-linux.default = ipupdDevShell "aarch64-linux";
      };

      formatter.x86_64-linux = nixpkgs.legacyPackages.x86_64-linux.nixpkgs-fmt;

      packages = {
        x86_64-linux.default = ipupdPackage "x86_64-linux";
        aarch64-linux.default = ipupdPackage "aarch64-linux";
      };
    };
}
