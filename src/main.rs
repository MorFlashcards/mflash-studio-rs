#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod archive;
mod audio;
mod config;
mod dialogs;
mod input;
mod media;
mod menubar;
pub mod models;
mod plugin;
mod plugins;
mod sfx;
mod workspaces;

use crate::plugin::MFlashPlugin;
use eframe::egui;

#[derive(PartialEq)]
pub enum SchemaFormat {
    Json,
    Toml,
    Yaml,
    Xml,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Workspace {
    Deck,
    Browse,
    VisualEditor,
    Media,
    SchemaEditor,
}

#[derive(Clone)]
pub struct AppStateSnapshot {
    pub deck: Option<models::MFlashDeck>,
    pub raw_schema_text: String,
    pub selected_index: usize,
}

pub struct MFlashStudioApp {
    pub path: String,
    pub deck: Option<models::MFlashDeck>,
    pub raw_schema_text: String,
    pub json_error: Option<String>,

    /// The UUID of the currently extracted engine workspace
    pub active_workspace_id: Option<String>,

    /// The live JSON dump from the SQLite database
    pub active_schema_json: Option<String>,

    /// Enables automatic SQLite-backed live saves while editing.
    pub enable_live_save: bool,

    /// Path to the active SQLite workspace database, when a deck is opened through mflash-core.
    pub active_db_path: Option<std::path::PathBuf>,

    pub workspace: Workspace,
    pub active_schema_format: SchemaFormat,
    pub selected_index: usize,
    pub search_query: String,
    pub lang_search_query: String,

    pub find_visible: bool,
    pub replace_visible: bool,
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
    pub fn sync_card_to_core(&self, card_index: usize) {
        if !self.enable_live_save {
            return;
        }

        let Some(deck) = &self.deck else {
            return;
        };

        if card_index >= deck.cards.len() {
            return;
        }

        let card = &deck.cards[card_index];

        let pb_card = mflash_core::pb::Card {
            id: card.id.clone(),
            term: card.term.clone(),
            definition: card.definition.clone(),
            prompt: card.prompt.clone(),
            answer: card.answer.clone(),
            ..Default::default()
        };

        let cache_path = if let Some(db_path) = &self.active_db_path {
            db_path
                .parent()
                .map(std::path::Path::to_path_buf)
                .or_else(|| {
                    self.active_workspace_id.as_ref().and_then(|workspace_id| {
                        dirs::home_dir().map(|home| home.join(".mflash_cache").join(workspace_id))
                    })
                })
        } else {
            self.active_workspace_id.as_ref().and_then(|workspace_id| {
                dirs::home_dir().map(|home| home.join(".mflash_cache").join(workspace_id))
            })
        };

        let Some(cache_path) = cache_path else {
            return;
        };

        match mflash_core::db::init_workspace_db(&cache_path) {
            Ok(conn) => {
                if let Err(e) = mflash_core::db::update_single_card(&conn, &pb_card) {
                    eprintln!("❌ Live save failed for card {}: {}", card_index, e);
                }
            }
            Err(e) => {
                eprintln!(
                    "❌ Failed to connect to live-save database at {}: {}",
                    cache_path.display(),
                    e
                );
            }
        }
    }

    pub fn refresh_schema_text(&mut self) {
        let Some(deck) = &self.deck else {
            self.raw_schema_text.clear();
            self.json_error = None;
            return;
        };

        let result = match self.active_schema_format {
            SchemaFormat::Json => serde_json::to_string_pretty(deck).map_err(|e| e.to_string()),
            SchemaFormat::Toml => toml::to_string_pretty(deck).map_err(|e| e.to_string()),
            SchemaFormat::Yaml => serde_yml::to_string(deck).map_err(|e| e.to_string()),
            SchemaFormat::Xml => quick_xml::se::to_string(deck).map_err(|e| e.to_string()),
        };

        match result {
            Ok(schema_text) => {
                self.raw_schema_text = schema_text;
                self.json_error = None;
            }
            Err(e) => {
                self.json_error = Some(format!("Schema Render Error: {}", e));
            }
        }
    }

