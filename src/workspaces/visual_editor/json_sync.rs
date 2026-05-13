// src/workspaces/visual_editor/json_sync.rs

use crate::MFlashStudioApp;

pub fn sync_current_card(app: &mut MFlashStudioApp) {
    if let Ok(mut parsed_json) = serde_json::from_str::<serde_json::Value>(&app.raw_schema_text) {
        if let Some(cards) = parsed_json.get_mut("cards").and_then(|c| c.as_array_mut()) {
            if let Some(card_val) = cards.get_mut(app.selected_index) {
                if let Some(card_obj) = card_val.as_object_mut() {
                    if let Some(deck) = &app.deck {
                        let card = &deck.cards[app.selected_index];

                        card_obj.insert("kind".to_string(), serde_json::json!(card.kind));

                        card_obj.insert(
                            "term".to_string(),
                            serde_json::Value::String(card.term.clone().unwrap_or_default()),
                        );

                        card_obj.insert(
                            "definition".to_string(),
                            serde_json::Value::String(
                                card.definition.clone().unwrap_or_default(),
                            ),
                        );

                        insert_opt_str(card_obj, "prompt", &card.prompt);
                        insert_opt_str(card_obj, "answer", &card.answer);
                        insert_opt_str(card_obj, "term_lang", &card.term_lang);
                        insert_opt_str(card_obj, "def_lang", &card.def_lang);
                        insert_opt_str(card_obj, "notes", &card.notes);

                        if let Some(lexical) = &card.lexical {
                            card_obj.insert("lexical".to_string(), serde_json::json!(lexical));
                        } else {
                            card_obj.insert("lexical".to_string(), serde_json::Value::Null);
                        }

                        if let Some(occlusion) = &card.occlusion {
                            card_obj.insert("occlusion".to_string(), serde_json::json!(occlusion));
                        } else {
                            card_obj.insert("occlusion".to_string(), serde_json::Value::Null);
                        }

                        let json_tags = card
                            .tags
                            .iter()
                            .map(|tag| serde_json::Value::String(tag.clone()))
                            .collect();

                        card_obj.insert("tags".to_string(), serde_json::Value::Array(json_tags));

                        let json_examples = card
                            .examples
                            .iter()
                            .map(|example| match example {
                                crate::models::Example::Text(text) => {
                                    serde_json::Value::String(text.clone())
                                }
                                crate::models::Example::Detailed(info) => serde_json::json!(info),
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
            app.raw_schema_text = updated_json;
        }
    }
}

fn insert_opt_str(
    card_obj: &mut serde_json::Map<String, serde_json::Value>,
    key: &str,
    value: &Option<String>,
) {
    if let Some(text) = value {
        card_obj.insert(key.to_string(), serde_json::Value::String(text.clone()));
    } else {
        card_obj.insert(key.to_string(), serde_json::Value::Null);
    }
}