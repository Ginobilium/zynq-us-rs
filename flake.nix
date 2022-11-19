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
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        name = "zynq-us-rs-dev-shell";
        buildInputs = (with pkgs; [
          openocd
          gdb
        ]) ++ [
          rustPlatform.rust.rustc
          rustPlatform.rust.cargo
          cargo-xbuild
          mkbootimage
        ];
      };

      formatter.x86_64-linux = pkgs.nixpkgs-fmt;
    };
}
