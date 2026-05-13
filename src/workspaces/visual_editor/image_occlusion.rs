// src/workspaces/visual_editor/image_occlusion.rs

use crate::models::Card;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, card: &mut Card, editor_mode: bool) -> bool {
    let mut changed = false;

    ui.add_space(20.0);

    if editor_mode {
        ui.heading("Image Occlusion");
        ui.label(
            egui::RichText::new("Image occlusion visual editor coming soon.")
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

        if ui.button("Create empty occlusion object").clicked() {
            card.occlusion = Some(crate::models::Occlusion {
                image: crate::models::MediaInfo {
                    id: None,
                    media_type: "image".to_string(),
                    role: Some("occlusion_image".to_string()),
                    src: String::new(),
                    alt: None,
                    description: None,
                },
                masks: Vec::new(),
            });
            changed = true;
        }
    } else {
        ui.heading("Image Occlusion (Reader Mode)");
        ui.label(card.prompt.as_deref().unwrap_or(""));
    }

    changed
}
