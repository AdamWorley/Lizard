{
  description = "Keystroke Audio Player development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
          targets = [ "x86_64-pc-windows-gnu" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            cargo-watch
            cargo-edit
            
            # Cross-compilation tools
            pkgsCross.mingwW64.stdenv.cc
            pkgsCross.mingwW64.windows.pthreads
            wine
            
            # Audio dependencies
            alsa-lib
            pkg-config
            libudev-zero
            
            # X11 dependencies (for Linux global key capture)
            xorg.libX11
            xorg.libXi
            xorg.libXtst
            
            # Additional system libraries that might be needed
            openssl
            cmake
          ];

          shellHook = ''
            echo "🦀 Rust development environment loaded!"
            echo "Available commands:"
            echo "  cargo run      - Run the keystroke audio player"
            echo "  cargo watch -x run - Auto-restart on file changes"
            echo "  cargo clippy   - Run linter"
            echo "  cargo fmt      - Format code"
            echo "  cargo build --target x86_64-pc-windows-gnu - Build Windows executable"
            echo ""
            echo "Place your audio files in the ./audio/ directory"
          '';

          # Environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          
          # For audio libraries
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.alsa-lib
            pkgs.xorg.libX11
            pkgs.xorg.libXi
            pkgs.xorg.libXtst
          ];
        };
      });
}