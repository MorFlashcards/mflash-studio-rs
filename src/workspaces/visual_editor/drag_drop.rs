// src/workspaces/visual_editor/drag_drop.rs

use crate::MFlashStudioApp;
use eframe::egui;

pub fn find_dropped_image(ctx: &egui::Context) -> Option<String> {
    let mut dropped_image_path = None;

    ctx.input(|input| {
        for file in &input.raw.dropped_files {
            if let Some(path) = &file.path {
                let ext = path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                if matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "webp" | "gif") {
                    dropped_image_path = Some(path.to_string_lossy().to_string());
                    break;
                }
            }
        }
    });

    dropped_image_path
}

pub fn process_image(app: &mut MFlashStudioApp, ctx: &egui::Context, path: String) {
    if app.deck.is_none() {
        return;
    }

    app.push_snapshot();

    if let Some(deck) = &mut app.deck {
        deck.cards[app.selected_index].media = vec![crate::models::MediaInfo {
            id: None,
            src: path.clone(),
            media_type: "image".to_string(),
            role: Some("illustration".to_string()),
            alt: None,
            description: None,
        }];
    }

    sync_image_to_json(app, &path);
    app.load_image(ctx);
}

fn sync_image_to_json(app: &mut MFlashStudioApp, path: &str) {
    if let Ok(mut parsed_json) = serde_json::from_str::<serde_json::Value>(&app.raw_schema_text) {
        if let Some(cards) = parsed_json.get_mut("cards").and_then(|cards| cards.as_array_mut()) {
            if let Some(card_val) = cards.get_mut(app.selected_index) {
                if let Some(card_obj) = card_val.as_object_mut() {
                    let mut new_media = serde_json::Map::new();

                    new_media.insert("id".to_string(), serde_json::Value::Null);
                    new_media.insert(
                        "src".to_string(),
                        serde_json::Value::String(path.to_string()),
                    );
                    new_media.insert(
                        "type".to_string(),
                        serde_json::Value::String("image".to_string()),
                    );
                    new_media.insert(
                        "role".to_string(),
                        serde_json::Value::String("illustration".to_string()),
                    );

                    // Best effort to preserve alt tag if replacing media.
                    if let Some(media_arr) = card_obj.get("media").and_then(|media| media.as_array())
                    {
                        if let Some(first_media) = media_arr.first().and_then(|media| media.as_object())
                        {
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
            app.raw_schema_text = updated_json;
        }
    }
}