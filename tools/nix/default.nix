{
  system ? builtins.currentSystem,
  pkgs ? import <nixpkgs> {},
  withManuals ? false, # building the manuals is expensive
}: let
  lib = pkgs.lib;
  oxicloud = pkgs.callPackage ./package.nix {};
in {
  shell = pkgs.mkShell {
    packages = [
      oxicloud
      # Alpine Rust
      pkgs.postgresql
      pkgs.cargo
      pkgs.rustup
      pkgs.openssl
      pkgs.binutils
      pkgs.zlib-ng
      pkgs.zstd
      pkgs.mpfr
      pkgs.cryptopp
      pkgs.cacert
      pkgs.mkcert
      pkgs.certstrap
      pkgs.musl
      pkgs.isl
      pkgs.gcc
      pkgs.libgcc
      pkgs.jansson
      pkgs.libatomic_ops
      pkgs.pax-utils
      pkgs.gomp
      pkgs.libressl
      pkgs.mpc
      pkgs.pkg-config
      pkgs.libpq
      pkgs.libpqxx
      pkgs.perl
      pkgs.gnumake
      pkgs.su-exec
      # Postgres
      pkgs.tzdata
      pkgs.keyutils
      pkgs.gsasl
      pkgs.xz
      pkgs.libedit
      pkgs.libuuid
      pkgs.libxslt
      # WOPI
      # collabora/code:latest
      # Extra (I think)
      pkgs.ffmpeg
      pkgs.libavif
      pkgs.librtprocess
      pkgs.pdftk
      pkgs.imagemagick
    ];
  };
}
