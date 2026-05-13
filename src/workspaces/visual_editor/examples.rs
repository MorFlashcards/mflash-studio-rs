// src/workspaces/visual_editor/examples.rs

use crate::models::{Card, Example};
use eframe::egui;

#[derive(Debug, Default)]
pub struct ExampleActions {
    pub add_example: bool,
    pub remove_example: Option<usize>,
    pub changed: bool,
}

pub fn render_edit(ui: &mut egui::Ui, card: &mut Card) -> ExampleActions {
    let mut actions = ExampleActions::default();

    ui.add_space(15.0);
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Examples:").weak());
        if ui.button("➕").on_hover_text("Add Example").clicked() {
            actions.add_example = true;
        }
    });

    for (i, example) in card.examples.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            ui.label("•");

            let text_ref = match example {
                Example::Text(s) => s,
                Example::Detailed(info) => &mut info.text,
            };

            let resp = ui.add(
                egui::TextEdit::multiline(text_ref)
                    .font(egui::TextStyle::Body)
                    .frame(false)
                    .desired_width(ui.available_width() - 30.0),
            );

            if resp.changed() {
                actions.changed = true;
            }

            if ui.button("❌").clicked() {
                actions.remove_example = Some(i);
            }
        });
    }

    actions
}

pub fn render_read(ui: &mut egui::Ui, card: &Card) {
    if card.examples.is_empty() {
        return;
    }

    ui.add_space(15.0);
    ui.label(egui::RichText::new("Examples:").weak());

    for example in &card.examples {
        let text_ref = match example {
            Example::Text(s) => s,
            Example::Detailed(info) => &info.text,
        };

        ui.label(
            egui::RichText::new(format!("• \"{}\"", text_ref))
                .italics(),
        );
    }
}