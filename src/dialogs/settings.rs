use crate::MFlashStudioApp;
use eframe::egui;

pub fn render(app: &mut MFlashStudioApp, ctx: &egui::Context) {
    if !app.show_settings {
        return;
    }

    let mut is_open = app.show_settings;
    let mut should_close = false;

    egui::Window::new("Settings / Preferences")
        .id(egui::Id::new("preferences_settings_window"))
        .open(&mut is_open)
        .resizable(true)
        .default_size([760.0, 560.0])
        .min_size([640.0, 420.0])
        .show(ctx, |ui| {
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

                            // These should eventually match the `id` fields in your TOML files
                            let cats = ["Flashcards", "SFX", "TTS", "Audio", "Raw JSON"];

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

                                        ui.heading(&app.settings_category);
                                        ui.add_space(8.0);

                                        match app.settings_category.as_str() {
                                            "Flashcards" => {
                                                ui.checkbox(
                                                    &mut app.enable_live_save,
                                                    "⚡ Enable Live Save (SQLite Sync)",
                                                );

                                                ui.add_space(6.0);

                                                ui.label(
                                                    "When enabled, card edits are synced into the active SQLite workspace database.",
                                                );
                                            }

                                            _ => {
                                                // TODO: Here is where the generic TOML renderer will go!
                                                ui.label("TOML Schema parser will be wired up here.");
                                            }
                                        }
                                    });
                            },
                        );
                    },
                );

                ui.separator();

                // --- FOOTER ---
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
