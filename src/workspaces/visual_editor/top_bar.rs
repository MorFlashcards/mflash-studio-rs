// src/workspaces/visual_editor/top_bar.rs

use eframe::egui;

pub struct TopBar<'a> {
    pub selected_index: usize,
    pub total_cards: usize,
    pub editor_mode: bool,

    pub enable_tts: &'a mut bool,
    pub enable_media_audio: &'a mut bool,

    pub go_back: &'a mut bool,
    pub trigger_speak: &'a mut bool,
    pub action_add_card: &'a mut bool,
    pub action_save: &'a mut bool,
}

pub fn render(ui: &mut egui::Ui, top_bar: TopBar<'_>) {
    ui.horizontal(|ui| {
        if ui.button("⮜ Back").clicked() {
            *top_bar.go_back = true;
        }

        ui.label(format!(
            "Card {} of {}",
            top_bar.selected_index + 1,
            top_bar.total_cards
        ));

        if top_bar.editor_mode {
            ui.add_space(10.0);
            if ui.button("➕ Add New Card").clicked() {
                *top_bar.action_add_card = true;
            }
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("🔊 Speak").clicked() {
                *top_bar.trigger_speak = true;
            }

            ui.checkbox(top_bar.enable_tts, "🤖 TTS");
            ui.checkbox(top_bar.enable_media_audio, "🎵 File");
            ui.label(egui::RichText::new("Audio:").weak());

            if top_bar.editor_mode {
                ui.add_space(10.0);
                if ui.button("💾 Save").clicked() {
                    *top_bar.action_save = true;
                }
            }
        });
    });
}