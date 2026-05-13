use crate::{models, MFlashStudioApp, SchemaFormat, Workspace};
use eframe::egui;

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.menu_button("Edit", |ui| {
        render_history(app, ui);
        ui.separator();

        render_clipboard(app, ui);
        ui.separator();

        render_quick_actions(app, ui);
        ui.separator();

        render_find_tools(app, ui);
        ui.separator();

        render_card_menu(app, ui);
        render_text_tools_menu(app, ui);
        render_schema_editor_menu(app, ui);
        render_selection_menu(app, ui);

        ui.separator();
        render_preferences(app, ui);
    });
}

fn render_history(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    let undo_enabled = !app.undo_stack.is_empty();
    let redo_enabled = !app.redo_stack.is_empty();

    if ui
        .add_enabled(undo_enabled, egui::Button::new("Undo\tCtrl+Z"))
        .clicked()
    {
        app.undo(ui.ctx());
        ui.close_menu();
    }

    if ui
        .add_enabled(redo_enabled, egui::Button::new("Redo\tCtrl+Y"))
        .clicked()
    {
        app.redo(ui.ctx());
        ui.close_menu();
    }
}

fn render_clipboard(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    let has_selection = !app.last_selected_text.is_empty();

    if ui
        .add_enabled(has_selection, egui::Button::new("Cut\tCtrl+X"))
        .clicked()
    {
        ui.ctx().copy_text(app.last_selected_text.clone());
        app.json_error = Some(
            "Cut copied the last tracked selection. Direct text removal still belongs to the active text field."
                .to_string(),
        );
        ui.close_menu();
    }

    if ui
        .add_enabled(has_selection, egui::Button::new("Copy\tCtrl+C"))
        .clicked()
    {
        ui.ctx().copy_text(app.last_selected_text.clone());
        ui.close_menu();
    }

    if ui.button("Paste\tCtrl+V").clicked() {
        app.json_error = Some(
            "Use Ctrl+V inside the active text field. Menu paste is limited by this egui version."
                .to_string(),
        );
        ui.close_menu();
    }
}

fn render_quick_actions(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    let has_card = has_current_card(app);

    if ui
        .add_enabled(has_card, egui::Button::new("Duplicate"))
        .clicked()
    {
        duplicate_current_card(app, ui.ctx());
        ui.close_menu();
    }

    if ui
        .add_enabled(has_card, egui::Button::new("Delete"))
        .clicked()
    {
        delete_current_card(app, ui.ctx());
        ui.close_menu();
    }
}

fn render_find_tools(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    let schema_active = app.workspace == Workspace::SchemaEditor;

    if ui
        .add_enabled(schema_active, egui::Button::new("Find...\tCtrl+F"))
        .clicked()
    {
        app.find_visible = true;
        ui.close_menu();
    }

    if ui
        .add_enabled(schema_active, egui::Button::new("Replace...\tCtrl+H"))
        .clicked()
    {
        app.find_visible = true;
        app.replace_visible = true;
        ui.close_menu();
    }

    if ui
        .add_enabled(schema_active, egui::Button::new("Find Next"))
        .clicked()
    {
        find_next(app);
        ui.close_menu();
    }

    if ui
        .add_enabled(schema_active, egui::Button::new("Find Previous"))
        .clicked()
    {
        find_previous(app);
        ui.close_menu();
    }

    if !schema_active {
        ui.label(egui::RichText::new("Find tools are active in Schema Editor.").weak());
    }
}

