// src/workspaces/visual_editor/audio_actions.rs

use crate::MFlashStudioApp;

pub fn handle_speak(
    app: &mut MFlashStudioApp,
    deck_term_fallback: &str,
    deck_def_fallback: &str,
) {
    if let Some(deck) = &app.deck {
        let card = &deck.cards[app.selected_index];

        let term_lang = card
            .term_lang
            .as_deref()
            .unwrap_or(deck_term_fallback);

        let def_lang = card
            .def_lang
            .as_deref()
            .unwrap_or(deck_def_fallback);

        let mut played_media = false;

        // 1. Try to find and play attached audio media first.
        if app.enable_media_audio {
            if let Some(audio_meta) = card.media.iter().find(|media| media.media_type == "audio") {
                println!("Found pre-recorded audio file to play: {}", audio_meta.src);
                // app.audio.play_file(&audio_meta.src);
                played_media = true;
            }
        }

        // 2. Fallback to native TTS if no file played.
        if app.enable_tts && !played_media {
            app.audio
                .speak(card.term.as_deref().unwrap_or(""), Some(term_lang), true);

            app.audio.speak(
                card.definition.as_deref().unwrap_or(""),
                Some(def_lang),
                false,
            );
        }
    }
}