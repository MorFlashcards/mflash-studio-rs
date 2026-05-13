// src/workspaces/visual_editor/card_kind.rs

use crate::models::Card;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, card: &mut Card) -> bool {
    let old_kind = card.kind.clone();

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Card Type:").strong());

        egui::ComboBox::from_id_source("card_kind_selector")
            .selected_text(match card.kind {
                crate::models::CardKind::Basic => "Basic (Term/Def)",
                crate::models::CardKind::ImageOcclusion => "Image Occlusion",
                crate::models::CardKind::Listening => "Listening",
                crate::models::CardKind::MediaPrompt => "Media Prompt",
                crate::models::CardKind::Cloze => "Cloze / Fill-in-the-blank",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut card.kind,
                    crate::models::CardKind::Basic,
                    "Basic (Term/Def)",
                );
                ui.selectable_value(
                    &mut card.kind,
                    crate::models::CardKind::ImageOcclusion,
                    "Image Occlusion",
                );
                ui.selectable_value(
                    &mut card.kind,
                    crate::models::CardKind::Listening,
                    "Listening",
                );
                ui.selectable_value(
                    &mut card.kind,
                    crate::models::CardKind::MediaPrompt,
                    "Media Prompt",
                );
                ui.selectable_value(
                    &mut card.kind,
                    crate::models::CardKind::Cloze,
                    "Cloze / Fill-in-the-blank",
                );
            });
    });

    card.kind != old_kind
}