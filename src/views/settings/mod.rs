use crate::MFlashStudioApp;
use eframe::egui;

pub mod audio_settings;
pub mod flashcard_settings;

pub fn render(app: &mut MFlashStudioApp, ctx: &egui::Context) {
    if !app.show_settings {
        return;
    }

    let mut is_open = app.show_settings;
    let mut should_close = false;

    egui::Window::new("Studio Settings")
        .id(egui::Id::new("studio_settings_window_v2"))
        .open(&mut is_open)
        .resizable(true)
        .default_size([760.0, 560.0])
        .min_size([640.0, 420.0])
        .show(ctx, |ui| {
            ui.set_min_size(egui::vec2(640.0, 420.0));

            // Main vertical shell:
            // content area on top, footer at bottom.
            ui.vertical(|ui| {
                let footer_height = 42.0;
                let content_height = (ui.available_height() - footer_height).max(260.0);

                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), content_height),
                    egui::Layout::left_to_right(egui::Align::Min),
                    |ui| {
                        // --- SIDEBAR ---
                        ui.vertical(|ui| {
                            ui.set_width(140.0);

                            let cats = [
                                "Global",
                                "List",
                                "Flashcards",
                                "Audio",
                                "Plugins",
                                "Raw JSON",
                            ];

                            for cat in cats {
                                if ui
                                    .selectable_label(app.settings_category == cat, cat)
                                    .clicked()
                                {
                                    app.settings_category = cat.to_string();
                                }
                            }
                        });

                        ui.separator();

                        // --- MAIN CONTENT AREA ---
                        ui.allocate_ui_with_layout(
                            egui::vec2(ui.available_width(), content_height),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                egui::ScrollArea::vertical()
                                    .auto_shrink([false, false])
                                    .show(ui, |ui| {
                                        ui.set_width(ui.available_width());

                                        match app.settings_category.as_str() {
                                            "Flashcards" => {
                                                flashcard_settings::render(app, ui);
                                            }

                                            "Audio" => {
                                                audio_settings::render(app, ui);
                                            }

                                            "Global" => {
                                                ui.heading("Global Settings");
                                                ui.add_space(8.0);
                                                ui.label("Global studio settings will go here.");
                                            }

                                            "List" => {
                                                ui.heading("List Settings");
                                                ui.add_space(8.0);
                                                ui.label("List settings will go here.");
                                            }

                                            "Plugins" => {
                                                ui.heading("Plugin Settings");
                                                ui.add_space(8.0);
                                                ui.label("Plugin settings will go here.");
                                            }

                                            "Raw JSON" => {
                                                ui.heading("Raw JSON");
                                                ui.add_space(8.0);
                                                ui.label("Raw JSON settings/config inspection will go here.");
                                            }

                                            _ => {
                                                ui.label("Settings panel under construction.");
                                            }
                                        }
                                    });
                            },
                        );
                    },
                );

                ui.separator();

                // --- NORMAL FOOTER ---
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("✕ Close").clicked() {
                            should_close = true;
                        }
                    });
                });
            });
        });

    app.show_settings = is_open && !should_close;
}
