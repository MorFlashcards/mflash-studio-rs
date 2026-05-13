use crate::MFlashStudioApp;
use eframe::egui;

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.menu_button("View", |ui| {
        // --- SEARCH ---
        if ui
            .add(egui::Button::new("Find...").shortcut_text("Ctrl+F"))
            .clicked()
        {
            app.find_visible = !app.find_visible;
            if !app.find_visible {
                app.find_matches.clear();
                app.current_match_idx = 0;
            }
            ui.close_menu();
        }

        ui.separator();

        // --- ZOOM CONTROLS ---
        ui.menu_button("Zoom", |ui| {
            let current_zoom = ui.ctx().zoom_factor();

            if ui
                .add(egui::Button::new("Zoom In").shortcut_text("Ctrl++"))
                .clicked()
            {
                ui.ctx().set_zoom_factor(current_zoom * 1.2);
                ui.close_menu();
            }

            if ui
                .add(egui::Button::new("Zoom Out").shortcut_text("Ctrl+-"))
                .clicked()
            {
                ui.ctx().set_zoom_factor(current_zoom / 1.2);
                ui.close_menu();
            }

            ui.separator();

            if ui
                .add(egui::Button::new("Actual Size").shortcut_text("Ctrl+0"))
                .clicked()
            {
                ui.ctx().set_zoom_factor(1.0);
                ui.close_menu();
            }
        });

        ui.separator();

        // --- FULL SCREEN ---
        // Ask the OS viewport if we are currently in fullscreen
        let is_fullscreen = ui.ctx().input(|i| i.viewport().fullscreen.unwrap_or(false));
        let fullscreen_text = if is_fullscreen {
            "Exit Full Screen"
        } else {
            "Full Screen"
        };

        if ui
            .add(egui::Button::new(fullscreen_text).shortcut_text("F11"))
            .clicked()
        {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Fullscreen(!is_fullscreen));
            ui.close_menu();
        }
    });
}
