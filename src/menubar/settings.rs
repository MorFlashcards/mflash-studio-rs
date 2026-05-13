use crate::MFlashStudioApp;
use eframe::egui;

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    if ui.button("Settings").clicked() {
        app.show_settings = true;
    }
}
