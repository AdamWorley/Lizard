use rdev::{listen, EventType, Key};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use tray_icon::{TrayIconBuilder, menu::{Menu, MenuItem}, Icon};

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


fn create_icon() -> Result<Icon, Box<dyn std::error::Error>> {
    // Create a simple 16x16 lizard icon using raw bytes
    let icon_rgba = vec![
        // Simple 16x16 green lizard icon in RGBA format
        0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,
        0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   74, 124, 89, 255,   74, 124, 89, 255,   74, 124, 89, 255,   74, 124, 89, 255,   74, 124, 89, 255,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,
        0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   74, 124, 89, 255,   93, 143, 106, 255,   0, 0, 0, 255,   93, 143, 106, 255,   93, 143, 106, 255,   74, 124, 89, 255,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,
        0, 0, 0, 0,   0, 0, 0, 0,   74, 124, 89, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   74, 124, 89, 255,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,
        0, 0, 0, 0,   74, 124, 89, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   74, 124, 89, 255,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,
        74, 124, 89, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   74, 124, 89, 255,   74, 124, 89, 255,   74, 124, 89, 255,   74, 124, 89, 255,   0, 0, 0, 0,
        93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   74, 124, 89, 255,   0, 0, 0, 0,   74, 124, 89, 255,   74, 124, 89, 255,   74, 124, 89, 255,   74, 124, 89, 255,
        93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   74, 124, 89, 255,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   74, 124, 89, 255,   74, 124, 89, 255,   74, 124, 89, 255,
        74, 124, 89, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   74, 124, 89, 255,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   74, 124, 89, 255,   74, 124, 89, 255,
        0, 0, 0, 0,   74, 124, 89, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   93, 143, 106, 255,   74, 124, 89, 255,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   74, 124, 89, 255,
        0, 0, 0, 0,   0, 0, 0, 0,   74, 124, 89, 255,   74, 124, 89, 255,   74, 124, 89, 255,   74, 124, 89, 255,   74, 124, 89, 255,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,
        0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,
        0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,
        0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,
        0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,
        0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,   0, 0, 0, 0,
    ];
    
    let icon = Icon::from_rgba(icon_rgba, 16, 16)?;
    Ok(icon)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Lizard keystroke audio player...");
    
    // Create shared state for controlling the application
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    
    // Create tray menu
    let quit_item = MenuItem::new("Quit", true, None);
    let about_item = MenuItem::new("About", true, None);
    let menu = Menu::new();
    menu.append(&about_item)?;
    menu.append(&quit_item)?;
    
    // Extract menu item IDs before creating the thread
    let quit_item_id = quit_item.id();
    let about_item_id = about_item.id();
    
    // Create tray icon with custom lizard icon
    let icon = create_icon()?;
    let _tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Lizard - Keystroke Audio Player")
        .with_icon(icon)
        .build()?;
    
    println!("Lizard is now running in system tray. Press any letter in 'LIZARD' to play audio.");
    
    // Start keystroke listening in background thread
    let keystroke_thread = thread::spawn(move || {
        // Create a callback function that can be used with listen
        fn callback_wrapper(event: rdev::Event) {
            // We need to handle the event here without capturing variables
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
        
        if let Err(error) = listen(callback_wrapper) {
            eprintln!("Error listening for events: {:?}", error);
        }
    });
    
    // Handle tray events in a separate thread
    std::thread::spawn({
        let running_clone = running_clone.clone();
        
        move || {
            loop {
                if let Ok(event) = tray_icon::menu::MenuEvent::receiver().try_recv() {
                    match event.id {
                        id if id == quit_item_id => {
                            println!("Quitting Lizard...");
                            running_clone.store(false, Ordering::Relaxed);
                            std::process::exit(0);
                        }
                        id if id == about_item_id => {
                            println!("Lizard v0.1.0 - Keystroke Audio Player");
                            println!("Press L, I, Z, A, R, or D to play lizard sounds!");
                        }
                        _ => {}
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    });
    
    // Keep the main thread alive
    loop {
        if !running.load(Ordering::Relaxed) {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    // Wait for the keystroke thread to finish
    let _ = keystroke_thread.join();
    
    Ok(())
}