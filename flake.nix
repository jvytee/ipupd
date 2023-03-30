{
  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
  in
  {
    devShells.${system}.default = with import nixpkgs { inherit system; }; mkShell {
      buildInputs = [
        cargo
        rust-analyzer
        rustc
        rustfmt
      ];
    };
  };
}
