use crate::MFlashStudioApp;
use eframe::egui;

pub fn render(_app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.heading("Raw JSON Workspace Settings");
    ui.add_space(8.0);

    // --- SECTION: Keyboard Shortcuts ---
    ui.collapsing("⌨ Keyboard Shortcuts", |ui| {
        ui.label("Workspace-specific shortcuts for the JSON editor:");
        egui::Grid::new("json_shortcuts_grid")
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Ctrl + F").strong());
                ui.label("Toggle Find & Replace Widget");
                ui.end_row();

                ui.label(egui::RichText::new("Ctrl + S").strong());
                ui.label("Apply & Save changes to disk");
                ui.end_row();

                ui.label(egui::RichText::new("Ctrl + .").strong());
                ui.label("Toggle Regex mode (inside Find widget)");
                ui.end_row();
            });
    });

    ui.add_space(12.0);

    // --- SECTION: Editor Appearance ---
    ui.collapsing("🎨 Editor Appearance", |ui| {
        ui.label("Customizing the look and feel of the JSON editor.");
        ui.add_enabled_ui(false, |ui| {
            ui.checkbox(&mut false, "Enable Syntax Highlighting (Placeholder)");
            ui.checkbox(&mut false, "Show Line Numbers (Currently always on)");
            ui.horizontal(|ui| {
                ui.label("Editor Theme:");
                ui.label(egui::RichText::new("Default Dark").italics());
            });
        });
        ui.label(egui::RichText::new("Theming and syntax coloring settings coming soon.").weak().italics());
    });

    ui.add_space(12.0);

    // --- SECTION: MFlash Debugger & Advanced ---
    ui.collapsing("🛠 Debugger & Advanced", |ui| {
        ui.label("Advanced tools for mflash deck validation and execution.");
        
        ui.add_enabled_ui(false, |ui| {
            ui.button("Open Command Palette (Ctrl+Shift+P)");
            ui.checkbox(&mut false, "Enable Breakpoints & Step-through Debugging");
        });

        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("Breakpoints will highlight the current line during study execution simulation.")
                .weak()
                .italics(),
        );
    });

    ui.add_space(20.0);
    ui.separator();
}
