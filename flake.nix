{
  description = "Bare-metal rust on Zynq UltraScale+";

  inputs = {
    zynq-rs.url = git+https://git.m-labs.hk/M-Labs/zynq-rs;
    nixpkgs.url = github:NixOS/nixpkgs/nixos-22.05;
    mozilla-overlay.url = github:mozilla/nixpkgs-mozilla;
    binutils-src = {
      type = "tarball";
      url = "https://ftp.gnu.org/gnu/binutils/binutils-2.39.tar.bz2";
      narHash = "sha256-ksj3VaJMrb82zUgEVSHW/agX4X8q9hgUrywHCNz9ENk=";
      flake = false;
    };
    gcc-src = {
      type = "tarball";
      url = "https://ftp.gnu.org/gnu/gcc/gcc-12.2.0/gcc-12.2.0.tar.xz";
      narHash = "sha256-u1AF7a7xtXdepsfbvw2+501EF88rOFVMOZX3DUFf/9I=";
      flake = false;
    };
    gdb-src = {
      type = "tarball";
      url = "https://ftp.gnu.org/gnu/gdb/gdb-12.1.tar.xz";
      narHash = "sha256-tyR2yqsOQupCC3csq0W41vntPhjJ0Wqx//8O0veE1M4=";
      flake = false;
    };
    newlib-src = {
      type = "tarball";
      url = "ftp://sourceware.org/pub/newlib/newlib-4.2.0.20211231.tar.gz";
      narHash = "sha256-nJtCMMkSPoWnS5CTvlOalydfCMk906Oq4bUT37sl99I=";
      flake = false;
    };
    openocd-src = {
      type = "tarball";
      url = "https://sourceforge.net/projects/openocd/files/openocd/0.12.0-rc2/openocd-0.12.0-rc2.tar.bz2";
      narHash = "sha256-xwi6Y7aglrLo0p1TAwCGU+ApNgNRXnJcffrWZc5RdwE=";
      flake = false;
    };
    rustManifest = {
      url = https://static.rust-lang.org/dist/2022-12-11/channel-rust-nightly.toml;
      flake = false;
    };
  };

  outputs = { self, zynq-rs, nixpkgs, mozilla-overlay, binutils-src, gcc-src, gdb-src, newlib-src, openocd-src, rustManifest }:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; overlays = [ mozilla-overlay.overlays.rust ]; };

      rust = (pkgs.lib.rustLib.fromManifestFile rustManifest {
        inherit (pkgs) stdenv lib fetchurl patchelf;
      }).rust.override {
        extensions = [ "rust-src" ];
      };
      rustPlatform = pkgs.recurseIntoAttrs (pkgs.makeRustPlatform {
        rustc = rust;
        cargo = rust;
      });

      mkbootimage = zynq-rs.packages.x86_64-linux.mkbootimage;

      gnu-platform = "aarch64-none-elf";

      binutils-pkg = { zlib, bison, perl, texinfo, gettext, extraConfigureFlags ? [ ] }: pkgs.stdenv.mkDerivation rec {
        basename = "binutils";
        pname = "${basename}-${gnu-platform}";
        version = "2.39";
        src = binutils-src;
        configureFlags = [
          "--enable-deterministic-archives"
          "--with-system-zlib"
          "--target=${gnu-platform}"
          "--with-cpu=cortex-a53"
          "--with-fpu=vfpv3"
          "--with-float=hard"
          "--with-mode=thumb"
        ] ++ extraConfigureFlags;
        outputs = [ "out" "info" "man" ];
        depsBuildBuild = [ pkgs.buildPackages.stdenv.cc ];
        nativeBuildInputs = [ bison perl texinfo ];
        buildInputs = [ zlib gettext ];
        enableParallelBuilding = true;
        meta = {
          description = "Tools for manipulating binaries (linker, assembler, etc.)";
          longDescription = ''
            The GNU Binutils are a collection of binary tools.  The main
            ones are `ld' (the GNU linker) and `as' (the GNU assembler).
            They also include the BFD (Binary File Descriptor) library,
            `gprof', `nm', `strip', etc.
          '';
          homepage = http://www.gnu.org/software/binutils/;
          license = pkgs.lib.licenses.gpl3Plus;
          /* Give binutils a lower priority than gcc-wrapper to prevent a
            collision due to the ld/as wrappers/symlinks in the latter. */
          priority = "10";
        };
      };

      gcc-pkg = { gmp, mpfr, libmpc, platform-binutils, extraConfigureFlags ? [ ] }: pkgs.stdenv.mkDerivation rec {
        basename = "gcc";
        pname = "${basename}-${gnu-platform}";
        version = "12.2.0";
        src = gcc-src;
        preConfigure = ''
          mkdir build
          cd build
        '';
        configureScript = "../configure";
        configureFlags = [
          "--target=${gnu-platform}"
          "--with-arch=armv8.2-a"
          "--with-tune=cortex-a53"
          "--disable-libssp"
          "--enable-languages=c"
          "--with-as=${platform-binutils}/bin/${gnu-platform}-as"
          "--with-ld=${platform-binutils}/bin/${gnu-platform}-ld"
        ] ++ extraConfigureFlags;
        outputs = [ "out" "info" "man" ];
        hardeningDisable = [ "format" "pie" ];
        propagatedBuildInputs = [ gmp mpfr libmpc platform-binutils ];
        enableParallelBuilding = true;
        dontFixup = true;
      };

      gdb-pkg =
        { pkg-config
        , texinfo
        , perl
        , setupDebugInfoDirs
        , ncurses
        , readline
        , gmp
        , mpfr
        , expat
        , libipt
        , zlib
        , guile
        , sourceHighlight
        ,
        }: pkgs.stdenv.mkDerivation rec {
          basename = "gdb";
          pname = "${basename}-${gnu-platform}";
          version = "12.1";
          src = gdb-src;

          nativeBuildInputs = [ pkg-config texinfo perl setupDebugInfoDirs ];
          buildInputs = [ ncurses readline gmp mpfr expat libipt zlib guile sourceHighlight ];
          propagatedNativeBuildInputs = [ setupDebugInfoDirs ];
          depsBuildBuild = [ pkgs.buildPackages.stdenv.cc ];

          preConfigure = ''
            mkdir _build
            cd _build
          '';
          configureScript = "../configure";
          configureFlags = [
            "--disable-werror"
            "--disable-install-libbfd"
            "--disable-shared"
            "--enable-static"
            "--with-system-zlib"
            "--with-system-readline"
            "--enable-tui"
            "--with-curses"
            "--with-gmp=${gmp.dev}"
            "--with-mpfr=${mpfr.dev}"
            # "--with-guile=${guile.dev}"
            "--with-expat"
            "--with-libexpat-prefix=${expat.dev}"
            "--target=${gnu-platform}"
            "--with-arch=armv8.2-a"
            "--with-tune=cortex-a53"
          ];
          postInstall = '' 
          # Remove Info files already provided by Binutils and other packages.
          rm -v $out/share/info/bfd.info
        '';
          enableParallelBuilding = true;
        };

      newlib-pkg = { platform-binutils, platform-gcc }: pkgs.stdenv.mkDerivation rec {
        pname = "newlib";
        version = "4.2.0.20211231";
        src = newlib-src;
        nativeBuildInputs = [ platform-binutils platform-gcc ];
        configureFlags = [
          "--target=${gnu-platform}"

          "--with-cpu=cortex-a53"
          # "--with-fpu=vfpv3"
          "--with-float=hard"
          "--with-mode=thumb"
          "--enable-interwork"
          "--disable-multilib"

          "--disable-newlib-supplied-syscalls"
          "--with-gnu-ld"
          "--with-gnu-as"
          "--disable-newlib-io-float"
          "--disable-werror"
        ];
        dontFixup = true;
      };
      gnutoolchain = rec {
        binutils-bootstrap = pkgs.callPackage binutils-pkg { };
        gcc-bootstrap = pkgs.callPackage gcc-pkg {
          platform-binutils = binutils-bootstrap;
          extraConfigureFlags = [ "--disable-libgcc" ];
        };
        newlib = pkgs.callPackage newlib-pkg {
          platform-binutils = binutils-bootstrap;
          platform-gcc = gcc-bootstrap;
        };
        binutils = pkgs.callPackage binutils-pkg {
          extraConfigureFlags = [ "--with-lib-path=${newlib}/arm-none-eabi/lib" ];
        };
        gcc = pkgs.callPackage gcc-pkg {
          platform-binutils = binutils;
          extraConfigureFlags = [ "--enable-newlib" "--with-headers=${newlib}/arm-none-eabi/include" ];
        };
        gdb = pkgs.callPackage gdb-pkg { readline = pkgs.readline81; };
      };

      openocd = pkgs.openocd.overrideAttrs (oa: rec {
        version = "0.12.0-rc2";
        src = openocd-src;
        patches = [ ];
        buildInputs = oa.buildInputs ++ [ pkgs.capstone ];
      });
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        name = "zynq-us-rs-dev-shell";
        buildInputs = (with pkgs; [
          rustup  # currently required for build-std to find the rust source
        ]) ++ [
          rustPlatform.rust.rustc
          rustPlatform.rust.cargo
          gnutoolchain.binutils
          gnutoolchain.gcc
          gnutoolchain.gdb
          mkbootimage
          openocd
        ];
      };

      inherit rustPlatform;

      formatter.x86_64-linux = pkgs.nixpkgs-fmt;
    };
}