fn render_card_menu(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    let has_deck = app.deck.is_some();
    let has_card = has_current_card(app);

    ui.menu_button("Card", |ui| {
        if ui
            .add_enabled(has_deck, egui::Button::new("New Card"))
            .clicked()
        {
            new_card_from_current_template(app, ui.ctx());
            ui.close_menu();
        }

        if ui
            .add_enabled(has_card, egui::Button::new("Duplicate Card"))
            .clicked()
        {
            duplicate_current_card(app, ui.ctx());
            ui.close_menu();
        }

        if ui
            .add_enabled(has_card, egui::Button::new("Delete Card"))
            .clicked()
        {
            delete_current_card(app, ui.ctx());
            ui.close_menu();
        }

        ui.separator();

        if ui
            .add_enabled(
                can_move_current_card_up(app),
                egui::Button::new("Move Card Up"),
            )
            .clicked()
        {
            move_current_card(app, -1, ui.ctx());
            ui.close_menu();
        }

        if ui
            .add_enabled(
                can_move_current_card_down(app),
                egui::Button::new("Move Card Down"),
            )
            .clicked()
        {
            move_current_card(app, 1, ui.ctx());
            ui.close_menu();
        }

        ui.separator();

        if ui
            .add_enabled(has_card, egui::Button::new("Reset Card Media"))
            .clicked()
        {
            reset_current_card_media(app, ui.ctx());
            ui.close_menu();
        }
    });
}

fn render_text_tools_menu(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.menu_button("Text Tools", |ui| {
        let schema_active = app.workspace == Workspace::SchemaEditor;
        let has_schema_text = !app.raw_schema_text.is_empty();
        let has_card = has_current_card(app);

        if ui
            .add_enabled(
                schema_active && has_schema_text,
                egui::Button::new("Trim Whitespace"),
            )
            .clicked()
        {
            transform_schema_text(app, trim_lines);
            ui.close_menu();
        }

        if ui
            .add_enabled(
                schema_active && has_schema_text,
                egui::Button::new("Normalize Spaces"),
            )
            .clicked()
        {
            transform_schema_text(app, normalize_spaces);
            ui.close_menu();
        }

        if ui
            .add_enabled(
                schema_active && has_schema_text,
                egui::Button::new("Sentence Case"),
            )
            .clicked()
        {
            transform_schema_text(app, sentence_case);
            ui.close_menu();
        }

        if ui
            .add_enabled(
                schema_active && has_schema_text,
                egui::Button::new("Lowercase"),
            )
            .clicked()
        {
            transform_schema_text(app, |text| text.to_lowercase());
            ui.close_menu();
        }

        if ui
            .add_enabled(
                schema_active && has_schema_text,
                egui::Button::new("Uppercase"),
            )
            .clicked()
        {
            transform_schema_text(app, |text| text.to_uppercase());
            ui.close_menu();
        }

        if ui
            .add_enabled(
                schema_active && has_schema_text,
                egui::Button::new("Clear Field"),
            )
            .clicked()
        {
            app.push_snapshot();
            app.raw_schema_text.clear();
            app.json_error = Some("Schema text cleared. Use Undo to restore it.".to_string());
            ui.close_menu();
        }

        ui.separator();

        if ui
            .add_enabled(has_card, egui::Button::new("Swap Front / Back"))
            .clicked()
        {
            swap_current_card_front_back(app);
            ui.close_menu();
        }

        if !schema_active {
            ui.label(egui::RichText::new("Text tools currently target Schema Editor text.").weak());
        }
    });
}

fn render_schema_editor_menu(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.menu_button("Schema Editor", |ui| {
        let schema_active = app.workspace == Workspace::SchemaEditor;

        if ui
            .add_enabled(schema_active, egui::Button::new("Format Document"))
            .clicked()
        {
            if app.sync_text_to_deck() {
                app.refresh_schema_text();
                app.json_error = Some("Schema formatted successfully.".to_string());
            }
            ui.close_menu();
        }

        if ui
            .add_enabled(schema_active, egui::Button::new("Validate Schema"))
            .clicked()
        {
            if app.sync_text_to_deck() {
                app.json_error = Some("Schema is valid.".to_string());
            }
            ui.close_menu();
        }

        if ui
            .add_enabled(schema_active, egui::Button::new("Sync Text to Deck"))
            .clicked()
        {
            if app.sync_text_to_deck() {
                app.json_error = Some("Schema text synced to deck.".to_string());
            }
            ui.close_menu();
        }

        if ui
            .add_enabled(schema_active, egui::Button::new("Refresh Text from Deck"))
            .clicked()
        {
            app.refresh_schema_text();
            app.json_error = Some("Schema text refreshed from deck.".to_string());
            ui.close_menu();
        }

        ui.separator();

        ui.menu_button("Convert Format", |ui| {
            render_format_choice(app, ui, SchemaFormat::Json, "JSON");
            render_format_choice(app, ui, SchemaFormat::Toml, "TOML");
            render_format_choice(app, ui, SchemaFormat::Yaml, "YAML");
            render_format_choice(app, ui, SchemaFormat::Xml, "XML");
        });

        ui.separator();

        if ui.button("Open Schema Editor").clicked() {
            app.switch_workspace(Workspace::SchemaEditor, ui.ctx());
            ui.close_menu();
        }
    });
}