    pub fn sync_text_to_deck(&mut self) -> bool {
        let result: Result<models::MFlashDeck, String> = match self.active_schema_format {
            SchemaFormat::Json => {
                serde_json::from_str(&self.raw_schema_text).map_err(|e| e.to_string())
            }
            SchemaFormat::Toml => toml::from_str(&self.raw_schema_text).map_err(|e| e.to_string()),
            SchemaFormat::Yaml => {
                serde_yml::from_str(&self.raw_schema_text).map_err(|e| e.to_string())
            }
            SchemaFormat::Xml => {
                quick_xml::de::from_str(&self.raw_schema_text).map_err(|e| e.to_string())
            }
        };

        match result {
            Ok(deck) => {
                self.deck = Some(deck);

                if let Some(deck) = &self.deck {
                    if deck.cards.is_empty() {
                        self.selected_index = 0;
                    } else {
                        self.selected_index = self.selected_index.min(deck.cards.len() - 1);
                    }
                }

                self.json_error = None;
                true
            }
            Err(e) => {
                self.json_error = Some(format!("Schema Parse Error: {}", e));
                false
            }
        }
    }

    pub fn open_deck(&mut self, path: String, ctx: &egui::Context) {
        if let Some((deck, json_text)) = crate::archive::load_mflash(&path) {
            self.path = path;
            self.deck = Some(deck);

            self.active_schema_format = SchemaFormat::Json;
            self.raw_schema_text = json_text;

            self.selected_index = 0;
            self.json_error = None;
            self.load_image(ctx);
        }
    }

    pub fn save_deck_as(&mut self, new_path: String) {
        self.path = new_path;
        self.save_deck();
    }

    pub fn save_deck(&mut self) {
        let Some(workspace_id) = self.active_workspace_id.clone() else {
            self.json_error = Some("Save Error: No active mflash-core workspace.".to_string());
            return;
        };

        // 0. FLUSH PENDING EDITS
        // If the user is actively typing in the Visual Editor and hits Ctrl+S,
        // force the selected card into SQLite before exporting deck.pb.
        if self.workspace == Workspace::VisualEditor {
            self.sync_card_to_core(self.selected_index);
        }

        // If the user is editing raw schema text, parse that into the deck first.
        if self.workspace == Workspace::SchemaEditor && !self.sync_text_to_deck() {
            return;
        }

        let cache_dir = if let Some(db_path) = &self.active_db_path {
            db_path
                .parent()
                .map(std::path::Path::to_path_buf)
                .or_else(|| {
                    dirs::home_dir().map(|home| home.join(".mflash_cache").join(&workspace_id))
                })
        } else {
            dirs::home_dir().map(|home| home.join(".mflash_cache").join(&workspace_id))
        };

        let Some(cache_dir) = cache_dir else {
            self.json_error = Some("Save Error: Could not find home/cache directory.".to_string());
            return;
        };

        println!("💾 Initiating universal save sequence...");

        // 1. Export DB -> Protobuf
        let conn = match mflash_core::db::init_workspace_db(&cache_dir) {
            Ok(conn) => conn,
            Err(e) => {
                self.json_error = Some(format!("Save Error: Failed to connect to DB: {}", e));
                return;
            }
        };

        let pb_deck = match mflash_core::db::export_db_to_pb(&conn) {
            Ok(pb_deck) => pb_deck,
            Err(e) => {
                self.json_error =
                    Some(format!("Save Error: Failed to compile DB to Protobuf: {}", e));
                return;
            }
        };

        // 2. Write deck.pb into the workspace
        if let Err(e) = mflash_core::schema::write_deck(&cache_dir, &pb_deck) {
            self.json_error = Some(format!("Save Error: Failed to write deck.pb: {}", e));
            return;
        }

        // 3. Choose output path
        let output_path = if self.path.is_empty() {
            let Some(save_path) = rfd::FileDialog::new()
                .add_filter("mflash decks", &["mflash"])
                .save_file()
            else {
                return;
            };

            self.path = save_path.to_string_lossy().to_string();
            save_path
        } else {
            std::path::PathBuf::from(&self.path)
        };

        // 4. Pack workspace -> .mflash
        match mflash_core::workspace::pack_deck(&workspace_id, &output_path) {
            Ok(_) => {
                println!("✅ Deck saved successfully to {}", output_path.display());
                self.json_error = None;
                self.sfx.play(crate::sfx::SoundEffect::Save);
            }
            Err(e) => {
                eprintln!("❌ Failed to pack deck: {}", e);
                self.json_error = Some(format!("Save Error: Failed to pack deck: {}", e));
            }
        }
    }

    pub fn has_valid_selected_card(&self) -> bool {
        self.deck
            .as_ref()
            .map(|deck| !deck.cards.is_empty() && self.selected_index < deck.cards.len())
            .unwrap_or(false)
    }

