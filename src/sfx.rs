use std::io::Cursor;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
// 1. Updated imports for rodio 0.22+
use rodio::{Decoder, DeviceSinkBuilder, Player};

pub enum SoundEffect {
    Save,
}

// 2. Make sure this file actually exists on your hard drive!
const SAVE_SOUND_BYTES: &[u8] = include_bytes!("../assets/save_scratch.wav");

pub struct SfxEngine {
    tx: Sender<SoundEffect>,
    pub is_muted: Arc<Mutex<bool>>,
}

impl SfxEngine {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let is_muted = Arc::new(Mutex::new(false));
        let thread_is_muted = Arc::clone(&is_muted);

        thread::spawn(move || {
            // 3. rodio 0.22: Initialize the audio hardware via DeviceSinkBuilder
            let stream_handle =
                DeviceSinkBuilder::open_default_sink().expect("Failed to get audio output device");

            // 4. rodio 0.22: Create a Player (formerly Sink) and connect it to the mixer
            let player = Player::connect_new(stream_handle.mixer());

            while let Ok(effect) = rx.recv() {
                if *thread_is_muted.lock().unwrap() {
                    continue;
                }

                match effect {
                    SoundEffect::Save => {
                        let cursor = Cursor::new(SAVE_SOUND_BYTES);

                        // 5. rodio 0.22 uses try_from instead of new() for decoders
                        if let Ok(source) = Decoder::try_from(cursor) {
                            player.append(source);
                        } else {
                            eprintln!("Failed to decode save_scratch.wav");
                        }
                    }
                }
            }
        });

        Self { tx, is_muted }
    }

    pub fn play(&self, effect: SoundEffect) {
        if !*self.is_muted.lock().unwrap() {
            let _ = self.tx.send(effect);
        }
    }

    pub fn toggle_mute(&self) {
        let mut muted = self.is_muted.lock().unwrap();
        *muted = !*muted;
    }
}