fn render_selection_menu(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.menu_button("Selection", |ui| {
        let has_deck = app.deck.is_some();
        let has_card = has_current_card(app);

        if ui.button("Select All").clicked() {
            if app.workspace == Workspace::SchemaEditor {
                app.last_selected_text = app.raw_schema_text.clone();
                app.json_error =
                    Some("Schema text copied into the app selection buffer.".to_string());
            } else {
                app.json_error = Some(
                    "Select All is currently implemented for the Schema Editor buffer.".to_string(),
                );
            }
            ui.close_menu();
        }

        if ui
            .add_enabled(has_card, egui::Button::new("Select Current Card"))
            .clicked()
        {
            select_current_card_as_json(app);
            ui.close_menu();
        }

        ui.separator();

        if ui
            .add_enabled(has_deck, egui::Button::new("Select Cards With Media"))
            .clicked()
        {
            select_cards_with_media(app);
            ui.close_menu();
        }

        if ui
            .add_enabled(has_deck, egui::Button::new("Select Cards Missing Audio"))
            .clicked()
        {
            select_cards_missing_audio(app);
            ui.close_menu();
        }

        if ui
            .add_enabled(has_deck, egui::Button::new("Select Cards Missing Images"))
            .clicked()
        {
            select_cards_missing_images(app);
            ui.close_menu();
        }
    });
}

fn render_preferences(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    if ui.button("Preferences...\tCtrl+,").clicked() {
        app.show_settings = true;
        ui.close_menu();
    }
}

fn render_format_choice(
    app: &mut MFlashStudioApp,
    ui: &mut egui::Ui,
    target_format: SchemaFormat,
    label: &str,
) {
    let selected = app.active_schema_format == target_format;

    if ui.selectable_label(selected, label).clicked() {
        if app.workspace == Workspace::SchemaEditor {
            if app.sync_text_to_deck() {
                app.active_schema_format = target_format;
                app.refresh_schema_text();
                app.json_error = Some(format!("Schema converted to {}.", label));
            }
        } else {
            app.active_schema_format = target_format;
            app.switch_workspace(Workspace::SchemaEditor, ui.ctx());
            app.refresh_schema_text();
            app.json_error = Some(format!("Opened Schema Editor as {}.", label));
        }

        ui.close_menu();
    }
}

fn has_current_card(app: &MFlashStudioApp) -> bool {
    let Some(deck) = &app.deck else {
        return false;
    };

    !deck.cards.is_empty() && app.selected_index < deck.cards.len()
}

fn can_move_current_card_up(app: &MFlashStudioApp) -> bool {
    has_current_card(app) && app.selected_index > 0
}

fn can_move_current_card_down(app: &MFlashStudioApp) -> bool {
    let Some(deck) = &app.deck else {
        return false;
    };

    has_current_card(app) && app.selected_index + 1 < deck.cards.len()
}

fn new_card_from_current_template(app: &mut MFlashStudioApp, ctx: &egui::Context) {
    let insert_at = match &app.deck {
        Some(deck) if deck.cards.is_empty() => 0,
        Some(deck) => app.selected_index.min(deck.cards.len() - 1) + 1,
        None => return,
    };

    app.push_snapshot();

    let Some(deck) = &mut app.deck else {
        return;
    };

    let new_card = models::Card {
        id: format!("card-{}", deck.cards.len() + 1),
        kind: models::CardKind::Basic,
        term: Some(String::new()),
        definition: Some(String::new()),
        prompt: None,
        answer: None,
        term_lang: deck.default_term_lang.clone(),
        def_lang: deck.default_def_lang.clone(),
        notes: None,
        tags: Vec::new(),
        examples: Vec::new(),
        media: Vec::new(),
        occlusion: None,
        lexical: None,
    };

    deck.cards.insert(insert_at, new_card);
    app.selected_index = insert_at;
    app.load_image(ctx);

    app.json_error = Some("Created a new blank card.".to_string());
}

