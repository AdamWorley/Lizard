# Windows Instructions for Lizard Keystroke Audio Player

## Pre-built Executable

A Windows executable has been built and is located at:
```
target/x86_64-pc-windows-gnu/release/keystroke-audio-player.exe
```

File size: ~4MB

## Running on Windows

### Method 1: Direct Run
1. Copy the `keystroke-audio-player.exe` file to your Windows machine
2. Copy the `audio/` directory containing `lizard.mp3` to the same location as the exe
3. Double-click `keystroke-audio-player.exe` to run

### Method 2: Command Line
1. Open Command Prompt or PowerShell
2. Navigate to the directory containing the executable
3. Run: `keystroke-audio-player.exe`

## Usage

Once running, the program will:
- Listen for global keystrokes (works across all applications)
- Play `lizard.mp3` when any of these keys are pressed: L, I, Z, A, R, D
- Run in the background until terminated with Ctrl+C

## Important Notes

### Permissions
- Windows may show a security warning when first running the executable
- You may need to "Allow" or "Run anyway" in Windows Defender SmartScreen
- No additional permissions are required for global keystroke listening on Windows

### Antivirus Software
- Some antivirus software may flag keystroke listeners as potentially unwanted
- Add the executable to your antivirus whitelist if needed
- The program only listens for specific keys and plays audio - it does not log or transmit keystrokes

### Audio Requirements
- Ensure your system has audio output enabled
- The `audio/lizard.mp3` file must be in the same directory as the executable
- Supported formats: WAV, MP3, FLAC, OGG

### Stopping the Program
- Press `Ctrl+C` in the command line window (if run from command line)
- Close the command line window
- Use Task Manager to end the process if needed

## File Structure
```
your-folder/
├── keystroke-audio-player.exe
└── audio/
    └── lizard.mp3
```

## Troubleshooting

### "Audio file not found" error
- Ensure the `audio/` directory exists in the same location as the .exe
- Verify `lizard.mp3` is present in the audio directory

### No sound when pressing keys
- Check system volume
- Verify audio output device is working
- Try running from command line to see error messages

### Program doesn't respond to keystrokes
- Try running as Administrator (right-click → "Run as administrator")
- Ensure no other keystroke capture programs are interfering