    pub fn switch_workspace(&mut self, target: Workspace, ctx: &egui::Context) {
        if self.workspace == target {
            return;
        }

        if self.workspace == Workspace::SchemaEditor && !self.sync_text_to_deck() {
            return;
        }

        if target == Workspace::VisualEditor {
            let Some(deck) = &self.deck else {
                return;
            };

            if deck.cards.is_empty() {
                return;
            }

            self.selected_index = self.selected_index.min(deck.cards.len() - 1);
        }

        if target == Workspace::SchemaEditor {
            self.refresh_schema_text();
        }

        self.workspace = target;

        if self.workspace == Workspace::VisualEditor {
            self.load_image(ctx);
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

        let Some(data) = &self.deck else {
            return;
        };

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
            raw_schema_text: self.raw_schema_text.clone(),
            selected_index: self.selected_index,
        });

        self.redo_stack.clear();
    }

    pub fn undo(&mut self, ctx: &egui::Context) {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(AppStateSnapshot {
                deck: self.deck.clone(),
                raw_schema_text: self.raw_schema_text.clone(),
                selected_index: self.selected_index,
            });

            self.deck = prev.deck;
            self.raw_schema_text = prev.raw_schema_text;
            self.selected_index = prev.selected_index;

            self.load_image(ctx);
        }
    }

    pub fn redo(&mut self, ctx: &egui::Context) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(AppStateSnapshot {
                deck: self.deck.clone(),
                raw_schema_text: self.raw_schema_text.clone(),
                selected_index: self.selected_index,
            });

            self.deck = next.deck;
            self.raw_schema_text = next.raw_schema_text;
            self.selected_index = next.selected_index;

            self.load_image(ctx);
        }
    }
}

fn main() -> eframe::Result<()> {
    let path = std::env::args().nth(1).unwrap_or_default();
    let config = config::load_config();

    let (deck, raw_schema_text) = if !path.is_empty() {
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
                raw_schema_text,
                json_error: None,
                active_workspace_id: None,
                active_schema_json: None,
                enable_live_save: true,
                active_db_path: None,
                workspace: Workspace::Browse,
                active_schema_format: SchemaFormat::Json,
                selected_index: 0,
                search_query: String::new(),
                lang_search_query: String::new(),

                find_visible: false,
                replace_visible: false,
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
        menubar::render(self, ctx);

        egui::TopBottomPanel::top("workspace_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;

                ui.label(egui::RichText::new("Workspaces:").weak());

                if self.config.workspaces.show_deck
                    && ui
                        .selectable_label(self.workspace == Workspace::Deck, "Deck Settings")
                        .clicked()
                {
                    self.switch_workspace(Workspace::Deck, ctx);
                }

                if self.config.workspaces.show_browse
                    && ui
                        .selectable_label(self.workspace == Workspace::Browse, "Browse")
                        .clicked()
                {
                    self.switch_workspace(Workspace::Browse, ctx);
                }

                if self.config.workspaces.show_visual_editor
                    && ui
                        .selectable_label(
                            self.workspace == Workspace::VisualEditor,
                            "Visual Editor",
                        )
                        .clicked()
                {
                    self.switch_workspace(Workspace::VisualEditor, ctx);
                }

                if self.config.workspaces.show_media
                    && ui
                        .selectable_label(self.workspace == Workspace::Media, "Assets")
                        .clicked()
                {
                    self.switch_workspace(Workspace::Media, ctx);
                }

                if self.config.workspaces.show_schema_editor
                    && ui
                        .selectable_label(
                            self.workspace == Workspace::SchemaEditor,
                            "Schema Editor",
                        )
                        .clicked()
                {
                    self.switch_workspace(Workspace::SchemaEditor, ctx);
                }
            });
        });

        self.handle_shortcuts(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.deck.is_none() && self.workspace != Workspace::SchemaEditor {
                ui.centered_and_justified(|ui| {
                    ui.label("No deck loaded.");
                });

                return;
            }

            match self.workspace {
                Workspace::Deck => workspaces::deck::render(self, ui),

                Workspace::Browse => workspaces::browse::render(self, ui, ctx),

                Workspace::VisualEditor => {
                    if self.has_valid_selected_card() {
                        workspaces::visual_editor::render(self, ui);
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label("No card selected.");
                        });
                    }
                }

                Workspace::Media => workspaces::assets::render(self, ui, ctx),

                Workspace::SchemaEditor => workspaces::schema_editor::render(self, ui),
            }
        });

        dialogs::settings::render(self, ctx);
    }
}