#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod audio;
mod config;
mod views;
mod plugin;   
mod plugins;  
mod media;

use eframe::egui;
use crate::plugin::MFlashPlugin;

#[derive(PartialEq)]
pub enum ViewMode { List, Flashcard, RawJson }

#[derive(Clone)]
pub struct AppStateSnapshot {
    pub deck: Option<models::MFlashDeck>,
    pub raw_json: String,
    pub selected_index: usize,
}

// Renamed for mflash-studio-rs identity
pub struct MFlashStudioApp {
    pub path: String,
    pub deck: Option<models::MFlashDeck>,
    pub raw_json: String,
    pub json_error: Option<String>,
    pub mode: ViewMode,
    pub selected_index: usize,
    pub search_query: String,
    pub audio: audio::AudioEngine,
    pub config: config::AppConfig,
    pub current_texture: Option<egui::TextureHandle>,
    pub plugins: Vec<Box<dyn MFlashPlugin>>, 
    
    pub undo_stack: Vec<AppStateSnapshot>,
    pub redo_stack: Vec<AppStateSnapshot>,

    pub last_selected_text: String,
    pub last_cursor_range: Option<std::ops::Range<usize>>,

    // Studio Preferences
    pub editor_mode: bool,
    pub show_images: bool,
    pub show_settings: bool,
    pub settings_category: String,
}

impl MFlashStudioApp {
    pub fn set_index(&mut self, new_index: usize, ctx: &egui::Context) {
        let (safe_index, card_clone) = {
            let Some(data) = &self.deck else { return };
            if data.cards.is_empty() { return; }
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
        if self.selected_index >= data.cards.len() { return; }
        
        if let Some(media_info) = &data.cards[self.selected_index].media {
            let path = std::path::Path::new(&media_info.path);
            
            if path.is_absolute() && path.exists() {
                if let Ok(bytes) = std::fs::read(path) {
                    if let Ok(img) = image::load_from_memory(&bytes) {
                        let size = [img.width() as _, img.height() as _];
                        let image_buffer = img.to_rgba8();
                        let pixels = image_buffer.as_flat_samples();
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                        self.current_texture = Some(ctx.load_texture("dropped_img", color_image, egui::TextureOptions::default()));
                        return; 
                    }
                }
            }

            self.current_texture = media::load_texture(ctx, &self.path, &media_info.path);
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
        models::load_mflash(&path).map(|(d, j)| (Some(d), j)).unwrap_or((None, String::new()))
    } else { 
        (None, String::new()) 
    };

    let plugins = plugins::get_active_plugins(&config.plugins.enabled);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config.ui.window_width, config.ui.window_height])
            .with_title("mflash-studio-rs"), // Updated title
        ..Default::default()
    };

    eframe::run_native(
        "mflash-studio-rs",
        options,
        Box::new(|cc| {
            let mut app = MFlashStudioApp {
                path, deck, raw_json, 
                json_error: None, 
                mode: ViewMode::List, selected_index: 0,
                search_query: String::new(), audio: audio::AudioEngine::new(),
                config, current_texture: None, plugins,
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
                        if let Some(path) = rfd::FileDialog::new().add_filter("MFlash Deck", &["mflash"]).pick_file() {
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
                        if !self.path.is_empty() {
                            match models::save_mflash(&self.path, &self.path, &self.raw_json) {
                                Ok(_) => self.json_error = None,
                                Err(e) => self.json_error = Some(format!("Save Error: {}", e)),
                            }
                        }
                        ui.close_menu(); 
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.add_enabled(!self.undo_stack.is_empty(), egui::Button::new("⮜ Undo")).clicked() { 
                        self.undo(ctx);
                        ui.close_menu(); 
                    }
                    if ui.add_enabled(!self.redo_stack.is_empty(), egui::Button::new("⮞ Redo")).clicked() { 
                        self.redo(ctx);
                        ui.close_menu(); 
                    }
                });

                ui.menu_button("View", |ui| {
                    if ui.button("⚙ Studio Preferences...").clicked() {
                        self.show_settings = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.selectable_label(self.mode == ViewMode::List, "List View").clicked() { self.mode = ViewMode::List; ui.close_menu(); }
                    if ui.selectable_label(self.mode == ViewMode::Flashcard, "Flashcard Studio").clicked() { self.mode = ViewMode::Flashcard; self.load_image(ctx); ui.close_menu(); }
                    ui.separator();
                    if ui.selectable_label(self.mode == ViewMode::RawJson, "See Raw JSON").clicked() { self.mode = ViewMode::RawJson; ui.close_menu(); }
                });
            });
        });

        // Shortcut: Ctrl+,
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::Comma)) {
            self.show_settings = true;
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

        // --- PREFERENCES WINDOW FIX ---
        if self.show_settings {
            let mut is_open = self.show_settings;
            let mut should_close = false; 
            
            egui::Window::new("Studio Preferences")
                .open(&mut is_open) 
                .resizable(true)
                .default_size([650.0, 480.0])
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.set_width(140.0);
                            let cats = ["Global", "List", "Flashcards", "Plugins", "Raw JSON"];
                            for cat in cats {
                                if ui.selectable_label(self.settings_category == cat, cat).clicked() {
                                    self.settings_category = cat.to_string();
                                }
                            }
                        });
                        ui.separator();
                        ui.vertical(|ui| {
                            match self.settings_category.as_str() {
                                "Flashcards" => {
                                    let mut mock_toggle = true;
                                    ui.checkbox(&mut mock_toggle, egui::RichText::new("Enable Studio shortcuts").strong()); 
                                    ui.add_space(10.0);
                                    egui::Grid::new("pref_grid").num_columns(2).spacing([40.0, 10.0]).show(ui, |ui| {
                                        ui.label(egui::RichText::new("Action").weak());
                                        ui.label(egui::RichText::new("Shortcut Key").weak());
                                        ui.end_row();
                                        ui.label("Editor Mode");
                                        ui.checkbox(&mut self.editor_mode, "");
                                        ui.end_row();
                                        ui.label("Show Images");
                                        ui.checkbox(&mut self.show_images, "");
                                        ui.end_row();
                                    });
                                }
                                _ => { ui.label("Settings coming soon."); }
                            }
                        });
                    });
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                        if ui.button("✕ Close").clicked() { should_close = true; }
                    });
                });
            self.show_settings = is_open && !should_close;
        }
    }
}