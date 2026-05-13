// src/workspaces/visual_editor/language_picker.rs

use eframe::egui;

pub struct LanguagePicker<'a> {
    pub id_source: &'static str,
    pub current_lang: &'a mut Option<String>,
    pub fallback_lang: &'a str,
    pub search_query: &'a mut String,
}

pub fn render(ui: &mut egui::Ui, picker: LanguagePicker<'_>) -> bool {
    let mut current_lang = picker.current_lang.clone().unwrap_or_default();

    let display_text = if current_lang.is_empty() {
        format!("Default: {}", picker.fallback_lang)
    } else {
        current_lang.clone()
    };

    let response = egui::ComboBox::from_id_source(picker.id_source)
        .selected_text(display_text)
        .width(150.0)
        .show_ui(ui, |ui| {
            ui.add(
                egui::TextEdit::singleline(picker.search_query)
                    .hint_text("Search..."),
            )
            .request_focus();

            ui.separator();

            let search_lower = picker.search_query.to_lowercase();
            let mut changed = false;

            for lang in crate::audio::bcp47::SUPPORTED_LANGUAGES {
                if lang.display_name.to_lowercase().contains(&search_lower)
                    || lang.bcp_47.to_lowercase().contains(&search_lower)
                {
                    if ui
                        .selectable_value(
                            &mut current_lang,
                            lang.bcp_47.to_string(),
                            format!("{} ({})", lang.display_name, lang.bcp_47),
                        )
                        .clicked()
                    {
                        changed = true;
                    }
                }
            }

            if !search_lower.is_empty()
                && ui
                    .button(format!("Use custom: '{}'", picker.search_query))
                    .clicked()
            {
                current_lang = picker.search_query.clone();
                changed = true;
            }

            changed
        });

    ui.label(
        egui::RichText::new("🗣 Language:")
            .weak()
            .size(12.0),
    );

    if let Some(true) = response.inner {
        *picker.current_lang = if current_lang.trim().is_empty() {
            None
        } else {
            Some(current_lang.trim().to_string())
        };

        picker.search_query.clear();
        return true;
    }

    false
}