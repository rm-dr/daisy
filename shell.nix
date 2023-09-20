let
 pkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/f155f0cf4ea43c4e3c8918d2d327d44777b6cad4.tar.gz") {};
in pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    rustfmt
    m4
   ];
}

