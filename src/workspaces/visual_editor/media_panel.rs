// src/workspaces/visual_editor/media_panel.rs

use eframe::egui;

pub struct MediaPanel<'a> {
    pub show_images: bool,
    pub editor_mode: bool,
    pub current_texture: Option<&'a egui::TextureHandle>,
}

pub fn render(ui: &mut egui::Ui, panel: MediaPanel<'_>) {
    ui.add_space(20.0);

    if panel.show_images {
        if let Some(tex) = panel.current_texture {
            ui.add(egui::Image::from_texture(tex).max_width(ui.available_width()));
        } else if panel.editor_mode {
            ui.add_space(40.0);
            egui::Frame::none()
                .fill(egui::Color32::from_black_alpha(20))
                .stroke(egui::Stroke::new(1.0, egui::Color32::DARK_GRAY))
                .rounding(8.0)
                .inner_margin(egui::Margin::same(40.0))
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("📥 Drag & Drop Image Here")
                            .size(16.0)
                            .weak(),
                    );
                });
        }
    } else {
        ui.add_space(40.0);
        ui.label(egui::RichText::new("🚫 Images Hidden").weak().italics());
    }
}