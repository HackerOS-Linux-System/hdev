use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub const THEMES: &[&str] = &[
    "hacker-dark",
    "hacker-green",
    "cyberpunk",
    "matrix",
    "nord",
    "solarized-dark",
    "dracula",
    "monokai",
    "gruvbox",
    "one-dark",
];

/// Ta sama struktura co `HdevConfig` w wersji TUI (src/config.rs) — celowo,
/// zeby obie wersje hdev mogly wspoldzielic ~/.cache/HackerOS/hdev/config.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HdevConfig {
    pub theme: String,
    pub font_size: u8,
    pub tab_size: u8,
    pub auto_save: bool,
    pub show_line_numbers: bool,
    pub show_file_tree: bool,
    pub word_wrap: bool,
    pub last_opened_path: Option<String>,
    pub recent_files: Vec<String>,
    pub installed_plugins: Vec<String>,
    pub marketplace_url: String,
    pub terminal_shell: String,
    pub default_language_override: String,
    pub autocomplete_enabled: bool,
    // Pola dodane w wersji GUI — opcjonalne, zeby config.json z wersji TUI
    // nadal wczytywal sie bez bledu (serde `default`).
    #[serde(default = "default_true")]
    pub minimap_enabled: bool,
    #[serde(default)]
    pub open_tabs_on_startup: bool,
    #[serde(default = "default_terminal_font_size")]
    pub terminal_font_size: u8,
}

fn default_true() -> bool {
    true
}
fn default_terminal_font_size() -> u8 {
    13
}

impl Default for HdevConfig {
    fn default() -> Self {
        Self {
            theme: "hacker-dark".into(),
            font_size: 14,
            tab_size: 4,
            auto_save: true,
            show_line_numbers: true,
            show_file_tree: true,
            word_wrap: false,
            last_opened_path: None,
            recent_files: Vec::new(),
            installed_plugins: Vec::new(),
            marketplace_url:
                "https://raw.githubusercontent.com/HackerOS-Linux-System/hdev/main/community/marketplace.json"
                    .into(),
            terminal_shell: "sh".into(),
            default_language_override: "auto".into(),
            autocomplete_enabled: true,
            minimap_enabled: true,
            open_tabs_on_startup: true,
            terminal_font_size: 13,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionData {
    pub open_files: Vec<String>,
    pub active_file: Option<String>,
    pub panel_state: String,
    pub terminal_history: Vec<String>,
}

fn config_dir(app: &AppHandle) -> PathBuf {
    app.state::<crate::state::AppState>().config_dir.clone()
}

fn config_path(app: &AppHandle) -> PathBuf {
    config_dir(app).join("config.json")
}

fn session_path(app: &AppHandle) -> PathBuf {
    config_dir(app).join("session.json")
}

#[tauri::command]
pub fn config_load(app: AppHandle) -> Result<HdevConfig, String> {
    let path = config_path(&app);
    if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        Ok(serde_json::from_str(&content).unwrap_or_default())
    } else {
        let cfg = HdevConfig::default();
        config_save(app, cfg.clone())?;
        Ok(cfg)
    }
}

#[tauri::command]
pub fn config_save(app: AppHandle, config: HdevConfig) -> Result<(), String> {
    let dir = config_dir(&app);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let content = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(config_path(&app), content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn session_load(app: AppHandle) -> Result<SessionData, String> {
    let path = session_path(&app);
    if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        Ok(serde_json::from_str(&content).unwrap_or_default())
    } else {
        Ok(SessionData::default())
    }
}

#[tauri::command]
pub fn session_save(app: AppHandle, session: SessionData) -> Result<(), String> {
    let dir = config_dir(&app);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let content = serde_json::to_string_pretty(&session).map_err(|e| e.to_string())?;
    fs::write(session_path(&app), content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn config_themes() -> Vec<&'static str> {
    THEMES.to_vec()
}
