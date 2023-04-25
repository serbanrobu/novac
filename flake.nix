{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { flake-utils, nixpkgs, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            bacon # Background rust code check
            clang
            evcxr # Rust REPL
            lldb
            llvmPackages.bintools
            nil # Nix language server
            nixpkgs-fmt # Nix formatter
            nodePackages.vscode-langservers-extracted
            openssl
            pkg-config
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [ "rust-analyzer" "rust-src" ];
              targets = [ "wasm32-unknown-unknown" ];
            }))
            taplo # TOML language server
            trunk
          ];
        };
      }
    );
}