fn duplicate_current_card(app: &mut MFlashStudioApp, ctx: &egui::Context) {
    let Some(deck) = &app.deck else {
        return;
    };

    if deck.cards.is_empty() || app.selected_index >= deck.cards.len() {
        return;
    }

    let insert_at = app.selected_index + 1;
    let mut cloned = deck.cards[app.selected_index].clone();
    cloned.id = format!("card-{}", deck.cards.len() + 1);

    app.push_snapshot();

    let Some(deck) = &mut app.deck else {
        return;
    };

    deck.cards.insert(insert_at, cloned);
    app.selected_index = insert_at;
    app.load_image(ctx);

    app.json_error = Some("Duplicated current card.".to_string());
}

fn delete_current_card(app: &mut MFlashStudioApp, ctx: &egui::Context) {
    if !has_current_card(app) {
        return;
    }

    app.push_snapshot();

    let Some(deck) = &mut app.deck else {
        return;
    };

    deck.cards.remove(app.selected_index);

    if deck.cards.is_empty() {
        app.selected_index = 0;
    } else {
        app.selected_index = app.selected_index.min(deck.cards.len() - 1);
    }

    app.load_image(ctx);
    app.json_error = Some("Deleted current card.".to_string());
}

fn move_current_card(app: &mut MFlashStudioApp, direction: isize, ctx: &egui::Context) {
    let Some(deck) = &app.deck else {
        return;
    };

    if deck.cards.is_empty() || app.selected_index >= deck.cards.len() {
        return;
    }

    let current = app.selected_index as isize;
    let target = current + direction;

    if target < 0 || target >= deck.cards.len() as isize {
        return;
    }

    app.push_snapshot();

    let Some(deck) = &mut app.deck else {
        return;
    };

    deck.cards.swap(current as usize, target as usize);
    app.selected_index = target as usize;
    app.load_image(ctx);

    app.json_error = if direction < 0 {
        Some("Moved card up.".to_string())
    } else {
        Some("Moved card down.".to_string())
    };
}

fn reset_current_card_media(app: &mut MFlashStudioApp, ctx: &egui::Context) {
    if !has_current_card(app) {
        return;
    }

    app.push_snapshot();

    let Some(deck) = &mut app.deck else {
        return;
    };

    deck.cards[app.selected_index].media.clear();
    deck.cards[app.selected_index].occlusion = None;

    app.current_texture = None;
    app.load_image(ctx);
    app.json_error = Some("Reset current card media and occlusion data.".to_string());
}

fn swap_current_card_front_back(app: &mut MFlashStudioApp) {
    if !has_current_card(app) {
        return;
    }

    app.push_snapshot();

    let Some(deck) = &mut app.deck else {
        return;
    };

    let card = &mut deck.cards[app.selected_index];

    std::mem::swap(&mut card.term, &mut card.definition);
    std::mem::swap(&mut card.prompt, &mut card.answer);
    std::mem::swap(&mut card.term_lang, &mut card.def_lang);

    app.json_error = Some("Swapped current card front/back fields.".to_string());
}

fn transform_schema_text<F>(app: &mut MFlashStudioApp, transform: F)
where
    F: FnOnce(&str) -> String,
{
    app.push_snapshot();
    app.raw_schema_text = transform(&app.raw_schema_text);
    app.json_error = Some("Text tool applied to schema text.".to_string());
}

fn trim_lines(text: &str) -> String {
    text.lines().map(str::trim).collect::<Vec<_>>().join("\n")
}

