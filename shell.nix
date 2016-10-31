{ pkgs ? import <nixpkgs> {} }:

with pkgs;
let funs = pkgs.callPackage ./nix/rust-nightly.nix { };
    cargoNightly = funs.cargo {
      date = "2016-10-28";
      hash = "0dpv1r65wjkm4dw1bc19fvv3nrp2iy1i1q0dqg37yjhq4fng7ni5";
    };

    rustNightly = funs.rust {
      date = "2016-10-28";
      hash = "0z1501dcs2zfxg3a4ghrc8s37xa00pz9piamg8938h1p0nc6lbv3";
    };

    gdb = stdenv.lib.overrideDerivation pkgs.gdb (oldAttrs: {
      src = fetchgit {
        url = "https://github.com/phil-opp/binutils-gdb";
        rev = "385ef013e9aa172cf3ca851be7b809328c89f74b";
        sha256 = "127a5gi59cw5yzyv03xvzvabqna9akfj8blh5zc28aa51izdgzjv";
      };
      buildInputs = oldAttrs.buildInputs ++ [ pkgs.flex pkgs.bison pkgs.gnum4 ];
    });

in stdenv.mkDerivation {
  name = "rux-build-env";

  SSL_CERT_FILE = "/etc/ssl/certs/ca-bundle.crt";

  buildInputs = [
    gnumake
    (binutils.override { cross = { config = "x86_64-none-elf"; }; })
    qemu
    file
    gdb
    rustNightly
    cargoNightly
  ];
}
