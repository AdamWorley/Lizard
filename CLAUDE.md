# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Build and Run
- `cargo run` - Build and run the keystroke audio player
- `cargo build` - Build the project
- `cargo build --release` - Build optimized release version
- `cargo build --target x86_64-pc-windows-gnu --release` - Build Windows executable

### Development Tools
- `cargo clippy` - Run linter and catch common mistakes
- `cargo fmt` - Format code according to Rust standards
- `cargo check` - Quick compile check without producing executable
- `cargo watch -x run` - Auto-restart on file changes (available in nix shell)

### Nix Development Environment
- `nix develop` - Enter development shell with all dependencies
- Provides: Rust toolchain, Windows cross-compilation tools, audio libraries (ALSA), X11 dependencies, build tools

## Project Architecture

This is a single-file Rust application (`src/main.rs`) that implements a global keystroke listener with audio playback functionality.

### Core Components
- **Global Event Listener**: Uses `rdev` crate for cross-platform keystroke capture
- **Audio Engine**: Uses `rodio` crate for audio file playback and output management
- **Key Mapping**: Simple match-based system mapping specific keys to audio files

### Key Dependencies
- `rdev` (0.3) - Global input event listening across platforms
- `rodio` (0.17) - Audio playback and format decoding
- `serde` + `serde_json` (1.0) - Serialization support (currently unused)

### Audio System Design
- Creates new `OutputStream` and `Sink` for each playback
- Supports WAV, MP3, FLAC, OGG formats via `rodio::Decoder`
- Synchronous playback (blocks until audio completes)

### Current Configuration
- Maps L, I, Z, A, R, D keys to play `audio/lizard.mp3`
- Audio files must be placed in `audio/` directory
- Requires appropriate system permissions for global key capture

### Platform Considerations
- **Linux**: Requires X11 libraries and udev access
- **macOS**: Requires accessibility permissions for global keystroke listening
- **Windows**: Should work without additional setup

## File Structure
- `src/main.rs` - Single source file containing all logic
- `audio/` - Directory for audio files (contains `lizard.mp3`)
- `Cargo.toml` - Project configuration and dependencies
- `flake.nix` - Nix development environment with system dependencies
- `.cargo/config.toml` - Cargo configuration for Windows cross-compilation
- `WINDOWS_INSTRUCTIONS.md` - Instructions for running on Windows
- `target/x86_64-pc-windows-gnu/release/keystroke-audio-player.exe` - Pre-built Windows executable