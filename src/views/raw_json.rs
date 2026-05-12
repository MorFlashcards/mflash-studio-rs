use crate::MFlashStudioApp;
use eframe::egui;

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    let mut go_back = false;
    let mut apply_changes = false;
    let mut action_save = false; // NEW

    ui.horizontal(|ui| {
        ui.heading("Raw deck.json");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("⮜ Cancel & Back").clicked() {
                if let Ok(json) = serde_json::to_string_pretty(&app.deck) {
                    app.raw_json = json;
                }
                app.json_error = None;
                go_back = true;
            }

            // NEW: Updated to explicitly save to the physical file
            if ui.button("💾 Apply & Save").clicked() {
                apply_changes = true;
                action_save = true;
            }
        });
    });

    ui.separator();

    if let Some(err) = &app.json_error {
        ui.label(egui::RichText::new(format!("⚠️ JSON Error: {}", err)).color(egui::Color32::RED));
        ui.add_space(10.0);
    }

    egui::ScrollArea::both().show(ui, |ui| {
        ui.horizontal(|ui| {
            let line_count = app.raw_json.split('\n').count().max(1);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.add_space(2.0);

                for i in 1..=line_count {
                    ui.label(egui::RichText::new(format!("{:3}", i)).monospace().weak());
                }
            });

            ui.separator();

            let output = egui::TextEdit::multiline(&mut app.raw_json)
                .font(egui::TextStyle::Monospace)
                .code_editor()
                .desired_width(f32::INFINITY)
                .frame(false)
                .show(ui);

            let response = output.response.clone();

            if let Some(range) = output.cursor_range {
                let p = range.primary.ccursor.index;
                let s = range.secondary.ccursor.index;
                let start = p.min(s);
                let end = p.max(s);

                if start != end {
                    let chars: Vec<char> = app.raw_json.chars().collect();
                    let safe_start = start.min(chars.len());
                    let safe_end = end.min(chars.len());

                    app.last_selected_text = chars[safe_start..safe_end].iter().collect();
                    app.last_cursor_range = Some(safe_start..safe_end);
                } else if response.clicked() {
                    app.last_selected_text.clear();
                    app.last_cursor_range = None;
                }
            }

            if response.secondary_clicked() {
                if let Some(range) = &app.last_cursor_range {
                    if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), response.id) {
                        let ccursor_start = egui::text::CCursor::new(range.start);
                        let ccursor_end = egui::text::CCursor::new(range.end);

                        state
                            .cursor
                            .set_char_range(Some(egui::text::CCursorRange::two(
                                ccursor_start,
                                ccursor_end,
                            )));
                        state.store(ui.ctx(), response.id);
                    }
                }
            }

            response.context_menu(|ui| {
                let has_selection = !app.last_selected_text.is_empty();

                if ui
                    .add_enabled(has_selection, egui::Button::new("✂ Cut Selection"))
                    .clicked()
                {
                    ui.output_mut(|o| o.copied_text = app.last_selected_text.clone());
                    if let Some(range) = &app.last_cursor_range {
                        let chars: Vec<char> = app.raw_json.chars().collect();
                        let mut new_text = String::new();
                        new_text.extend(&chars[..range.start]);
                        new_text.extend(&chars[range.end..]);
                        app.raw_json = new_text;
                    }
                    app.last_selected_text.clear();
                    app.last_cursor_range = None;
                    ui.close_menu();
                }

                if ui
                    .add_enabled(has_selection, egui::Button::new("📄 Copy Selection"))
                    .clicked()
                {
                    ui.output_mut(|o| o.copied_text = app.last_selected_text.clone());
                    ui.close_menu();
                }

                if ui.button("📋 Paste (Insert/Replace)").clicked() {
                    if let Ok(mut clipboard) = arboard::Clipboard::new() {
                        if let Ok(text) = clipboard.get_text() {
                            if let Some(range) = &app.last_cursor_range {
                                let chars: Vec<char> = app.raw_json.chars().collect();
                                let mut new_text = String::new();
                                new_text.extend(&chars[..range.start]);
                                new_text.push_str(&text);
                                new_text.extend(&chars[range.end..]);
                                app.raw_json = new_text;
                            } else if let Some(cursor) = output.cursor_range {
                                let idx = cursor.primary.ccursor.index;
                                let chars: Vec<char> = app.raw_json.chars().collect();
                                let safe_idx = idx.min(chars.len());
                                let mut new_text = String::new();
                                new_text.extend(&chars[..safe_idx]);
                                new_text.push_str(&text);
                                new_text.extend(&chars[safe_idx..]);
                                app.raw_json = new_text;
                            } else {
                                app.raw_json.push_str(&text);
                            }

                            app.last_selected_text.clear();
                            app.last_cursor_range = None;
                        }
                    }
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("✂ Cut All JSON").clicked() {
                    ui.output_mut(|o| o.copied_text = app.raw_json.clone());
                    app.raw_json.clear();
                    ui.close_menu();
                }
                if ui.button("📄 Copy All JSON").clicked() {
                    ui.output_mut(|o| o.copied_text = app.raw_json.clone());
                    ui.close_menu();
                }
            });
        });
    });

    if apply_changes {
        match serde_json::from_str::<crate::models::MFlashDeck>(&app.raw_json) {
            Ok(new_deck) => {
                app.push_snapshot();
                app.deck = Some(new_deck);
                app.json_error = None;

                if let Some(d) = &app.deck {
                    app.selected_index = app.selected_index.min(d.cards.len().saturating_sub(1));
                }

                // NEW: Triggers physical save
                if action_save {
                    app.save_deck();
                }

                go_back = true;
            }
            Err(e) => {
                app.json_error = Some(e.to_string());
            }
        }
    }

    if go_back {
        app.mode = crate::ViewMode::List;
    }
}
