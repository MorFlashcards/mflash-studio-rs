use crate::MFlashStudioApp;
use eframe::egui;

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.heading("Flashcard Editor Preferences");
    ui.add_space(10.0);

    egui::Grid::new("flashcard_pref_grid")
        .num_columns(2)
        .spacing([40.0, 10.0])
        .show(ui, |ui| {
            ui.label("Editor Mode");
            ui.checkbox(&mut app.editor_mode, "");
            ui.end_row();

            ui.label("Show Images");
            ui.checkbox(&mut app.show_images, "");
            ui.end_row();
        });
}
