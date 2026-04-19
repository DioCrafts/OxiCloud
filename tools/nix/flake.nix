{
  description = "OxiCloud Nix flake with module";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    systems = ["x86_64-linux"];
    forAllSystems = f:
      nixpkgs.lib.genAttrs systems (
        system:
          f (import nixpkgs {inherit system;})
      );
  in {
    packages = forAllSystems (pkgs: {
      oxicloud = pkgs.callPackage ./package.nix {};
    });

    nixosModules.oxicloud = import ./module.nix;

    # optional default
    defaultPackage.x86_64-linux = self.packages.x86_64-linux.oxicloud;
  };
}
