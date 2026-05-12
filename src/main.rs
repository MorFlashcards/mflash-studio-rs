#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod config;
mod media;
mod plugin;
mod plugins;
mod sfx;
mod workspaces;
pub mod archive;
pub mod models;

use crate::plugin::MFlashPlugin;
use eframe::egui;

#[derive(PartialEq)]
pub enum Workspace {
    Deck,
    List,
    Flashcard,
    Media,
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
    pub workspace: Workspace,
    pub selected_index: usize,
    pub search_query: String,
    pub lang_search_query: String,

    pub find_visible: bool,
    pub find_query: String,
    pub replace_query: String,
    pub find_use_regex: bool,
    pub find_case_sensitive: bool,
    pub find_matches: Vec<std::ops::Range<usize>>,
    pub current_match_idx: usize,

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

    pub show_lang_codes: bool,
    pub show_phonetic: bool,
    pub show_part_of_speech: bool,
    pub show_notes: bool,
    pub show_tags: bool,

    pub enable_tts: bool,
    pub enable_media_audio: bool,
}

impl MFlashStudioApp {
    pub fn save_deck(&mut self) {
        if !self.path.is_empty() {
            match crate::archive::save_mflash(&self.path, &self.path, &self.raw_json) {
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
        crate::archive::load_mflash(&path)
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
                workspace: Workspace::List,
                selected_index: 0,
                search_query: String::new(),
                lang_search_query: String::new(),

                find_visible: false,
                find_query: String::new(),
                replace_query: String::new(),
                find_use_regex: false,
                find_case_sensitive: false,
                find_matches: Vec::new(),
                current_match_idx: 0,

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
                show_lang_codes: false,
                show_phonetic: false,
                show_part_of_speech: false,
                show_notes: false,
                show_tags: false,
                enable_tts: true,
                enable_media_audio: true,
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
                // FILE MENU
                ui.menu_button("File", |ui| {
                    if ui.button("New Deck...").clicked() {
                        // TODO: Implement New
                        ui.close_menu();
                    }
                    if ui.button("Open Deck...").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("MFlash Deck", &["mflash"])
                            .pick_file()
                        {
                            let path_str = path.to_string_lossy().to_string();
                            if let Some((new_deck, new_json)) =
                                crate::archive::load_mflash(&path_str)
                            {
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
                        self.save_deck();
                        ui.close_menu();
                    }
                    if ui.button("Close Deck").clicked() {
                        // TODO: Implement Close
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                // EDIT MENU
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
                    ui.separator();
                    if ui.button("Cut").clicked() {
                        // TODO: Global Cut
                        ui.close_menu();
                    }
                    if ui.button("Copy").clicked() {
                        // TODO: Global Copy
                        ui.close_menu();
                    }
                    if ui.button("Paste").clicked() {
                        // TODO: Global Paste
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Preferences / Settings").clicked() {
                        self.show_settings = true;
                        ui.close_menu();
                    }
                });

                // VIEW MENU
                ui.menu_button("View", |ui| {
                    if ui.button("Zoom In").clicked() {
                        // TODO
                        ui.close_menu();
                    }
                    if ui.button("Zoom Out").clicked() {
                        // TODO
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Fullscreen").clicked() {
                        // TODO
                        ui.close_menu();
                    }
                    if ui.button("Toggle UI Elements").clicked() {
                        // TODO
                        ui.close_menu();
                    }
                });

                // TOOLS MENU
                ui.menu_button("Tools", |ui| {
                    if ui.button("Plugin Manager").clicked() {
                        // TODO
                        ui.close_menu();
                    }
                    if ui.button("Deck Exporter").clicked() {
                        // TODO
                        ui.close_menu();
                    }
                    if ui.button("Media Manager").clicked() {
                        // TODO
                        ui.close_menu();
                    }
                });

                // HELP MENU
                ui.menu_button("Help", |ui| {
                    if ui.button("Documentation").clicked() {
                        // TODO
                        ui.close_menu();
                    }
                    if ui.button("Discord Server").clicked() {
                        // TODO
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("About").clicked() {
                        // TODO
                        ui.close_menu();
                    }
                });

                // TOP LEVEL SETTINGS (Doubling up)
                if ui.button("Settings").clicked() {
                    self.show_settings = true;
                }
            });
        });

        egui::TopBottomPanel::top("workspace_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;

                ui.label(egui::RichText::new("Workspaces:").weak());

                ui.selectable_value(&mut self.workspace, Workspace::Deck, "Deck Settings");
                ui.selectable_value(&mut self.workspace, Workspace::List, "List View");

                // Keep the image loading trigger when switching to Flashcard view
                if ui
                    .selectable_value(
                        &mut self.workspace,
                        Workspace::Flashcard,
                        "Flashcard Studio",
                    )
                    .clicked()
                {
                    self.load_image(ctx);
                }

                ui.selectable_value(&mut self.workspace, Workspace::Media, "Media Assets");
                ui.selectable_value(&mut self.workspace, Workspace::RawJson, "Raw JSON");
            });
        });

        // GLOBAL SHORTCUTS
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::Comma)) {
            self.show_settings = true;
        }
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::S)) {
            self.save_deck();
        }
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::F)) {
            if self.workspace == Workspace::RawJson {
                self.find_visible = !self.find_visible;

                if !self.find_visible {
                    self.find_matches.clear();
                    self.current_match_idx = 0;
                }
            }
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
            // Allow the Raw JSON workspace to show even if no deck is loaded
            if self.deck.is_none() && self.workspace != Workspace::RawJson {
                ui.centered_and_justified(|ui| ui.label("No deck loaded."));
                return;
            }

            match self.workspace {
                Workspace::Deck => workspaces::deck::render(self, ui),
                Workspace::List => workspaces::list::render(self, ui, ctx),
                Workspace::Flashcard => workspaces::flashcards::render(self, ui),
                Workspace::Media => {
                    // Step 18 Media Placeholder
                    ui.heading("Media Assets");
                    ui.label("Media library and asset management will go here.");
                }
                Workspace::RawJson => workspaces::raw_json::render(self, ui),
            }
        });

        workspaces::settings::render(self, ctx);
    }
}