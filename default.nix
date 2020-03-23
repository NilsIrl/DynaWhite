
with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "dice-game";
  buildInputs = [
    pkgs.openssl
    pkgs.openjdk
    pkgs.maven
  ];

  shellHook = ''
    export OPENSSL_DIR="${openssl.dev}"
    export OPENSSL_LIB_DIR="${openssl.out}/lib"
    '';

}
