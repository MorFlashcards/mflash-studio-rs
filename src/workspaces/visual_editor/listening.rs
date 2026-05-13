// src/workspaces/visual_editor/listening.rs

use crate::models::Card;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, card: &mut Card, editor_mode: bool) -> bool {
    let mut changed = false;

    ui.add_space(20.0);

    if editor_mode {
        ui.heading("Listening Card");
        ui.label(
            egui::RichText::new("Listening card editor coming soon.")
                .weak()
                .italics(),
        );
        ui.add_space(10.0);

        ui.label(egui::RichText::new("Prompt").strong());
        let mut prompt_ref = card.prompt.clone().unwrap_or_default();

        if ui
            .add(egui::TextEdit::singleline(&mut prompt_ref).desired_width(f32::INFINITY))
            .changed()
        {
            card.prompt = if prompt_ref.trim().is_empty() {
                None
            } else {
                Some(prompt_ref.trim().to_string())
            };
            changed = true;
        }

        ui.add_space(10.0);

        ui.label(egui::RichText::new("Answer").strong());
        let mut answer_ref = card.answer.clone().unwrap_or_default();

        if ui
            .add(egui::TextEdit::singleline(&mut answer_ref).desired_width(f32::INFINITY))
            .changed()
        {
            card.answer = if answer_ref.trim().is_empty() {
                None
            } else {
                Some(answer_ref.trim().to_string())
            };
            changed = true;
        }
    } else {
        ui.heading("Listening Card (Reader Mode)");
        ui.label(card.prompt.as_deref().unwrap_or(""));
        ui.separator();
        ui.label(card.answer.as_deref().unwrap_or(""));
    }

    changed
}
