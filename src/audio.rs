pub struct AudioEngine {
    pub tts: Option<tts::Tts>,
}

impl AudioEngine {
    pub fn new() -> Self {
        Self { tts: tts::Tts::default().ok() }
    }

    /// Updated to accept the card's language field so it can switch accents
    pub fn speak(&mut self, text: &str, language: Option<&str>) {
        if let Some(tts) = &mut self.tts {
            
            // Normalize "lazy" JSON strings into strict ISO 639 codes
            let _iso = match language.map(|l| l.to_lowercase()).as_deref() {
                Some("english") => "en-US",
                Some("spanish") => "es-ES",
                Some("french")  => "fr-FR",
                Some("german")  => "de-DE",
                Some(code) => code, // If it's already an ISO code like "en", use it
                None => "en-US",    // Fallback to English if the field is missing
            };

            // NOTE: To fully change the voice, you would iterate over tts.voices() 
            // to find one matching the _iso code, then call tts.set_voice(&voice).
            // For now, we prepare the architecture and use the default system voice.
            
            let _ = tts.speak(text, true);
        }
    }
}