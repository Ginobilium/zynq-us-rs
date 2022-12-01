{
  description = "Bare-metal rust on Zynq UltraScale+";

  inputs = {
    zynq-rs.url = git+https://git.m-labs.hk/M-Labs/zynq-rs;
    nixpkgs.follows = "zynq-rs/nixpkgs";
    mozilla-overlay.follows = "zynq-rs/mozilla-overlay";
  };

  outputs = { self, zynq-rs, nixpkgs, mozilla-overlay }:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; overlays = [ (import mozilla-overlay) ]; };

      rustPlatform = zynq-rs.rustPlatform;
      cargo-xbuild = zynq-rs.packages.x86_64-linux.cargo-xbuild;
      mkbootimage = zynq-rs.packages.x86_64-linux.mkbootimage;

      openocd = pkgs.openocd.overrideAttrs(oa: rec {
        version = "0.12.0-rc2";
        src = builtins.fetchurl {
          url = "https://sourceforge.net/projects/${oa.pname}/files/${oa.pname}/${version}/${oa.pname}-${version}.tar.bz2";
          sha256 = "17cfd2428c9abbbbf668ab6134dfac4ce4fb9454550f2b22104855a020a8261f";
        };
        patches = [];
        buildInputs = oa.buildInputs ++ [ pkgs.capstone ];
      });
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        name = "zynq-us-rs-dev-shell";
        buildInputs = (with pkgs; [
          gdb
        ]) ++ [
          rustPlatform.rust.rustc
          rustPlatform.rust.cargo
          cargo-xbuild
          mkbootimage
          openocd
        ];
      };

      formatter.x86_64-linux = pkgs.nixpkgs-fmt;
    };
}
