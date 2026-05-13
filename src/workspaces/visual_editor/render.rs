use crate::MFlashStudioApp;
use eframe::egui;

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    let mut go_back = false;
    let mut trigger_speak = false;

    let mut json_needs_sync = false;
    let mut core_needs_sync = false;
    let mut action_add_sentence = false;
    let mut action_remove_sentence = None;
    let mut action_add_card = false;
    let mut action_save = false;

    // Fallback defaults from the v2 root deck
    let deck_term_fallback = app
        .deck
        .as_ref()
        .and_then(|d| d.default_term_lang.clone())
        .unwrap_or_else(|| "English".to_string());

    let deck_def_fallback = app
        .deck
        .as_ref()
        .and_then(|d| d.default_def_lang.clone())
        .unwrap_or_else(|| "English".to_string());

    if let Some(deck) = &mut app.deck {
        let total_cards = deck.cards.len();
        let card = &mut deck.cards[app.selected_index];

        // --- TOP NAVIGATION BAR ---
        super::top_bar::render(
            ui,
            super::top_bar::TopBar {
                selected_index: app.selected_index,
                total_cards,
                editor_mode: app.editor_mode,

                enable_tts: &mut app.enable_tts,
                enable_media_audio: &mut app.enable_media_audio,

                go_back: &mut go_back,
                trigger_speak: &mut trigger_speak,
                action_add_card: &mut action_add_card,
                action_save: &mut action_save,
            },
        );

        ui.separator();

        // --- CARD KIND SELECTOR ---
        if app.editor_mode {
            if super::card_kind::render(ui, card) {
                json_needs_sync = true;
                core_needs_sync = true;
            }
            ui.separator();
        }

        // --- QUICK ACCESS OPTIONS BAR ---
        if app.editor_mode {
            super::quick_options::render(
                ui,
                super::quick_options::QuickOptions {
                    show_lang_codes: &mut app.show_lang_codes,
                    show_phonetic: &mut app.show_phonetic,
                    show_part_of_speech: &mut app.show_part_of_speech,
                    show_notes: &mut app.show_notes,
                    show_tags: &mut app.show_tags,
                },
            );
            ui.separator();
        }

        // --- DYNAMIC EDITOR ROUTING ---
        match card.kind {
            crate::models::CardKind::Basic => {
                let basic_actions = super::basic_editor::render(
                    ui,
                    card,
                    super::basic_editor::BasicEditorOptions {
                        editor_mode: app.editor_mode,

                        show_lang_codes: app.show_lang_codes,
                        show_phonetic: app.show_phonetic,
                        show_part_of_speech: app.show_part_of_speech,
                        show_notes: app.show_notes,
                        show_tags: app.show_tags,

                        lang_search_query: &mut app.lang_search_query,

                        deck_term_fallback: &deck_term_fallback,
                        deck_def_fallback: &deck_def_fallback,

                        font_size_header: app.config.ui.font_size_header,
                        font_size_body: app.config.ui.font_size_body,

                        show_images: app.show_images,
                        current_texture: app.current_texture.as_ref(),
                    },
                );

                if basic_actions.json_needs_sync {
                    json_needs_sync = true;
                }

                if basic_actions.core_needs_sync {
                    core_needs_sync = true;
                }

                if basic_actions.action_add_sentence {
                    action_add_sentence = true;
                }

                if let Some(index) = basic_actions.action_remove_sentence {
                    action_remove_sentence = Some(index);
                }
            }
            crate::models::CardKind::ImageOcclusion => {
                if super::image_occlusion::render(ui, card, app.editor_mode) {
                    json_needs_sync = true;
                    core_needs_sync = true;
                }
            }
            
            crate::models::CardKind::Listening => {
                if super::listening::render(ui, card, app.editor_mode) {
                    json_needs_sync = true;
                    core_needs_sync = true;
                }
            }
            
            crate::models::CardKind::MediaPrompt => {
                if super::media_prompt::render(ui, card, app.editor_mode) {
                    json_needs_sync = true;
                    core_needs_sync = true;
                }
            }
            
            crate::models::CardKind::Cloze => {
                super::cloze::render(ui, app.editor_mode);
            }
        }

        let current_card = card.clone();
        ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
            ui.horizontal(|ui| {
                for i in 0..app.plugins.len() {
                    app.plugins[i].render_ui(ui, Some(&current_card));
                }
            });
        });
    }

    if action_add_card {
        app.push_snapshot();
        let new_card_id = format!(
            "card_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
        );

        if let Some(deck) = &mut app.deck {
            let new_card = crate::models::Card {
                id: new_card_id.clone(),
                kind: crate::models::CardKind::Basic,
                term: Some(String::new()),
                definition: Some(String::new()),
                prompt: None,
                answer: None,
                term_lang: None,
                def_lang: None,
                lexical: None,
                notes: None,
                media: Vec::new(),
                tags: Vec::new(),
                examples: Vec::new(),
                occlusion: None,
            };
            deck.cards.push(new_card);
            app.selected_index = deck.cards.len() - 1;
        }

        if let Ok(mut parsed_json) = serde_json::from_str::<serde_json::Value>(&app.raw_schema_text)
        {
            if let Some(cards) = parsed_json.get_mut("cards").and_then(|c| c.as_array_mut()) {
                let new_card_obj = serde_json::json!({
                    "id": new_card_id,
                    "kind": "basic",
                    "term": "",
                    "definition": "",
                    "prompt": null,
                    "answer": null,
                    "term_lang": null,
                    "def_lang": null,
                    "lexical": null,
                    "notes": null,
                    "media": [],
                    "tags": [],
                    "examples": [],
                    "occlusion": null
                });
                cards.push(new_card_obj);
            }
            if let Ok(updated_json) = serde_json::to_string_pretty(&parsed_json) {
                app.raw_schema_text = updated_json;
            }
        }
        app.load_image(ui.ctx());
        core_needs_sync = true;
    }

    if action_add_sentence || action_remove_sentence.is_some() {
        app.push_snapshot();
        if let Some(deck) = &mut app.deck {
            let card = &mut deck.cards[app.selected_index];

            if action_add_sentence {
                card.examples
                    .push(crate::models::Example::Text(String::new()));
            }
            if let Some(idx) = action_remove_sentence {
                card.examples.remove(idx);
            }
        }
        json_needs_sync = true;
        core_needs_sync = true;
    }

    if json_needs_sync {
        super::json_sync::sync_current_card(app);
    }

    if app.editor_mode && app.show_images {
        if let Some(path) = super::drag_drop::find_dropped_image(ui.ctx()) {
            super::drag_drop::process_image(app, ui.ctx(), path);
            core_needs_sync = true;
        }
    }

    if go_back {
        app.workspace = crate::Workspace::Browse;
    }

    if core_needs_sync {
        app.sync_card_to_core(app.selected_index);
    }

    if action_save {
        app.save_deck();
    }

    if trigger_speak {
        super::audio_actions::handle_speak(
            app,
            &deck_term_fallback,
            &deck_def_fallback,
        );
    }
}