use crate::MFlashStudioApp;
use eframe::egui;

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.heading("Text-to-Speech (TTS)");
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new("Configure text-to-speech engine and global parameters.").weak(),
    );

    ui.add_space(14.0);
    render_tts_engine(app, ui);

    ui.add_space(18.0);
    ui.separator();
    ui.add_space(14.0);

    render_global_parameters(app, ui);

    ui.add_space(18.0);
    ui.separator();
    ui.add_space(14.0);

    render_voice_profiles(app, ui);
}

fn render_tts_engine(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.heading("TTS Engine");

    ui.checkbox(&mut app.enable_tts, "Enable Text-to-Speech");

    ui.label(
        egui::RichText::new(
            "When enabled, the app can read terms, definitions, and prompts aloud.",
        )
        .weak()
        .size(12.0),
    );

    ui.add_space(8.0);

    ui.add_enabled_ui(false, |ui| {
        ui.horizontal(|ui| {
            ui.label("Engine:");
            egui::ComboBox::from_id_source("tts_engine_selector")
                .selected_text("System Default")
                .show_ui(ui, |ui| {
                    ui.selectable_label(true, "System Default");
                    ui.selectable_label(false, "Piper");
                    ui.selectable_label(false, "eSpeak NG");
                    ui.selectable_label(false, "External Command");
                });
        });

        ui.horizontal(|ui| {
            ui.label("Fallback behavior:");
            egui::ComboBox::from_id_source("tts_fallback_selector")
                .selected_text("Use TTS when no media audio exists")
                .show_ui(ui, |ui| {
                    ui.selectable_label(true, "Use TTS when no media audio exists");
                    ui.selectable_label(false, "Always prefer TTS");
                    ui.selectable_label(false, "Never fallback to TTS");
                });
        });
    });

    ui.label(
        egui::RichText::new("Engine selection is a placeholder for now.")
            .weak()
            .italics()
            .size(12.0),
    );
}

fn render_global_parameters(_app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.heading("Global Parameters");

    ui.add_enabled_ui(false, |ui| {
        ui.add(
            egui::Slider::new(&mut 1.0_f32, 0.5..=2.0)
                .text("Rate")
                .clamp_to_range(true),
        );

        ui.add(
            egui::Slider::new(&mut 1.0_f32, 0.5..=2.0)
                .text("Pitch")
                .clamp_to_range(true),
        );

        ui.add(
            egui::Slider::new(&mut 1.0_f32, 0.0..=2.0)
                .text("Volume")
                .clamp_to_range(true),
        );
    });

    ui.label(
        egui::RichText::new("Rate, pitch, and volume controls are placeholders for now.")
            .weak()
            .italics()
            .size(12.0),
    );
}

fn render_voice_profiles(_app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.heading("Voice Profiles");

    ui.label(
        egui::RichText::new(
            "Future voice profiles can map languages or card fields to specific voices.",
        )
        .weak(),
    );

    ui.add_space(8.0);

    egui::Grid::new("tts_voice_profiles_grid")
        .striped(true)
        .num_columns(3)
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Profile").strong());
            ui.label(egui::RichText::new("Language").strong());
            ui.label(egui::RichText::new("Voice").strong());
            ui.end_row();

            ui.label("Term");
            ui.label("Deck default term language");
            ui.label("System default");
            ui.end_row();

            ui.label("Definition");
            ui.label("Deck default definition language");
            ui.label("System default");
            ui.end_row();

            ui.label("Prompt");
            ui.label("Auto");
            ui.label("System default");
            ui.end_row();
        });

    ui.add_space(8.0);

    ui.add_enabled_ui(false, |ui| {
        ui.button("+ Add Voice Profile");
    });

    ui.label(
        egui::RichText::new("Voice profile editing is not wired yet.")
            .weak()
            .italics()
            .size(12.0),
    );
}