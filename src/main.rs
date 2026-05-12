#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod config;
mod media;
mod models;
mod plugin;
mod plugins;
mod sfx;
mod views;

use crate::plugin::MFlashPlugin;
use eframe::egui;

#[derive(PartialEq)]
pub enum ViewMode {
    List,
    Flashcard,
    RawJson,
}

#[derive(Clone)]
pub struct AppStateSnapshot {
    pub deck: Option<models::MFlashDeck>,
    pub raw_json: String,
    pub selected_index: usize,
}

pub struct MFlashStudioApp {
    pub path: String,
    pub deck: Option<models::MFlashDeck>,
    pub raw_json: String,
    pub json_error: Option<String>,
    pub mode: ViewMode,
    pub selected_index: usize,
    pub search_query: String,
    pub audio: audio::AudioEngine,
    pub sfx: sfx::SfxEngine,
    pub config: config::AppConfig,
    pub current_texture: Option<egui::TextureHandle>,
    pub plugins: Vec<Box<dyn MFlashPlugin>>,

    pub undo_stack: Vec<AppStateSnapshot>,
    pub redo_stack: Vec<AppStateSnapshot>,

    pub last_selected_text: String,
    pub last_cursor_range: Option<std::ops::Range<usize>>,

    pub editor_mode: bool,
    pub show_images: bool,
    pub show_settings: bool,
    pub settings_category: String,
}

impl MFlashStudioApp {
    // NEW: Centralized save method that can be called from any view
    pub fn save_deck(&mut self) {
        if !self.path.is_empty() {
            match models::save_mflash(&self.path, &self.path, &self.raw_json) {
                Ok(_) => {
                    self.json_error = None;
                    self.sfx.play(crate::sfx::SoundEffect::Save);
                }
                Err(e) => self.json_error = Some(format!("Save Error: {}", e)),
            }
        }
    }

    pub fn set_index(&mut self, new_index: usize, ctx: &egui::Context) {
        let (safe_index, card_clone) = {
            let Some(data) = &self.deck else { return };
            if data.cards.is_empty() {
                return;
            }
            let idx = new_index.min(data.cards.len() - 1);
            (idx, data.cards[idx].clone())
        };

        if self.selected_index != safe_index {
            self.selected_index = safe_index;
            self.load_image(ctx);

            for plugin in &mut self.plugins {
                plugin.on_card_change(&card_clone);
            }
        }
    }

    pub fn load_image(&mut self, ctx: &egui::Context) {
        self.current_texture = None;
        let Some(data) = &self.deck else { return };
        if self.selected_index >= data.cards.len() {
            return;
        }

        if let Some(media_info) = data.cards[self.selected_index].media.first() {
            let path = std::path::Path::new(&media_info.src);

            if path.is_absolute() && path.exists() {
                if let Ok(bytes) = std::fs::read(path) {
                    if let Ok(img) = image::load_from_memory(&bytes) {
                        let size = [img.width() as _, img.height() as _];
                        let image_buffer = img.to_rgba8();
                        let pixels = image_buffer.as_flat_samples();
                        let color_image =
                            egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                        self.current_texture = Some(ctx.load_texture(
                            "dropped_img",
                            color_image,
                            egui::TextureOptions::default(),
                        ));
                        return;
                    }
                }
            }

            self.current_texture = media::load_texture(ctx, &self.path, &media_info.src);
        }
    }

    pub fn push_snapshot(&mut self) {
        self.undo_stack.push(AppStateSnapshot {
            deck: self.deck.clone(),
            raw_json: self.raw_json.clone(),
            selected_index: self.selected_index,
        });
        self.redo_stack.clear();
    }

    pub fn undo(&mut self, ctx: &egui::Context) {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(AppStateSnapshot {
                deck: self.deck.clone(),
                raw_json: self.raw_json.clone(),
                selected_index: self.selected_index,
            });
            self.deck = prev.deck;
            self.raw_json = prev.raw_json;
            self.selected_index = prev.selected_index;
            self.load_image(ctx);
        }
    }

    pub fn redo(&mut self, ctx: &egui::Context) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(AppStateSnapshot {
                deck: self.deck.clone(),
                raw_json: self.raw_json.clone(),
                selected_index: self.selected_index,
            });
            self.deck = next.deck;
            self.raw_json = next.raw_json;
            self.selected_index = next.selected_index;
            self.load_image(ctx);
        }
    }
}

