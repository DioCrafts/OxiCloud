{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "oxicloud";
  version = "v.0.5.5";

  src = pkgs.fetchFromGitHub {
    owner = "DioCrafts";
    repo = "OxiCloud";
    rev = "v0.5.5";
    hash = "sha256-Nn8qgLdiw7w4PZIMCiI+UHZGNW64fjWZ5mErTJifRZU=";
  };

  cargoHash = "sha256-4KfrKL2AKkTt3cOXdl9Xr2qed+qy8WSWuqYfN8WJ0bQ=";

  buildInputs = [
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

  # If tests fail due to DB, disable:
  # doCheck = false;

  postInstall = ''
    mkdir -p $out/bin

    # main server
    if [ -f target/release/ ]; then
      cp -a target/release/oxicloud $out/bin/oxicloud
      cp -a target/release/generate-openapi $out/bin/generate-openapi
    fi

    cp -a static $out/static
    cp -a static-dist $out/static-dist
  '';

  meta = with pkgs.lib; {
    description = "Lightweight Rust-powered self-hosted cloud (Nextcloud alternative)";
    homepage = "https://github.com/DioCrafts/OxiCloud";
    license = licenses.mit;
    platforms = platforms.linux;
  };
}
