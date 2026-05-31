{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      eachSystem = nixpkgs.lib.genAttrs systems;
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
    in
    {
      overlays.default = final: prev: {
        ipupdate = final.callPackage ./package.nix { };
      };

      devShells = eachSystem (system: {
        default =
          with import nixpkgs { inherit system; };
          mkShell {
            nativeBuildInputs = [
              cargo
              clippy
              gdb
              openssl
              pkg-config
              rust-analyzer
              rustc
              rustfmt
            ];
          };
      });

      packages = eachSystem (system: {
        default =
          let
            pkgs = nixpkgs.legacyPackages.${system};
            overlay = self.overlays.default pkgs pkgs;
          in
          overlay.ipupdate;

        ipupdate-x86_64 =
          with import nixpkgs {
            localSystem = system;
            crossSystem = {
              system = "x86_64-unknown-linux-musl";
              isStatic = true;
              rustc.rustcTargetSpec = "x86_64-unknown-linux-musl";
            };
          };
          pkgs.callPackage ./package.nix { };

        ipupdate-aarch64 =
          with import nixpkgs {
            localSystem = system;
            crossSystem = {
              system = "aarch64-unknown-linux-musl";
              isStatic = true;
              rustc.rustcTargetSpec = "aarch64-unknown-linux-musl";
            };
          };
          callPackage ./package.nix { };
      });
    };
}
