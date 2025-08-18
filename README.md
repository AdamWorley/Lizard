# Lizard Keystroke Audio Player

A cross-platform audio player that plays sounds when specific keys are pressed. Originally written in Rust, now available in Go.

## Features

- Global keystroke listening for L-I-Z-A-R-D keys
- Audio playback when target keys are detected
- Cross-platform support (Linux, macOS, Windows)
- Background operation with graceful shutdown

## Quick Start

### Go Version (Recommended)

```bash
# Run directly
go run main.go

# Or build and run
go build -o lizard-audio-player
./lizard-audio-player
```

### Rust Version (Legacy)

```bash
# Run directly
cargo run

# Or build and run
cargo build --release
./target/release/keystroke-audio-player
```

## Usage

1. Ensure `audio/lizard.mp3` exists in the same directory
2. Run the application
3. Press any letter in "LIZARD" to trigger audio playback
4. Press Ctrl+C to exit

## Platform Requirements

### Linux
- **X11**: Works with proper permissions (`/dev/input/event*` access)
- **Wayland**: Limited support due to security restrictions
  - May require running with elevated privileges: `sudo go run main.go`
  - Consider using X11 session for full functionality

### macOS
- Requires accessibility permissions for global keystroke capture

### Windows
- Should work without additional setup
- May require Microsoft Visual C++ Redistributable

## File Structure

```
lizard/
├── main.go              # Go implementation
├── src/main.rs          # Rust implementation (legacy)
├── audio/lizard.mp3     # Audio file to play
├── go.mod              # Go dependencies
├── Cargo.toml          # Rust dependencies
└── README.md           # This file
```

## Development

### Go Dependencies
- `github.com/nathan-fiscaletti/key-logger` - Cross-platform keyboard input
- `github.com/faiface/beep` - Audio playback (when implemented)

### Rust Dependencies (Legacy)
- `rdev` - Global input event listening
- `rodio` - Audio playback and format decoding

## Troubleshooting

### Permission Denied Errors (Linux)
```bash
# Try running with elevated privileges
sudo go run main.go
```

### Wayland Compatibility Issues
- Switch to X11 session temporarily for testing
- Or run in X11 compatibility mode
- Global keystroke capture has security limitations on Wayland

### No Audio Output
- Verify `audio/lizard.mp3` exists
- Check system audio settings
- Ensure audio libraries are installed

## Supported Audio Formats

- WAV
- MP3  
- FLAC
- OGG

## Migration from Rust to Go

The Go version provides:
- Simpler dependency management
- Better cross-platform compatibility
- Easier deployment (single binary)
- More reliable keyboard input handling

To use the legacy Rust version, see `WINDOWS_INSTRUCTIONS.md` for Windows-specific build instructions.