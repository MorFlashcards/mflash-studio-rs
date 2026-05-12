use crate::MFlashStudioApp;
use eframe::egui;

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui) {
    ui.heading("Sound Effects");
    ui.add_space(8.0);

    let mut is_muted = *app.sfx.is_muted.lock().unwrap();

    ui.horizontal(|ui| {
        if ui.checkbox(&mut is_muted, "Mute Studio UI Sounds").clicked() {
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
