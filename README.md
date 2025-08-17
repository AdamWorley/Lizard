# Keystroke Audio Player

A cross-platform background service that plays audio files when specific keys are pressed.

## Features

- Global keystroke listening (works across all applications)
- Cross-platform support (Windows, macOS, Linux)
- Low resource usage
- Configurable key mappings

## Setup

1. Install Rust: https://rustup.rs/
2. Clone this repository
3. Add your audio files to the `audio/` directory
4. Build and run:

```bash
cargo run
```

For a nix system you can use the included nix devshell by using `nix develop`

## Configuration

Currently configured to play `audio/sample.wav` when either 'a' or 'd' keys are pressed.

To modify key mappings, edit the `create_key_mappings()` function in `src/main.rs`.

## Supported Audio Formats

- WAV
- MP3
- FLAC
- OGG

## Usage

1. Place your audio files in the `audio/` directory
2. Run the application: `cargo run`
3. The app runs in the background listening for configured keystrokes
4. Press Ctrl+C to exit

## Permissions

On macOS, you may need to grant accessibility permissions for global keystroke listening.

