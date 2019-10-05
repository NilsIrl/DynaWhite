
with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "dice-game";
  buildInputs = [
    pkgs.cargo
    pkgs.openssl
    pkgs.openjdk
    pkgs.moreutils
  ];

  shellHook = ''
    export OPENSSL_DIR="${openssl.dev}"
    export OPENSSL_LIB_DIR="${openssl.out}/lib"
    '';

}
