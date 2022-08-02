{
  description = "Game file syncronization client for Butburner game";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk/master";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    naersk,
  }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        naersk-lib = naersk.lib.${system};
      in rec {
        packages = {
          bitburner-oxide = naersk-lib.buildPackage {
            pname = "bitburner-oxide";
            src = ./.;
            buildInputs = with pkgs; [ pkg-config openssl ];
          };
          default = packages.bitburner-oxide;
        };
        apps = {
          bitburner-oxide = flake-utils.lib.mkApp { drv = packages.bitburner-oxide; };
          default = apps.bitburner-oxide;
        };
        devShell = with pkgs;
          mkShell {
            buildInputs = [
              rustc
              cargo
              pkg-config
              openssl
            ];
          };
      });
}
