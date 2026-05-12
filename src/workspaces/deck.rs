use crate::MFlashStudioApp;
use eframe::egui;

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    let mut json_needs_sync = false;
    let mut action_save = false;
    let mut dropped_cover_path: Option<String> = None;
    let mut adopt_discovered_cover: Option<String> = None;

    ui.horizontal(|ui| {
        ui.heading("Deck");

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("💾 Save").clicked() {
                action_save = true;
            }
        });
    });

    ui.separator();

    // Prefer the model field, then raw JSON, then a likely image already inside the .mflash package.
    let model_cover_path = app.deck.as_ref().and_then(|deck| deck.cover.as_ref().map(|c| c.src.clone()));
    let raw_json_cover_path = root_media_src_from_raw_json(&app.raw_json, "cover");
    let discovered_cover_path = if model_cover_path.is_none() && raw_json_cover_path.is_none() {
        discover_cover_media_in_package(&app.path)
    } else {
        None
    };

    let active_cover_path = model_cover_path
        .clone()
        .or(raw_json_cover_path.clone())
        .or(discovered_cover_path.clone());

    let cover_preview = active_cover_path
        .as_deref()
        .and_then(|cover_path| load_cover_texture(ui.ctx(), &app.path, cover_path));

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if let Some(deck) = &mut app.deck {
                ui.label(egui::RichText::new("Title").strong());
                let title_resp = ui.add(
                    egui::TextEdit::singleline(&mut deck.title).desired_width(f32::INFINITY),
                );

                if title_resp.changed() {
                    json_needs_sync = true;
                }

                ui.add_space(12.0);

                ui.label(egui::RichText::new("Deck ID").strong());
                ui.monospace(&deck.id);

                ui.add_space(12.0);

                ui.label(egui::RichText::new("Description").strong());
                let mut description = deck.description.clone().unwrap_or_default();
                if ui
                    .add(
                        egui::TextEdit::multiline(&mut description)
                            .desired_width(f32::INFINITY)
                            .desired_rows(3),
                    )
                    .changed()
                {
                    deck.description = if description.trim().is_empty() {
                        None
                    } else {
                        Some(description.trim().to_string())
                    };
                    json_needs_sync = true;
                }

                ui.add_space(12.0);

                ui.label(egui::RichText::new("Snippet").strong());
                let mut snippet = deck.snippet.clone().unwrap_or_default();
                if ui
                    .add(egui::TextEdit::singleline(&mut snippet).desired_width(f32::INFINITY))
                    .changed()
                {
                    deck.snippet = if snippet.trim().is_empty() {
                        None
                    } else {
                        Some(snippet.trim().to_string())
                    };
                    json_needs_sync = true;
                }

                ui.add_space(12.0);

                ui.label(egui::RichText::new("Default Term Language").strong());
                let mut term_lang = deck.default_term_lang.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut term_lang).changed() {
                    deck.default_term_lang = if term_lang.trim().is_empty() {
                        None
                    } else {
                        Some(term_lang.trim().to_string())
                    };
                    json_needs_sync = true;
                }

                ui.add_space(12.0);

                ui.label(egui::RichText::new("Default Definition Language").strong());
                let mut def_lang = deck.default_def_lang.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut def_lang).changed() {
                    deck.default_def_lang = if def_lang.trim().is_empty() {
                        None
                    } else {
                        Some(def_lang.trim().to_string())
                    };
                    json_needs_sync = true;
                }

                ui.add_space(12.0);

                ui.label(egui::RichText::new("Deck Tags").strong());
                let mut deck_tags = deck.deck_tags.join(", ");
                if ui
                    .add(egui::TextEdit::singleline(&mut deck_tags).desired_width(f32::INFINITY))
                    .changed()
                {
                    deck.deck_tags = deck_tags
                        .split(',')
                        .map(|tag| tag.trim().to_string())
                        .filter(|tag| !tag.is_empty())
                        .collect();
                    json_needs_sync = true;
                }

                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Cards").strong());
                    ui.label(deck.cards.len().to_string());
                });

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);

                ui.label(egui::RichText::new("Cover thumbnail").strong());

                if let Some(cover_path) = active_cover_path.clone() {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Current cover:").weak());
                        ui.monospace(&cover_path);
                    });

                    ui.add_space(8.0);

                    if let Some(tex) = &cover_preview {
                        ui.add(
                            egui::Image::from_texture(tex)
                                .max_width(260.0)
                                .max_height(260.0),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new(
                                "Could not preview this cover. The path exists in the deck metadata, but the image was not found or decoded.",
                            )
                            .weak()
                            .italics(),
                        );
                    }

                    if discovered_cover_path.as_deref() == Some(cover_path.as_str())
                        && deck.cover.is_none()
                    {
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(
                                    "Found this image inside the .mflash package, but cover is not set.",
                                )
                                .weak()
                                .italics(),
                            );

                            if ui.button("Use as cover").clicked() {
                                adopt_discovered_cover = Some(cover_path.clone());
                            }
                        });
                    }

                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        if ui.button("🗑 Remove cover").clicked() {
                            deck.cover = None;
                            json_needs_sync = true;
                        }

                        ui.label(
                            egui::RichText::new("Drag a new image here to replace it.")
                                .weak()
                                .italics(),
                        );
                    });
                } else {
                    egui::Frame::none()
                        .fill(egui::Color32::from_black_alpha(20))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::DARK_GRAY))
                        .rounding(8.0)
                        .inner_margin(egui::Margin::same(40.0))
                        .show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    egui::RichText::new("📥 Drag & Drop Deck Cover Image Here")
                                        .size(16.0)
                                        .weak(),
                                );
                                ui.add_space(4.0);
                                ui.label(
                                    egui::RichText::new("Supported: png, jpg, jpeg, webp, gif")
                                        .weak()
                                        .italics(),
                                );
                            });
                        });
                }

                ui.add_space(24.0);
            }
        });

    // Detect dropped cover image after the deck UI borrow is finished.
    ui.ctx().input(|i| {
        for file in &i.raw.dropped_files {
            if let Some(path) = &file.path {
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                if is_supported_image_extension(&ext) {
                    dropped_cover_path = Some(path.to_string_lossy().to_string());
                    break;
                }
            }
        }
    });

    if let Some(path) = adopt_discovered_cover {
        app.push_snapshot();

        if let Some(deck) = &mut app.deck {
            deck.cover = Some(crate::models::MediaInfo {
                id: None,
                src: path,
                media_type: "image".to_string(),
                role: Some("cover".to_string()),
                alt: None,
                description: None,
            });
        }

        sync_raw_json_from_deck(app);
    }

    if let Some(path) = dropped_cover_path {
        app.push_snapshot();

        if let Some(deck) = &mut app.deck {
            deck.cover = Some(crate::models::MediaInfo {
                id: None,
                media_type: "image".to_string(),
                role: Some("cover".to_string()),
                src: path,
                alt: Some("Deck cover image".to_string()),
                description: None,
            });
        }

        sync_raw_json_from_deck(app);
    }

    if json_needs_sync {
        sync_raw_json_from_deck(app);
    }

    if action_save {
        app.save_deck();
    }
}

