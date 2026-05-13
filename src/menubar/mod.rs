use eframe::egui;

pub mod edit;
pub mod file;
pub mod help;
pub mod settings;
pub mod tools;
pub mod view;

use crate::MFlashStudioApp;

pub fn render(app: &mut MFlashStudioApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            file::render(app, ui);
            edit::render(app, ui);
            view::render(app, ui);
            tools::render(app, ui);
            settings::render(app, ui);
            help::render(app, ui);
        });
    });
}
