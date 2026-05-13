use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub ui: UiConfig,
    pub audio: AudioConfig,
    pub plugins: PluginsConfig,
    pub workspaces: WorkspacesConfig,
    pub shortcuts: ShortcutsConfig,
}

#[derive(Deserialize, Clone)]
pub struct UiConfig {
    pub font_size_header: f32,
    pub font_size_body: f32,
    pub window_width: f32,
    pub window_height: f32,
    pub theme: String,
}

#[derive(Deserialize, Clone)]
pub struct AudioConfig {
    pub enabled: bool,
    pub rate: f32,
}

#[derive(Deserialize, Clone)]
pub struct PluginsConfig {
    pub enabled: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct WorkspacesConfig {
    pub show_deck: bool,
    pub show_browse: bool,
    pub show_visual_editor: bool,
    pub show_media: bool,
    pub show_schema_editor: bool,
}

#[derive(Deserialize, Clone)]
pub struct ShortcutsConfig {
    pub next_card: String,
    pub prev_card: String,
    pub next_list_item: String,
    pub prev_list_item: String,
    pub view_card: String,
    pub back_to_list: String,

    pub next_workspace: String,
    pub prev_workspace: String,
    pub workspace_deck: String,
    pub workspace_browse: String,
    pub workspace_visual_editor: String,
    pub workspace_media: String,
    pub workspace_schema_editor: String,

    pub save_deck: String,
    pub open_settings: String,
    pub toggle_find: String,
    pub exit_fullscreen: String,
    pub zoom_in: String,
    pub zoom_out: String,
    pub actual_size: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ui: UiConfig {
                font_size_header: 40.0,
                font_size_body: 22.0,
                window_width: 900.0,
                window_height: 600.0,
                theme: "dark".to_string(),
            },
            audio: AudioConfig {
                enabled: true,
                rate: 1.0,
            },
            plugins: PluginsConfig { enabled: vec![] },
            workspaces: WorkspacesConfig {
                show_deck: true,
                show_browse: true,
                show_visual_editor: true,
                show_media: true,
                show_schema_editor: true,
            },
            shortcuts: ShortcutsConfig {
                next_card: "ArrowRight".to_string(),
                prev_card: "ArrowLeft".to_string(),
                next_list_item: "ArrowDown".to_string(),
                prev_list_item: "ArrowUp".to_string(),
                view_card: "Enter".to_string(),
                back_to_list: "Escape".to_string(),

                next_workspace: "Ctrl+Tab".to_string(),
                prev_workspace: "Ctrl+Shift+Tab".to_string(),
                workspace_deck: "Ctrl+1".to_string(),
                workspace_browse: "Ctrl+2".to_string(),
                workspace_visual_editor: "Ctrl+3".to_string(),
                workspace_media: "Ctrl+4".to_string(),
                workspace_schema_editor: "Ctrl+5".to_string(),

                save_deck: "Ctrl+S".to_string(),
                open_settings: "Ctrl+Comma".to_string(),
                toggle_find: "Ctrl+F".to_string(),
                exit_fullscreen: "F11".to_string(),
                zoom_in: "Ctrl+Plus".to_string(),
                zoom_out: "Ctrl+Minus".to_string(),
                actual_size: "Ctrl+0".to_string(),
            },
        }
    }
}

pub fn load_config() -> AppConfig {
    fs::read_to_string("config.toml")
        .ok()
        .and_then(|c| toml::from_str(&c).ok())
        .unwrap_or_default()
}