fn sync_raw_json_from_deck(app: &mut MFlashStudioApp) {
    if let Some(deck) = &app.deck {
        if let Ok(updated_json) = serde_json::to_string_pretty(deck) {
            app.raw_json = updated_json;
        }
    }
}

fn root_media_src_from_raw_json(raw_json: &str, key: &str) -> Option<String> {
    serde_json::from_str::<serde_json::Value>(raw_json)
        .ok()
        .and_then(|value| {
            value
                .get(key)
                .and_then(|media| media.get("src"))
                .and_then(|src| src.as_str())
                .map(str::to_string)
        })
        .filter(|s| !s.trim().is_empty())
}

fn load_cover_texture(
    ctx: &egui::Context,
    deck_path: &str,
    cover_path: &str,
) -> Option<egui::TextureHandle> {
    let path = std::path::Path::new(cover_path);

    if path.is_absolute() && path.exists() {
        let bytes = std::fs::read(path).ok()?;
        return load_texture_from_bytes(ctx, "deck_cover_preview_file", &bytes);
    }

    crate::media::load_texture(ctx, deck_path, cover_path)
}

fn load_texture_from_bytes(
    ctx: &egui::Context,
    texture_name: &str,
    bytes: &[u8],
) -> Option<egui::TextureHandle> {
    let img = image::load_from_memory(bytes).ok()?;
    let size = [img.width() as _, img.height() as _];
    let image_buffer = img.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

    Some(ctx.load_texture(
        texture_name,
        color_image,
        egui::TextureOptions::default(),
    ))
}

fn discover_cover_media_in_package(deck_path: &str) -> Option<String> {
    if deck_path.ends_with(".json") || deck_path.ends_with(".mflash.json") {
        return None;
    }

    let file = std::fs::File::open(deck_path).ok()?;
    let mut archive = zip::ZipArchive::new(file).ok()?;

    let mut image_names = Vec::new();

    for i in 0..archive.len() {
        let file = archive.by_index(i).ok()?;
        let name = file.name().to_string();
        let lower = name.to_lowercase();

        if lower == "deck.json" || lower.ends_with('/') {
            continue;
        }

        let ext = std::path::Path::new(&lower)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        if is_supported_image_extension(ext) {
            image_names.push(name);
        }
    }

    image_names.sort_by_key(|name| {
        let lower = name.to_lowercase();

        if lower.contains("cover") {
            0
        } else if lower.contains("thumbnail") || lower.contains("thumb") {
            1
        } else if !lower.contains('/') {
            2
        } else {
            3
        }
    });

    image_names.into_iter().next()
}

fn is_supported_image_extension(ext: &str) -> bool {
    matches!(ext, "png" | "jpg" | "jpeg" | "webp" | "gif")
}
