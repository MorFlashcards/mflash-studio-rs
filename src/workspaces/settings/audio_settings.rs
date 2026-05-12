use crate::MFlashStudioApp;
use eframe::egui;

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.heading("Media Audio & Pronunciations");
    ui.add_space(8.0);

    ui.checkbox(&mut app.enable_media_audio, "Prioritize Recorded Media Audio");
    ui.label(
        egui::RichText::new("When checked, the app will play files from the deck (e.g., .mp3/.wav) before falling back to TTS.")
            .weak()
            .size(12.0),
    );

    ui.add_space(18.0);
    ui.separator();
    ui.add_space(14.0);

    ui.heading("Media Management (Coming Soon)");
    ui.label(
        egui::RichText::new("Placeholder for managing term_pronunciation, definition_pronunciation, and explanation_audio assets.")
            .weak()
            .italics(),
    );
}