fn normalize_spaces(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn sentence_case(text: &str) -> String {
    let mut chars = text.trim().chars();

    let Some(first) = chars.next() else {
        return String::new();
    };

    let mut result = first.to_uppercase().collect::<String>();
    result.push_str(&chars.as_str().to_lowercase());
    result
}

fn find_next(app: &mut MFlashStudioApp) {
    if app.find_matches.is_empty() {
        app.json_error = Some("No find matches available yet.".to_string());
        return;
    }

    app.current_match_idx = (app.current_match_idx + 1) % app.find_matches.len();
    app.json_error = Some(format!(
        "Find match {} of {}.",
        app.current_match_idx + 1,
        app.find_matches.len()
    ));
}

fn find_previous(app: &mut MFlashStudioApp) {
    if app.find_matches.is_empty() {
        app.json_error = Some("No find matches available yet.".to_string());
        return;
    }

    if app.current_match_idx == 0 {
        app.current_match_idx = app.find_matches.len() - 1;
    } else {
        app.current_match_idx -= 1;
    }

    app.json_error = Some(format!(
        "Find match {} of {}.",
        app.current_match_idx + 1,
        app.find_matches.len()
    ));
}

fn select_current_card_as_json(app: &mut MFlashStudioApp) {
    let Some(deck) = &app.deck else {
        return;
    };

    if deck.cards.is_empty() || app.selected_index >= deck.cards.len() {
        return;
    }

    match serde_json::to_string_pretty(&deck.cards[app.selected_index]) {
        Ok(card_json) => {
            app.last_selected_text = card_json;
            app.json_error = Some("Current card copied into the app selection buffer.".to_string());
        }
        Err(e) => {
            app.json_error = Some(format!("Selection Error: {}", e));
        }
    }
}

fn select_cards_with_media(app: &mut MFlashStudioApp) {
    let Some(deck) = &app.deck else {
        return;
    };

    let selected_indices = deck
        .cards
        .iter()
        .enumerate()
        .filter_map(|(index, card)| {
            if card.media.is_empty() {
                None
            } else {
                Some(index)
            }
        })
        .collect::<Vec<_>>();

    set_selected_indices(app, selected_indices, "card(s) with media");
}

fn select_cards_missing_audio(app: &mut MFlashStudioApp) {
    let Some(deck) = &app.deck else {
        return;
    };

    let selected_indices = deck
        .cards
        .iter()
        .enumerate()
        .filter_map(|(index, card)| {
            if card_has_audio(card) {
                None
            } else {
                Some(index)
            }
        })
        .collect::<Vec<_>>();

    set_selected_indices(app, selected_indices, "card(s) missing audio");
}

fn select_cards_missing_images(app: &mut MFlashStudioApp) {
    let Some(deck) = &app.deck else {
        return;
    };

    let selected_indices = deck
        .cards
        .iter()
        .enumerate()
        .filter_map(|(index, card)| {
            if card_has_image(card) {
                None
            } else {
                Some(index)
            }
        })
        .collect::<Vec<_>>();

    set_selected_indices(app, selected_indices, "card(s) missing images");
}

fn set_selected_indices(app: &mut MFlashStudioApp, selected_indices: Vec<usize>, label: &str) {
    app.last_selected_text = selected_indices
        .iter()
        .map(|index| index.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    app.json_error = Some(format!(
        "Selected {} {} into the app selection buffer.",
        selected_indices.len(),
        label
    ));
}

fn card_has_audio(card: &models::Card) -> bool {
    card.media.iter().any(|media| {
        let media_type = media.media_type.to_lowercase();
        let role = media.role.clone().unwrap_or_default().to_lowercase();

        media_type.contains("audio")
            || role.contains("audio")
            || media.src.ends_with(".mp3")
            || media.src.ends_with(".wav")
            || media.src.ends_with(".ogg")
            || media.src.ends_with(".flac")
            || media.src.ends_with(".m4a")
    })
}

fn card_has_image(card: &models::Card) -> bool {
    if card.occlusion.is_some() {
        return true;
    }

    card.media.iter().any(|media| {
        let media_type = media.media_type.to_lowercase();
        let role = media.role.clone().unwrap_or_default().to_lowercase();

        media_type.contains("image")
            || role.contains("image")
            || media.src.ends_with(".png")
            || media.src.ends_with(".jpg")
            || media.src.ends_with(".jpeg")
            || media.src.ends_with(".webp")
            || media.src.ends_with(".gif")
            || media.src.ends_with(".bmp")
            || media.src.ends_with(".svg")
    })
}
