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

                if app.editor_mode {
                    if ui.button("💾 Save").clicked() {
                        action_save = true;
                    }
                }
            });
        });

        ui.separator();
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
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let mut lang_str = card.term_lang.clone().unwrap_or_default();

                            let r = ui.add(
                                egui::TextEdit::singleline(&mut lang_str)
                                    .hint_text(format!("Default: {}", deck_term_fallback))
                                    .desired_width(120.0),
                            );

                            ui.label(egui::RichText::new("🗣 Language:").weak().size(12.0));

                            if r.changed() {
                                card.term_lang = if lang_str.trim().is_empty() {
                                    None
                                } else {
                                    Some(lang_str.trim().to_string())
                                };
                                json_needs_sync = true;
                            }
                        });
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
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let mut lang_str = card.def_lang.clone().unwrap_or_default();

                            let r = ui.add(
                                egui::TextEdit::singleline(&mut lang_str)
                                    .hint_text(format!("Default: {}", deck_def_fallback))
                                    .desired_width(120.0),
                            );

                            ui.label(egui::RichText::new("🗣 Language:").weak().size(12.0));

                            if r.changed() {
                                card.def_lang = if lang_str.trim().is_empty() {
                                    None
                                } else {
                                    Some(lang_str.trim().to_string())
                                };
                                json_needs_sync = true;
                            }
                        });
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
                            ui.label(
                                egui::RichText::new(format!("• \"{}\"", text_ref)).italics(),
                            );
                        }
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
        if let Some(deck) = &mut app.deck {
            let new_card = crate::models::Card {
                id: format!(
                    "card_{}",
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis()
                ),
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
                    "id": format!(
                        "card_{}",
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_millis()
                    ),
                    "term": "",
                    "definition": "",
                    "term_lang": null,
                    "def_lang": null,
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
                card.examples.push(crate::models::Example::Text(String::new()));
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

                            card_obj.insert(
                                "term".to_string(),
                                serde_json::Value::String(card.term.clone()),
                            );
                            card_obj.insert(
                                "definition".to_string(),
                                serde_json::Value::String(card.definition.clone()),
                            );

                            if let Some(tl) = &card.term_lang {
                                card_obj.insert(
                                    "term_lang".to_string(),
                                    serde_json::Value::String(tl.clone()),
                                );
                            } else {
                                card_obj.insert("term_lang".to_string(), serde_json::Value::Null);
                            }

                            if let Some(dl) = &card.def_lang {
                                card_obj.insert(
                                    "def_lang".to_string(),
                                    serde_json::Value::String(dl.clone()),
                                );
                            } else {
                                card_obj.insert("def_lang".to_string(), serde_json::Value::Null);
                            }

                            // Sync Enum examples back to JSON
                            let json_examples = card.examples.iter().map(|ex| {
                                match ex {
                                    crate::models::Example::Text(s) => serde_json::Value::String(s.clone()),
                                    crate::models::Example::Detailed(info) => serde_json::json!(info),
                                }
                            }).collect::<Vec<_>>();
                            
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
                        description: None
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
                                if let Some(media_arr) = card_obj.get("media").and_then(|m| m.as_array()) {
                                    if let Some(first_media) = media_arr.first().and_then(|m| m.as_object()) {
                                        if let Some(alt) = first_media.get("alt") {
                                            new_media.insert("alt".to_string(), alt.clone());
                                        }
                                    }
                                }

                                card_obj.insert(
                                    "media".to_string(),
                                    serde_json::Value::Array(vec![serde_json::Value::Object(new_media)]),
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
        app.mode = crate::ViewMode::List;
    }

    if action_save {
        app.save_deck();
    }

    if trigger_speak {
        if let Some(deck) = &app.deck {
            let card = &deck.cards[app.selected_index];

            let term_lang = card.term_lang.as_deref().unwrap_or(&deck_term_fallback);
            let def_lang = card.def_lang.as_deref().unwrap_or(&deck_def_fallback);

            app.audio.speak(&card.term, Some(term_lang), true);
            app.audio.speak(&card.definition, Some(def_lang), false);
        }
    }
}