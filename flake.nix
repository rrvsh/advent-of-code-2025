{
  inputs.nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  outputs =
    inputs:
    let
      pkgs = inputs.nixpkgs.legacyPackages.aarch64-darwin;
    in
    {
      devShells.aarch64-darwin.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          bacon
          cargo
          clippy
          just
          rustc
          rustfmt
        ];
      };
    };
}
