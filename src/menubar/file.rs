use crate::MFlashStudioApp;
use eframe::egui;

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.menu_button("File", |ui| {
        if ui.button("Open Deck...").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("mflash files", &["mflash"])
                .pick_file()
            {
                app.open_deck(path.display().to_string(), ui.ctx());
            }
            ui.close_menu();
        }

        ui.separator();

        if ui.button("Save").clicked() {
            app.save_deck();
            ui.close_menu();
        }

        if ui.button("Save As...").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("mflash files", &["mflash"])
                .set_file_name("untitled.mflash")
                .save_file()
            {
                app.save_deck_as(path.display().to_string());
            }
            ui.close_menu();
        }

        ui.separator();

        if ui.button("Export to JSON...").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("JSON files", &["json"])
                .set_file_name("export.json")
                .save_file()
            {
                let _ = std::fs::write(path, &app.raw_schema_text);
            }
            ui.close_menu();
        }

        ui.separator();

        if ui.button("Close Deck").clicked() {
            app.deck = None;
            app.path = String::new();
            app.raw_schema_text = String::new();
            app.current_texture = None;
            ui.close_menu();
        }

        ui.separator();

        if ui.button("Quit").clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
    });
}
