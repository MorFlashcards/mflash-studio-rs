use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub ui: UiConfig,
    pub audio: AudioConfig,
    pub plugins: PluginsConfig,
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
pub struct ShortcutsConfig {
    pub next_card: String,
    pub prev_card: String,
    pub view_card: String,
    pub back_to_list: String,
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
            shortcuts: ShortcutsConfig {
                next_card: "ArrowRight".to_string(),
                prev_card: "ArrowLeft".to_string(),
                view_card: "Enter".to_string(),
                back_to_list: "Escape".to_string(),
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
