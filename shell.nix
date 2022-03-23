let
  # Pinned nixpkgs, deterministic (hermetic) build. Last updated: 3/23/22.
  pkgs = import (fetchTarball ("https://github.com/NixOS/nixpkgs/archive/d5767724527e4ae969e29c35bc47d566470961ca.tar.gz")) { };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    # Haskell tooling
    cabal-install
    ghc
    # Rust tooling
    cargo
    rustc
  ];
}
