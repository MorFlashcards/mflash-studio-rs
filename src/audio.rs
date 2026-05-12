pub mod bcp47;

use crate::audio::bcp47::normalize_to_bcp47;
use std::sync::mpsc::{self, Sender};
use std::thread;
use tts::Tts;

pub enum AudioCommand {
    Speak {
        text: String,
        language: Option<String>,
        interrupt: bool,
    },
    Stop,
}

pub struct AudioEngine {
    tx: Sender<AudioCommand>,
}

impl AudioEngine {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut system_tts = Tts::default().ok();

            while let Ok(command) = rx.recv() {
                match command {
                    AudioCommand::Speak {
                        text,
                        language,
                        interrupt,
                    } => {
                        if let Some(tts) = &mut system_tts {
                            let lang_str = language.as_deref().unwrap_or("en-US");
                            let bcp47_code = normalize_to_bcp47(lang_str);

                            if let Ok(voices) = tts.voices() {
                                let matching_voice = voices.into_iter().find(|v| {
                                    // Compare both as lowercase to ensure custom tags match flexibly
                                    v.language().to_lowercase() == bcp47_code.to_lowercase()
                                        || v.language()
                                            .to_lowercase()
                                            .starts_with(&bcp47_code.to_lowercase())
                                });

                                if let Some(voice) = matching_voice {
                                    let _ = tts.set_voice(&voice);
                                }
                            }
                            let _ = tts.speak(text, interrupt);
                        }
                    }
                    AudioCommand::Stop => {
                        if let Some(tts) = &mut system_tts {
                            let _ = tts.stop();
                        }
                    }
                }
            }
        });

        Self { tx }
    }

    pub fn speak(&self, text: &str, language: Option<&str>, interrupt: bool) {
        let _ = self.tx.send(AudioCommand::Speak {
            text: text.to_string(),
            language: language.map(|s| s.to_string()),
            interrupt,
        });
    }

    pub fn stop(&self) {
        let _ = self.tx.send(AudioCommand::Stop);
    }
}
