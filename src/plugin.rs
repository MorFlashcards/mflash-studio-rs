use eframe::egui;
use crate::models::Card;

pub trait MFlashPlugin {
    fn name(&self) -> &str;
    fn render_ui(&mut self, _ui: &mut egui::Ui, _card: Option<&Card>) {}
    fn on_card_change(&mut self, _card: &Card) {}
}
