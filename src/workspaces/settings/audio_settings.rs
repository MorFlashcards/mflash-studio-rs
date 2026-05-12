use crate::MFlashStudioApp;
use eframe::egui;

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.set_width(ui.available_width());

    render_sound_effects(app, ui);

    section_separator(ui);

    render_tts_engine(ui);

    section_separator(ui);

    render_global_parameters(ui);

    section_separator(ui);

    render_voice_profiles(ui);
}

fn section_separator(ui: &mut egui::Ui) {
    ui.add_space(18.0);
    ui.separator();
    ui.add_space(14.0);
}

fn render_sound_effects(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.heading("Sound Effects");
    ui.add_space(8.0);

    let mut is_muted = *app.sfx.is_muted.lock().unwrap();

    ui.horizontal(|ui| {
        if ui
            .checkbox(&mut is_muted, "Mute Studio UI Sounds")
            .clicked()
        {
            app.sfx.toggle_mute();
        }
    });

    ui.add_space(4.0);
    ui.label(
        egui::RichText::new("Disable clicks, hovers, saves, and other interface sounds.")
            .weak()
            .size(12.0),
    );
}

fn render_tts_engine(ui: &mut egui::Ui) {
    ui.heading("Text-to-Speech (TTS)");
    ui.add_space(4.0);

    ui.label(egui::RichText::new("Select the engine used for text-to-speech playback.").weak());

    ui.add_space(14.0);

    ui.label(egui::RichText::new("Active Engine").strong());
    ui.add_space(6.0);

    let mut current_engine = 0; // Future: app.config.audio.engine

    ui.radio_value(
        &mut current_engine,
        0,
        "System Default (eSpeak / OS Native)",
    )
    .on_hover_text("Uses voices currently installed on your operating system. 100% offline.");

    ui.add_enabled(
        false,
        egui::RadioButton::new(current_engine == 1, "Piper (Local Neural Model) 📥"),
    )
    .on_hover_text("High quality neural voices. Requires downloading model files. 100% offline.");

    ui.add_enabled(
        false,
        egui::RadioButton::new(current_engine == 2, "ElevenLabs (Cloud API) 🌐"),
    )
    .on_hover_text("Requires an active internet connection and API key.");
}

fn render_global_parameters(ui: &mut egui::Ui) {
    ui.heading("Global Parameters");
    ui.add_space(8.0);

    let mut speech_rate = 1.0; // Future: app.config.audio.speech_rate

    ui.horizontal(|ui| {
        ui.label("Speech Rate:");

        ui.add_sized(
            [280.0, 18.0],
            egui::Slider::new(&mut speech_rate, 0.5..=2.0)
                .step_by(0.1)
                .text("x"),
        );

        ui.label(format!("{speech_rate:.1}x"));

        if ui.button("↺ Reset").clicked() {
            speech_rate = 1.0;
        }
    });

    ui.add_space(4.0);

    ui.horizontal(|ui| {
        ui.add_space(86.0);
        ui.label(egui::RichText::new("0.5x").weak().size(11.0));
        ui.add_space(88.0);
        ui.label(egui::RichText::new("1.0x normal").weak().size(11.0));
        ui.add_space(62.0);
        ui.label(egui::RichText::new("2.0x").weak().size(11.0));
    });
}

fn render_voice_profiles(ui: &mut egui::Ui) {
    ui.heading("Voice Profiles");
    ui.add_space(4.0);

    ui.label(
        egui::RichText::new("Assign specific text-to-speech voices to different languages.")
            .weak()
            .size(12.0),
    );

    ui.add_space(12.0);

    let mut selected_lang = "French";

    ui.horizontal(|ui| {
        ui.label("Language:");

        egui::ComboBox::from_id_source("settings_audio_language_selector")
            .width(180.0)
            .selected_text(selected_lang)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut selected_lang, "English", "English");
                ui.selectable_value(&mut selected_lang, "French", "French");
                ui.selectable_value(&mut selected_lang, "Estonian", "Estonian");
            });
    });

    ui.add_space(14.0);

    render_voice_table(ui);
}

fn render_voice_table(ui: &mut egui::Ui) {
    let mut active_voice = "fr-ca-x-cad-local";

    let mock_voices = [
        ("fr-ca-x-cad-local", "Google, Canada", "System Default"),
        (
            "fr-ca-x-caa-network",
            "Google online, Canada",
            "System Default",
        ),
        ("fr-FR-language", "Standard French", "System Default"),
    ];

    egui::Frame::group(ui.style())
        .inner_margin(egui::Margin::same(8.0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            egui::Grid::new("settings_audio_voice_table")
                .num_columns(3)
                .spacing([18.0, 8.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label(egui::RichText::new("").weak());
                    ui.label(egui::RichText::new("Voice").weak().strong());
                    ui.label(egui::RichText::new("Engine").weak().strong());
                    ui.end_row();

                    for (voice_id, description, engine) in mock_voices {
                        if ui.radio_value(&mut active_voice, voice_id, "").clicked() {
                            // Future: play a quick TTS preview here.
                            // app.audio.speak("Test", Some("fr-CA"), true);
                        }

                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new(voice_id).strong());
                            ui.label(egui::RichText::new(description).weak().size(12.0));
                        });

                        ui.label(egui::RichText::new(engine).weak());

                        ui.end_row();
                    }
                });
        });
}
