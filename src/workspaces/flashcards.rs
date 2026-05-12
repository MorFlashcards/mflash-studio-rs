use crate::MFlashStudioApp;
use eframe::egui;

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    let mut go_back = false;
    let mut trigger_speak = false;
    let mut dropped_image_path = None;

    let mut json_needs_sync = false;
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
        ui.horizontal(|ui| {
            if ui.button("⮜ Back").clicked() {
                go_back = true;
            }
            ui.label(format!(
                "Card {} of {}",
                app.selected_index + 1,
                total_cards
            ));

            if app.editor_mode {
                ui.add_space(10.0);
                if ui.button("➕ Add New Card").clicked() {
                    action_add_card = true;
                }
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🔊 Speak").clicked() {
                    trigger_speak = true;
                }

                // Audio toggles
                ui.checkbox(&mut app.enable_tts, "🤖 TTS");
                ui.checkbox(&mut app.enable_media_audio, "🎵 File");
                ui.label(egui::RichText::new("Audio:").weak());

                if app.editor_mode {
                    ui.add_space(10.0);
                    if ui.button("💾 Save").clicked() {
                        action_save = true;
                    }
                }
            });
        });

        ui.separator();

        // --- QUICK ACCESS OPTIONS BAR ---
        if app.editor_mode {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 16.0;
                ui.label(egui::RichText::new("👁 View:").strong());
                ui.checkbox(&mut app.show_lang_codes, "Language Codes");
                ui.checkbox(&mut app.show_phonetic, "Phonetic");
                ui.checkbox(&mut app.show_part_of_speech, "Part of Speech");
                ui.checkbox(&mut app.show_notes, "Notes");
                ui.checkbox(&mut app.show_tags, "Tags");
            });
            ui.separator();
        }

        ui.columns(2, |cols| {
            // --- LEFT COLUMN: TEXT ---
            cols[0].vertical(|ui| {
                ui.add_space(20.0);

                if app.editor_mode {
                    // --- TERM HEADER & LANGUAGE ---
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Term")
                                .strong()
                                .color(egui::Color32::GRAY),
                        );

                        if app.show_lang_codes {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    let mut current_lang =
                                        card.term_lang.clone().unwrap_or_default();

                                    // Determine what text to show on the closed dropdown button
                                    let display_text = if current_lang.is_empty() {
                                        format!("Default: {}", deck_term_fallback)
                                    } else {
                                        current_lang.clone()
                                    };

                                    let r = egui::ComboBox::from_id_source("term_lang_combo")
                                        .selected_text(display_text)
                                        .width(150.0)
                                        .show_ui(ui, |ui| {
                                            // 1. The Search Box inside the dropdown
                                            ui.add(
                                                egui::TextEdit::singleline(
                                                    &mut app.lang_search_query,
                                                )
                                                .hint_text("Search..."),
                                            )
                                            .request_focus();

                                            ui.separator();

                                            let search_lower = app.lang_search_query.to_lowercase();
                                            let mut changed = false;

                                            // 2. Filter and display options
                                            for lang in crate::audio::bcp47::SUPPORTED_LANGUAGES {
                                                if lang
                                                    .display_name
                                                    .to_lowercase()
                                                    .contains(&search_lower)
                                                    || lang
                                                        .bcp_47
                                                        .to_lowercase()
                                                        .contains(&search_lower)
                                                {
                                                    if ui
                                                        .selectable_value(
                                                            &mut current_lang,
                                                            lang.bcp_47.to_string(),
                                                            format!(
                                                                "{} ({})",
                                                                lang.display_name, lang.bcp_47
                                                            ),
                                                        )
                                                        .clicked()
                                                    {
                                                        changed = true;
                                                    }
                                                }
                                            }

                                            // Allow them to use whatever custom code they typed if they really want to
                                            if !search_lower.is_empty()
                                                && ui
                                                    .button(format!(
                                                        "Use custom: '{}'",
                                                        app.lang_search_query
                                                    ))
                                                    .clicked()
                                            {
                                                current_lang = app.lang_search_query.clone();
                                                changed = true;
                                            }

                                            changed
                                        });

                                    ui.label(egui::RichText::new("🗣 Language:").weak().size(12.0));

                                    // 3. Handle state updates if they picked something
                                    if let Some(inner_response) = r.inner {
                                        if inner_response {
                                            card.term_lang = if current_lang.trim().is_empty() {
                                                None
                                            } else {
                                                Some(current_lang.trim().to_string())
                                            };
                                            app.lang_search_query.clear(); // Clear search for next time
                                            json_needs_sync = true;
                                        }
                                    }
                                },
                            );
                        }
                    });

                    let term_resp = ui.add(
                        egui::TextEdit::singleline(&mut card.term)
                            .font(egui::TextStyle::Heading)
                            .frame(false)
                            .desired_width(f32::INFINITY),
                    );
                    if term_resp.changed() {
                        json_needs_sync = true;
                    }

                    // --- OPTIONAL: PHONETIC & POS ---
                    if app.show_phonetic {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Phonetic:").weak().size(12.0));
                            let mut phonetic_str = card.phonetic.clone().unwrap_or_default();
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut phonetic_str)
                                        .desired_width(150.0),
                                )
                                .changed()
                            {
                                card.phonetic = if phonetic_str.trim().is_empty() {
                                    None
                                } else {
                                    Some(phonetic_str.trim().to_string())
                                };
                                json_needs_sync = true;
                            }
                        });
                    }

                    if app.show_part_of_speech {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Part of Speech:").weak().size(12.0));
                            let mut pos_str = card.part_of_speech.clone().unwrap_or_default();
                            if ui
                                .add(egui::TextEdit::singleline(&mut pos_str).desired_width(150.0))
                                .changed()
                            {
                                card.part_of_speech = if pos_str.trim().is_empty() {
                                    None
                                } else {
                                    Some(pos_str.trim().to_string())
                                };
                                json_needs_sync = true;
                            }
                        });
                    }

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // --- DEFINITION HEADER & LANGUAGE ---
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Definition")
                                .strong()
                                .color(egui::Color32::GRAY),
                        );

                        if app.show_lang_codes {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    let mut current_lang =
                                        card.def_lang.clone().unwrap_or_default();

                                    // Determine what text to show on the closed dropdown button
                                    let display_text = if current_lang.is_empty() {
                                        format!("Default: {}", deck_def_fallback)
                                    } else {
                                        current_lang.clone()
                                    };

                                    let r = egui::ComboBox::from_id_source("def_lang_combo")
                                        .selected_text(display_text)
                                        .width(150.0)
                                        .show_ui(ui, |ui| {
                                            // 1. The Search Box inside the dropdown
                                            ui.add(
                                                egui::TextEdit::singleline(
                                                    &mut app.lang_search_query,
                                                )
                                                .hint_text("Search..."),
                                            )
                                            .request_focus();

                                            ui.separator();

                                            let search_lower = app.lang_search_query.to_lowercase();
                                            let mut changed = false;

                                            // 2. Filter and display options
                                            for lang in crate::audio::bcp47::SUPPORTED_LANGUAGES {
                                                if lang
                                                    .display_name
                                                    .to_lowercase()
                                                    .contains(&search_lower)
                                                    || lang
                                                        .bcp_47
                                                        .to_lowercase()
                                                        .contains(&search_lower)
                                                {
                                                    if ui
                                                        .selectable_value(
                                                            &mut current_lang,
                                                            lang.bcp_47.to_string(),
                                                            format!(
                                                                "{} ({})",
                                                                lang.display_name, lang.bcp_47
                                                            ),
                                                        )
                                                        .clicked()
                                                    {
                                                        changed = true;
                                                    }
                                                }
                                            }

                                            // Allow them to use whatever custom code they typed if they really want to
                                            if !search_lower.is_empty()
                                                && ui
                                                    .button(format!(
                                                        "Use custom: '{}'",
                                                        app.lang_search_query
                                                    ))
                                                    .clicked()
                                            {
                                                current_lang = app.lang_search_query.clone();
                                                changed = true;
                                            }

                                            changed
                                        });

                                    ui.label(egui::RichText::new("🗣 Language:").weak().size(12.0));

                                    // 3. Handle state updates if they picked something
                                    if let Some(inner_response) = r.inner {
                                        if inner_response {
                                            card.def_lang = if current_lang.trim().is_empty() {
                                                None
                                            } else {
                                                Some(current_lang.trim().to_string())
                                            };
                                            app.lang_search_query.clear(); // Clear search for next time
                                            json_needs_sync = true;
                                        }
                                    }
                                },
                            );
                        }
                    });

                    let def_resp = ui.add(
                        egui::TextEdit::multiline(&mut card.definition)
                            .font(egui::TextStyle::Body)
                            .frame(false)
                            .desired_width(f32::INFINITY),
                    );
                    if def_resp.changed() {
                        json_needs_sync = true;
                    }

                    // --- OPTIONAL: NOTES ---
                    if app.show_notes {
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new("Notes:").weak().size(12.0));
                        let mut notes_str = card.notes.clone().unwrap_or_default();
                        if ui
                            .add(
                                egui::TextEdit::multiline(&mut notes_str)
                                    .frame(false)
                                    .desired_width(f32::INFINITY),
                            )
                            .changed()
                        {
                            card.notes = if notes_str.trim().is_empty() {
                                None
                            } else {
                                Some(notes_str.trim().to_string())
                            };
                            json_needs_sync = true;
                        }
                    }

                    // --- OPTIONAL: TAGS ---
                    if app.show_tags {
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Tags (comma separated):")
                                    .weak()
                                    .size(12.0),
                            );
                            let mut tags_str = card.tags.join(", ");
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut tags_str)
                                        .desired_width(f32::INFINITY),
                                )
                                .changed()
                            {
                                card.tags = tags_str
                                    .split(',')
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect();
                                json_needs_sync = true;
                            }
                        });
                    }

                    ui.add_space(15.0);
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Examples:").weak());
                        if ui.button("➕").on_hover_text("Add Example").clicked() {
                            action_add_sentence = true;
                        }
                    });

                    // Render Example enum list in edit mode
                    for (i, example) in card.examples.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label("•");

                            let text_ref = match example {
                                crate::models::Example::Text(s) => s,
                                crate::models::Example::Detailed(info) => &mut info.text,
                            };

                            let resp = ui.add(
                                egui::TextEdit::multiline(text_ref)
                                    .font(egui::TextStyle::Body)
                                    .frame(false)
                                    .desired_width(ui.available_width() - 30.0),
                            );

                            if resp.changed() {
                                json_needs_sync = true;
                            }

                            if ui.button("❌").clicked() {
                                action_remove_sentence = Some(i);
                            }
                        });
                    }
                } else {
                    // Static Reader Mode
                    ui.heading(
                        egui::RichText::new(&card.term)
                            .size(app.config.ui.font_size_header)
                            .strong(),
                    );

                    if let Some(phonetic) = &card.phonetic {
                        ui.label(
                            egui::RichText::new(format!("/ {} /", phonetic))
                                .color(egui::Color32::GRAY),
                        );
                    }
                    if let Some(pos) = &card.part_of_speech {
                        ui.label(egui::RichText::new(pos).italics().size(12.0));
                    }

                    ui.separator();
                    ui.label(
                        egui::RichText::new(&card.definition).size(app.config.ui.font_size_body),
                    );

                    if !card.examples.is_empty() {
                        ui.add_space(15.0);
                        ui.label(egui::RichText::new("Examples:").weak());
                        for example in &card.examples {
                            let text_ref = match example {
                                crate::models::Example::Text(s) => s,
                                crate::models::Example::Detailed(info) => &info.text,
                            };
                            ui.label(egui::RichText::new(format!("• \"{}\"", text_ref)).italics());
                        }
                    }

                    if let Some(notes) = &card.notes {
                        ui.add_space(15.0);
                        ui.label(egui::RichText::new("Notes:").strong().size(12.0));
                        ui.label(notes);
                    }

                    if !card.tags.is_empty() {
                        ui.add_space(10.0);
                        ui.horizontal_wrapped(|ui| {
                            for tag in &card.tags {
                                ui.label(egui::RichText::new(format!("#{}", tag)).weak());
                            }
                        });
                    }
                }

                if let Some(url) = &card.hyperlink {
                    ui.add_space(15.0);
                    ui.hyperlink_to("🔗 View Reference Link", url);
                }
            });

            // --- RIGHT COLUMN: IMAGES ---
            cols[1].vertical_centered(|ui| {
                ui.add_space(20.0);

                if app.show_images {
                    if let Some(tex) = &app.current_texture {
                        ui.add(egui::Image::from_texture(tex).max_width(ui.available_width()));
                    } else if app.editor_mode {
                        ui.add_space(40.0);
                        egui::Frame::none()
                            .fill(egui::Color32::from_black_alpha(20))
                            .stroke(egui::Stroke::new(1.0, egui::Color32::DARK_GRAY))
                            .rounding(8.0)
                            .inner_margin(egui::Margin::same(40.0))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new("📥 Drag & Drop Image Here")
                                        .size(16.0)
                                        .weak(),
                                );
                            });
                    }
                } else {
                    ui.add_space(40.0);
                    ui.label(egui::RichText::new("🚫 Images Hidden").weak().italics());
                }
            });
        });

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
                term: String::new(),
                definition: String::new(),
                term_lang: None,
                def_lang: None,
                phonetic: None,
                part_of_speech: None,
                notes: None,
                hyperlink: None,
                media: Vec::new(),
                tags: Vec::new(),
                examples: Vec::new(),
            };
            deck.cards.push(new_card);
            app.selected_index = deck.cards.len() - 1;
        }

        if let Ok(mut parsed_json) = serde_json::from_str::<serde_json::Value>(&app.raw_json) {
            if let Some(cards) = parsed_json.get_mut("cards").and_then(|c| c.as_array_mut()) {
                let new_card_obj = serde_json::json!({
                    "id": new_card_id,
                    "term": "",
                    "definition": "",
                    "term_lang": null,
                    "def_lang": null,
                    "phonetic": null,
                    "part_of_speech": null,
                    "notes": null,
                    "hyperlink": null,
                    "media": [],
                    "tags": [],
                    "examples": []
                });
                cards.push(new_card_obj);
            }
            if let Ok(updated_json) = serde_json::to_string_pretty(&parsed_json) {
                app.raw_json = updated_json;
            }
        }
        app.load_image(ui.ctx());
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
    }

    if json_needs_sync {
        if let Ok(mut parsed_json) = serde_json::from_str::<serde_json::Value>(&app.raw_json) {
            if let Some(cards) = parsed_json.get_mut("cards").and_then(|c| c.as_array_mut()) {
                if let Some(card_val) = cards.get_mut(app.selected_index) {
                    if let Some(card_obj) = card_val.as_object_mut() {
                        if let Some(deck) = &app.deck {
                            let card = &deck.cards[app.selected_index];

                            // Core
                            card_obj.insert(
                                "term".to_string(),
                                serde_json::Value::String(card.term.clone()),
                            );
                            card_obj.insert(
                                "definition".to_string(),
                                serde_json::Value::String(card.definition.clone()),
                            );

                            // Optional Strings mapped to Null if empty
                            let mut insert_opt_str = |key: &str, val: &Option<String>| {
                                if let Some(s) = val {
                                    card_obj.insert(
                                        key.to_string(),
                                        serde_json::Value::String(s.clone()),
                                    );
                                } else {
                                    card_obj.insert(key.to_string(), serde_json::Value::Null);
                                }
                            };

                            insert_opt_str("term_lang", &card.term_lang);
                            insert_opt_str("def_lang", &card.def_lang);
                            insert_opt_str("phonetic", &card.phonetic);
                            insert_opt_str("part_of_speech", &card.part_of_speech);
                            insert_opt_str("notes", &card.notes);

                            // Tags array
                            let json_tags = card
                                .tags
                                .iter()
                                .map(|t| serde_json::Value::String(t.clone()))
                                .collect();
                            card_obj
                                .insert("tags".to_string(), serde_json::Value::Array(json_tags));

                            // Sync Enum examples back to JSON
                            let json_examples = card
                                .examples
                                .iter()
                                .map(|ex| match ex {
                                    crate::models::Example::Text(s) => {
                                        serde_json::Value::String(s.clone())
                                    }
                                    crate::models::Example::Detailed(info) => {
                                        serde_json::json!(info)
                                    }
                                })
                                .collect::<Vec<_>>();

                            card_obj.insert(
                                "examples".to_string(),
                                serde_json::Value::Array(json_examples),
                            );
                        }
                    }
                }
            }
            if let Ok(updated_json) = serde_json::to_string_pretty(&parsed_json) {
                app.raw_json = updated_json;
            }
        }
    }

    if app.editor_mode && app.show_images {
        ui.ctx().input(|i| {
            for file in &i.raw.dropped_files {
                if let Some(path) = &file.path {
                    let ext = path
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("")
                        .to_lowercase();
                    if ext == "png"
                        || ext == "jpg"
                        || ext == "jpeg"
                        || ext == "webp"
                        || ext == "gif"
                    {
                        dropped_image_path = Some(path.to_string_lossy().to_string());
                        break;
                    }
                }
            }
        });

        if let Some(path) = dropped_image_path {
            if app.deck.is_some() {
                app.push_snapshot();

                if let Some(deck) = &mut app.deck {
                    deck.cards[app.selected_index].media = vec![crate::models::MediaInfo {
                        src: path.clone(),
                        media_type: "image".to_string(),
                        alt: None,
                        description: None,
                    }];
                }

                if let Ok(mut parsed_json) =
                    serde_json::from_str::<serde_json::Value>(&app.raw_json)
                {
                    if let Some(cards) = parsed_json.get_mut("cards").and_then(|c| c.as_array_mut())
                    {
                        if let Some(card_val) = cards.get_mut(app.selected_index) {
                            if let Some(card_obj) = card_val.as_object_mut() {
                                let mut new_media = serde_json::Map::new();
                                new_media.insert(
                                    "src".to_string(),
                                    serde_json::Value::String(path.clone()),
                                );
                                new_media.insert(
                                    "type".to_string(),
                                    serde_json::Value::String("image".to_string()),
                                );

                                // Best effort to preserve alt tag if replacing media
                                if let Some(media_arr) =
                                    card_obj.get("media").and_then(|m| m.as_array())
                                {
                                    if let Some(first_media) =
                                        media_arr.first().and_then(|m| m.as_object())
                                    {
                                        if let Some(alt) = first_media.get("alt") {
                                            new_media.insert("alt".to_string(), alt.clone());
                                        }
                                    }
                                }

                                card_obj.insert(
                                    "media".to_string(),
                                    serde_json::Value::Array(vec![serde_json::Value::Object(
                                        new_media,
                                    )]),
                                );
                            }
                        }
                    }
                    if let Ok(updated_json) = serde_json::to_string_pretty(&parsed_json) {
                        app.raw_json = updated_json;
                    }
                }
                app.load_image(ui.ctx());
            }
        }
    }

    if go_back {
        app.workspace = crate::Workspace::List;
    }

    if action_save {
        app.save_deck();
    }

    if trigger_speak {
        if let Some(deck) = &app.deck {
            let card = &deck.cards[app.selected_index];

            let term_lang = card.term_lang.as_deref().unwrap_or(&deck_term_fallback);
            let def_lang = card.def_lang.as_deref().unwrap_or(&deck_def_fallback);

            let mut played_media = false;

            // 1. Try to find and play attached audio media first
            if app.enable_media_audio {
                if let Some(audio_meta) = card.media.iter().find(|m| m.media_type == "audio") {
                    // NOTE: You will need an audio crate like `rodio` in audio.rs to actually
                    // decode and play the .mp3 / .wav file located at audio_meta.src.
                    println!("Found pre-recorded audio file to play: {}", audio_meta.src);

                    // app.audio.play_file(&audio_meta.src);
                    played_media = true;
                }
            }

            // 2. Fallback to native TTS if no file played
            if app.enable_tts && !played_media {
                app.audio.speak(&card.term, Some(term_lang), true);
                app.audio.speak(&card.definition, Some(def_lang), false);
            }
        }
    }
}