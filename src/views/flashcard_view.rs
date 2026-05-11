use eframe::egui;
use crate::MFlashStudioApp;

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    let mut go_back = false;
    let mut trigger_speak = false;
    let mut dropped_image_path = None;
    
    let mut json_needs_sync = false;
    let mut action_add_sentence = false;
    let mut action_remove_sentence = None;

    if let Some(deck) = &mut app.deck {
        let total_cards = deck.cards.len();
        let card = &mut deck.cards[app.selected_index];
        
        ui.horizontal(|ui| {
            if ui.button("⮜ Back").clicked() { go_back = true; }
            ui.label(format!("Card {} of {}", app.selected_index + 1, total_cards));
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🔊 Speak").clicked() { trigger_speak = true; }
            });
        });

        ui.separator();
        ui.columns(2, |cols| {
            
            // --- LEFT COLUMN: TEXT ---
            cols[0].vertical(|ui| {
                ui.add_space(20.0);
                
                if app.editor_mode {
                    let term_resp = ui.add(egui::TextEdit::singleline(&mut card.term)
                        .font(egui::TextStyle::Heading)
                        .frame(false)
                        .desired_width(f32::INFINITY));
                    if term_resp.changed() { json_needs_sync = true; }
                    
                    ui.separator();
                    
                    let def_resp = ui.add(egui::TextEdit::multiline(&mut card.definition)
                        .font(egui::TextStyle::Body)
                        .frame(false)
                        .desired_width(f32::INFINITY));
                    if def_resp.changed() { json_needs_sync = true; }
                    
                    ui.add_space(15.0);
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Examples:").weak());
                        if ui.button("➕").on_hover_text("Add Example").clicked() {
                            action_add_sentence = true;
                        }
                    });

                    if let Some(sentences) = &mut card.example_sentences {
                        for (i, sentence) in sentences.iter_mut().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label("•");
                                let resp = ui.add(egui::TextEdit::multiline(sentence)
                                    .font(egui::TextStyle::Body)
                                    .frame(false)
                                    .desired_width(ui.available_width() - 30.0)); 
                                
                                if resp.changed() { json_needs_sync = true; }
                                
                                if ui.button("❌").clicked() {
                                    action_remove_sentence = Some(i);
                                }
                            });
                        }
                    }
                } else {
                    // Static Reader Mode
                    ui.heading(egui::RichText::new(&card.term).size(app.config.ui.font_size_header).strong());
                    ui.separator();
                    ui.label(egui::RichText::new(&card.definition).size(app.config.ui.font_size_body));
                    
                    if let Some(sentences) = &card.example_sentences {
                        if !sentences.is_empty() {
                            ui.add_space(15.0);
                            ui.label(egui::RichText::new("Examples:").weak());
                            for sentence in sentences {
                                ui.label(egui::RichText::new(format!("• \"{}\"", sentence)).italics());
                            }
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
                                ui.label(egui::RichText::new("📥 Drag & Drop Image Here").size(16.0).weak());
                            });
                    }
                } else {
                    // Images explicitly toggled off
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

    if action_add_sentence || action_remove_sentence.is_some() {
        app.push_snapshot(); 
        if let Some(deck) = &mut app.deck {
            let card = &mut deck.cards[app.selected_index];
            
            if action_add_sentence {
                if card.example_sentences.is_none() {
                    card.example_sentences = Some(Vec::new());
                }
                card.example_sentences.as_mut().unwrap().push(String::new());
            }
            if let Some(idx) = action_remove_sentence {
                if let Some(sentences) = &mut card.example_sentences {
                    sentences.remove(idx);
                }
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
                            
                            card_obj.insert("term".to_string(), serde_json::Value::String(card.term.clone()));
                            card_obj.insert("definition".to_string(), serde_json::Value::String(card.definition.clone()));
                            
                            if let Some(sentences) = &card.example_sentences {
                                card_obj.insert("example_sentences".to_string(), serde_json::Value::Array(
                                    sentences.iter().map(|s| serde_json::Value::String(s.clone())).collect()
                                ));
                            } else {
                                card_obj.insert("example_sentences".to_string(), serde_json::Value::Array(vec![]));
                            }
                        }
                    }
                }
            }
            if let Ok(updated_json) = serde_json::to_string_pretty(&parsed_json) {
                app.raw_json = updated_json;
            }
        }
    }

    // Only allow Drag & Drop if Editor Mode is active
    if app.editor_mode && app.show_images {
        ui.ctx().input(|i| {
            for file in &i.raw.dropped_files {
                if let Some(path) = &file.path {
                    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                    if ext == "png" || ext == "jpg" || ext == "jpeg" || ext == "webp" || ext == "gif" {
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
                    deck.cards[app.selected_index].media = Some(crate::models::MediaInfo {
                        path: path.clone(),
                    });
                }
                
                if let Ok(mut parsed_json) = serde_json::from_str::<serde_json::Value>(&app.raw_json) {
                    if let Some(cards) = parsed_json.get_mut("cards").and_then(|c| c.as_array_mut()) {
                        if let Some(card_val) = cards.get_mut(app.selected_index) {
                            if let Some(card_obj) = card_val.as_object_mut() {
                                let mut new_media = serde_json::Map::new();
                                new_media.insert("path".to_string(), serde_json::Value::String(path.clone()));
                                new_media.insert("type".to_string(), serde_json::Value::String("image".to_string()));
                                
                                if let Some(old_media) = card_obj.get("media").and_then(|m| m.as_object()) {
                                    if let Some(alt) = old_media.get("alt") {
                                        new_media.insert("alt".to_string(), alt.clone());
                                    }
                                }
                                
                                card_obj.insert("media".to_string(), serde_json::Value::Object(new_media));
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

    if go_back { app.mode = crate::ViewMode::List; }
    
    if trigger_speak {
        if let Some(data) = &app.deck {
            let card = &data.cards[app.selected_index];
            app.audio.speak(
                &format!("{}, {}", card.term, card.definition),
                card.term_language.as_deref()
            );
        }
    }
}
