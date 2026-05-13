// src/workspaces/visual_editor/cloze.rs

use eframe::egui;

pub fn render(ui: &mut egui::Ui, editor_mode: bool) {
    ui.add_space(20.0);

    if editor_mode {
        ui.heading("Cloze Card");
        ui.label(
            egui::RichText::new("Fill-in-the-blank editor coming soon.")
                .weak()
                .italics(),
        );
    } else {
        ui.heading("Cloze Card (Reader Mode)");
    }
}
