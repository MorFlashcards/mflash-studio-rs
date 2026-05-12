use crate::MFlashStudioApp;
use eframe::egui;

fn update_find_matches(app: &mut MFlashStudioApp) {
    app.find_matches.clear();

    if app.find_query.is_empty() {
        app.current_match_idx = 0;
        return;
    }

    if app.find_use_regex {
        let mut builder = regex::RegexBuilder::new(&app.find_query);
        builder.case_insensitive(!app.find_case_sensitive);

        if let Ok(re) = builder.build() {
            for mat in re.find_iter(&app.raw_json) {
                app.find_matches.push(mat.start()..mat.end());
            }
        }
    } else {
        let haystack = if app.find_case_sensitive {
            app.raw_json.clone()
        } else {
            app.raw_json.to_ascii_lowercase()
        };

        let needle = if app.find_case_sensitive {
            app.find_query.clone()
        } else {
            app.find_query.to_ascii_lowercase()
        };

        if needle.is_empty() {
            app.current_match_idx = 0;
            return;
        }

        let mut start = 0;

        while start <= haystack.len() {
            if let Some(idx) = haystack[start..].find(&needle) {
                let actual_idx = start + idx;
                app.find_matches
                    .push(actual_idx..(actual_idx + needle.len()));
                start = actual_idx + needle.len();
            } else {
                break;
            }
        }
    }

    if app.find_matches.is_empty() {
        app.current_match_idx = 0;
    } else if app.current_match_idx >= app.find_matches.len() {
        app.current_match_idx = app.find_matches.len() - 1;
    }
}

fn replace_all_case_insensitive_ascii(text: &str, find: &str, replace: &str) -> String {
    if find.is_empty() {
        return text.to_string();
    }

    let haystack = text.to_ascii_lowercase();
    let needle = find.to_ascii_lowercase();

    let mut result = String::new();
    let mut last = 0;
    let mut start = 0;

    while start <= haystack.len() {
        if let Some(idx) = haystack[start..].find(&needle) {
            let actual_idx = start + idx;

            result.push_str(&text[last..actual_idx]);
            result.push_str(replace);

            start = actual_idx + needle.len();
            last = start;
        } else {
            break;
        }
    }

    result.push_str(&text[last..]);
    result
}

fn build_find_regex(app: &MFlashStudioApp) -> Option<regex::Regex> {
    let mut builder = regex::RegexBuilder::new(&app.find_query);
    builder.case_insensitive(!app.find_case_sensitive);
    builder.build().ok()
}

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    let mut go_back = false;
    let mut apply_changes = false;
    let mut action_save = false;

    ui.horizontal(|ui| {
        ui.heading("Raw deck.json");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("⮜ Cancel & Back").clicked() {
                if let Some(deck) = &app.deck {
                    if let Ok(json) = serde_json::to_string_pretty(deck) {
                        app.raw_json = json;
                    }
                } else {
                    app.raw_json.clear();
                }
                app.json_error = None;
                go_back = true;
            }

            if ui.button("💾 Apply & Save").clicked() {
                apply_changes = true;
                action_save = true;
            }
        });
    });

    ui.separator();

    if app.find_visible {
        egui::Frame::window(ui.style()).show(ui, |ui| {
            ui.horizontal(|ui| {
                let find_changed = ui
                    .add(
                        egui::TextEdit::singleline(&mut app.find_query)
                            .hint_text("Find")
                            .desired_width(220.0),
                    )
                    .changed();

                let case_changed = ui
                    .selectable_label(app.find_case_sensitive, "Aa")
                    .on_hover_text("Case Sensitive")
                    .clicked();

                if case_changed {
                    app.find_case_sensitive = !app.find_case_sensitive;
                }

                let regex_changed = ui
                    .selectable_label(app.find_use_regex, ".*")
                    .on_hover_text("Use Regular Expression")
                    .clicked();

                if regex_changed {
                    app.find_use_regex = !app.find_use_regex;
                }

                if find_changed || case_changed || regex_changed {
                    app.current_match_idx = 0;
                    update_find_matches(app);
                }

                let match_count = app.find_matches.len();

                if match_count > 0 {
                    ui.label(format!("{} of {}", app.current_match_idx + 1, match_count));
                } else if app.find_use_regex
                    && !app.find_query.is_empty()
                    && build_find_regex(app).is_none()
                {
                    ui.label(egui::RichText::new("Invalid regex").color(egui::Color32::RED));
                } else {
                    ui.label(egui::RichText::new("No results").color(egui::Color32::RED));
                }

                if ui.button("⮝").clicked() && match_count > 0 {
                    app.current_match_idx = (app.current_match_idx + match_count - 1) % match_count;
                }

                if ui.button("⮟").clicked() && match_count > 0 {
                    app.current_match_idx = (app.current_match_idx + 1) % match_count;
                }

                if ui.button("❌").clicked() {
                    app.find_visible = false;
                }
            });

            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut app.replace_query)
                        .hint_text("Replace")
                        .desired_width(220.0),
                );

                if ui.button("Replace").clicked() && !app.find_matches.is_empty() {
                    let range = app.find_matches[app.current_match_idx].clone();

                    if app.find_use_regex {
                        if let Some(re) = build_find_regex(app) {
                            let replacement = re
                                .replace(&app.raw_json[range.clone()], app.replace_query.as_str())
                                .to_string();

                            app.raw_json.replace_range(range, &replacement);
                        }
                    } else {
                        app.raw_json.replace_range(range, &app.replace_query);
                    }

                    update_find_matches(app);
                }

                if ui.button("Replace All").clicked() && !app.find_query.is_empty() {
                    if app.find_use_regex {
                        if let Some(re) = build_find_regex(app) {
                            app.raw_json = re
                                .replace_all(&app.raw_json, app.replace_query.as_str())
                                .to_string();
                        }
                    } else if app.find_case_sensitive {
                        app.raw_json = app.raw_json.replace(&app.find_query, &app.replace_query);
                    } else {
                        app.raw_json = replace_all_case_insensitive_ascii(
                            &app.raw_json,
                            &app.find_query,
                            &app.replace_query,
                        );
                    }

                    update_find_matches(app);
                }
            });
        });

        ui.add_space(8.0);
    }

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

            if response.changed() && app.find_visible {
                update_find_matches(app);
            }

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

                    if app.find_visible {
                        update_find_matches(app);
                    }

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

                            if app.find_visible {
                                update_find_matches(app);
                            }
                        }
                    }
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("✂ Cut All JSON").clicked() {
                    ui.output_mut(|o| o.copied_text = app.raw_json.clone());
                    app.raw_json.clear();

                    if app.find_visible {
                        update_find_matches(app);
                    }

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
        app.workspace = crate::Workspace::List;
    }
}
