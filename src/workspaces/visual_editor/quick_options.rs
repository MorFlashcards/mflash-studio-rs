// src/workspaces/visual_editor/quick_options.rs

use eframe::egui;

pub struct QuickOptions<'a> {
    pub show_lang_codes: &'a mut bool,
    pub show_phonetic: &'a mut bool,
    pub show_part_of_speech: &'a mut bool,
    pub show_notes: &'a mut bool,
    pub show_tags: &'a mut bool,
}

pub fn render(ui: &mut egui::Ui, options: QuickOptions<'_>) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 16.0;
        ui.label(egui::RichText::new("👁 View:").strong());

        ui.checkbox(options.show_lang_codes, "Language Codes");
        ui.checkbox(options.show_phonetic, "Phonetic");
        ui.checkbox(options.show_part_of_speech, "Part of Speech");
        ui.checkbox(options.show_notes, "Notes");
        ui.checkbox(options.show_tags, "Tags");
    });
}