fn main() -> eframe::Result<()> {
    let path = std::env::args().nth(1).unwrap_or_default();
    let config = config::load_config();

    let (deck, raw_json) = if !path.is_empty() {
        models::load_mflash(&path)
            .map(|(d, j)| (Some(d), j))
            .unwrap_or((None, String::new()))
    } else {
        (None, String::new())
    };

    let plugins = plugins::get_active_plugins(&config.plugins.enabled);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config.ui.window_width, config.ui.window_height])
            .with_title("mflash-studio-rs"),
        ..Default::default()
    };

    eframe::run_native(
        "mflash-studio-rs",
        options,
        Box::new(|cc| {
            let mut app = MFlashStudioApp {
                path,
                deck,
                raw_json,
                json_error: None,
                mode: ViewMode::List,
                selected_index: 0,
                search_query: String::new(),
                audio: audio::AudioEngine::new(),
                sfx: sfx::SfxEngine::new(),
                config,
                current_texture: None,
                plugins,
                undo_stack: Vec::new(),
                redo_stack: Vec::new(),
                last_selected_text: String::new(),
                last_cursor_range: None,
                editor_mode: true,
                show_images: true,
                show_settings: false,
                settings_category: "Flashcards".to_string(),
            };
            app.load_image(&cc.egui_ctx);
            Box::new(app)
        }),
    )
}

impl eframe::App for MFlashStudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open Deck...").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("MFlash Deck", &["mflash"])
                            .pick_file()
                        {
                            let path_str = path.to_string_lossy().to_string();
                            if let Some((new_deck, new_json)) = models::load_mflash(&path_str) {
                                self.path = path_str;
                                self.deck = Some(new_deck);
                                self.raw_json = new_json;
                                self.selected_index = 0;
                                self.json_error = None;
                                self.undo_stack.clear();
                                self.redo_stack.clear();
                                self.load_image(ctx);
                            }
                        }
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Save").clicked() {
                        self.save_deck(); // Now uses centralized method
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui
                        .add_enabled(!self.undo_stack.is_empty(), egui::Button::new("⮜ Undo"))
                        .clicked()
                    {
                        self.undo(ctx);
                        ui.close_menu();
                    }
                    if ui
                        .add_enabled(!self.redo_stack.is_empty(), egui::Button::new("⮞ Redo"))
                        .clicked()
                    {
                        self.redo(ctx);
                        ui.close_menu();
                    }
                });

                ui.menu_button("View", |ui| {
                    if ui
                        .selectable_label(self.mode == ViewMode::List, "List View")
                        .clicked()
                    {
                        self.mode = ViewMode::List;
                        ui.close_menu();
                    }
                    if ui
                        .selectable_label(self.mode == ViewMode::Flashcard, "Flashcard Studio")
                        .clicked()
                    {
                        self.mode = ViewMode::Flashcard;
                        self.load_image(ctx);
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui
                        .selectable_label(self.mode == ViewMode::RawJson, "See Raw JSON")
                        .clicked()
                    {
                        self.mode = ViewMode::RawJson;
                        ui.close_menu();
                    }
                });

                // A direct button on the menu bar, no dropdown!
                if ui.button("⚙ Settings").clicked() {
                    self.show_settings = true;
                }
            });
        });

        // GLOBAL SHORTCUTS
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::Comma)) {
            self.show_settings = true;
        }
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::S)) {
            self.save_deck();
        }

        // Logic for navigation
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            let next = self.selected_index + 1;
            self.set_index(next, ctx);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            let prev = self.selected_index.saturating_sub(1);
            self.set_index(prev, ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.deck.is_none() {
                ui.centered_and_justified(|ui| ui.label("No deck loaded."));
                return;
            }
            match self.mode {
                ViewMode::List => views::list::render(self, ui, ctx),
                ViewMode::Flashcard => views::flashcard_view::render(self, ui),
                ViewMode::RawJson => views::raw_json::render(self, ui),
            }
        });

        // Render the settings window if it's toggled open
        views::settings::render(self, ctx);
    }
}
