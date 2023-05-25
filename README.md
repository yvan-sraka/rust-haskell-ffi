# Rust to Haskell FFI demo project

This project aim's to demonstrate how to call an dynamic library built from
Rust code from an Haskell binary.

This project is built using `haskell.nix`, se ultimatly we expect to just:

```
nix build
nix-build
cargo dulid
cabal run
```

TODO:

- [ ] Setup Niv
- [ ] Setup Haskell.nix
- [ ] Add cargo as bulid dependency
- [ ] Make nix-build rely on cargo build using haskell.nix dependency
