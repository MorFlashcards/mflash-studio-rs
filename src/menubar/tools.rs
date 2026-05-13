use crate::MFlashStudioApp;
use eframe::egui;

pub fn render(_app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.menu_button("Tools", |ui| {
        ui.label("Tools menu coming soon");
    });
}
