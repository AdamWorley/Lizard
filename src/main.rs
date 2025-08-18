use rdev::{listen, EventType, Key};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

fn get_key_audio_file(key: Key) -> Option<&'static str> {
    match key {
        Key::KeyL => Some("audio/lizard.mp3"),
        Key::KeyI => Some("audio/lizard.mp3"),
        Key::KeyZ => Some("audio/lizard.mp3"),
        Key::KeyA => Some("audio/lizard.mp3"),
        Key::KeyR => Some("audio/lizard.mp3"),
        Key::KeyD => Some("audio/lizard.mp3"),
        _ => None,
    }
}

fn play_audio_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    
    let file = File::open(file_path)?;
    let source = Decoder::new(BufReader::new(file))?;
    
    sink.append(source);
    sink.sleep_until_end();
    
    Ok(())
}


fn callback(event: rdev::Event) {
    if let EventType::KeyPress(key) = event.event_type {
        println!("Key pressed: {:?}", key);
        
        if let Some(audio_file) = get_key_audio_file(key) {
            println!("Playing audio: {} for key: {:?}", audio_file, key);
            
            if let Err(e) = play_audio_file(audio_file) {
                eprintln!("Error playing audio: {}", e);
            } else {
                println!("Audio playback completed successfully");
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Lizard keystroke audio player...");
    println!("Press any letter in 'LIZARD' to play audio. Press Ctrl+C to exit.");

    // Start listening for key events
    if let Err(error) = listen(callback) {
        eprintln!("Error listening for events: {:?}", error);
    }

    Ok(())
}