use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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

pub const EDITOR_LANGUAGES: &[&str] = &[
    "auto",
"Hacker Lang",
"Hacker Lang++",
"H#",
"Rust",
"Python",
"Go",
"C",
"C++",
"JavaScript",
"TypeScript",
"Shell",
"Lua",
"Java",
"Kotlin",
"Dart",
"Nim",
"Crystal",
"Odin",
"Vala",
"HTML",
"CSS",
"JSON",
"YAML",
"TOML",
"HCL",
"XML",
"HK Plugin",
"Plain Text",
];

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
    /// Nadpisanie języka dla bieżącej sesji ("auto" = automatyczne wykrywanie)
    pub default_language_override: String,
    pub autocomplete_enabled: bool,
}

impl Default for HdevConfig {
    fn default() -> Self {
        Self {
            theme: "hacker-dark".to_string(),
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
            .to_string(),
            terminal_shell: "sh".to_string(),
            default_language_override: "auto".to_string(),
                autocomplete_enabled: true,
        }
    }
}

impl HdevConfig {
    pub fn config_dir() -> PathBuf {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/root"));
        home.join(".cache").join("HackerOS").join("hdev")
    }

    pub fn config_path() -> PathBuf {
        Self::config_dir().join("config.json")
    }

    pub fn plugins_dir() -> PathBuf {
        Self::config_dir().join("plugins")
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path();
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let config: HdevConfig = serde_json::from_str(&content).unwrap_or_default();
            Ok(config)
        } else {
            let config = HdevConfig::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let dir = Self::config_dir();
        fs::create_dir_all(&dir)?;
        fs::create_dir_all(Self::plugins_dir())?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(Self::config_path(), content)?;
        Ok(())
    }

    pub fn add_recent_file(&mut self, path: &str) {
        self.recent_files.retain(|p| p != path);
        self.recent_files.insert(0, path.to_string());
        if self.recent_files.len() > 20 {
            self.recent_files.truncate(20);
        }
    }

    /// Zwraca indeks bieżącego motywu w tablicy THEMES
    pub fn theme_index(&self) -> usize {
        THEMES.iter().position(|&t| t == self.theme.as_str()).unwrap_or(0)
    }

    /// Przełącz na następny motyw
    pub fn next_theme(&mut self) {
        let idx = (self.theme_index() + 1) % THEMES.len();
        self.theme = THEMES[idx].to_string();
    }

    /// Przełącz na poprzedni motyw
    pub fn prev_theme(&mut self) {
        let idx = self.theme_index();
        let new = if idx == 0 { THEMES.len() - 1 } else { idx - 1 };
        self.theme = THEMES[new].to_string();
    }

    pub fn language_index(&self) -> usize {
        EDITOR_LANGUAGES
        .iter()
        .position(|&l| l == self.default_language_override.as_str())
        .unwrap_or(0)
    }

    pub fn next_language(&mut self) {
        let idx = (self.language_index() + 1) % EDITOR_LANGUAGES.len();
        self.default_language_override = EDITOR_LANGUAGES[idx].to_string();
    }

    pub fn prev_language(&mut self) {
        let idx = self.language_index();
        let new = if idx == 0 { EDITOR_LANGUAGES.len() - 1 } else { idx - 1 };
        self.default_language_override = EDITOR_LANGUAGES[new].to_string();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub open_files: Vec<String>,
    pub active_file: Option<String>,
    pub panel_state: String,
    pub terminal_history: Vec<String>,
}

impl Default for SessionData {
    fn default() -> Self {
        Self {
            open_files: Vec::new(),
            active_file: None,
            panel_state: "editor".to_string(),
            terminal_history: Vec::new(),
        }
    }
}

impl SessionData {
    pub fn session_path() -> PathBuf {
        HdevConfig::config_dir().join("session.json")
    }

    pub fn load() -> Result<Self> {
        let path = Self::session_path();
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            Ok(serde_json::from_str(&content).unwrap_or_default())
        } else {
            Ok(SessionData::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let dir = HdevConfig::config_dir();
        fs::create_dir_all(&dir)?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(Self::session_path(), content)?;
        Ok(())
    }
}

/// Marketplace plugin entry (z community/marketplace.json)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketplaceEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub category: String,
    pub tags: Vec<String>,
    pub rating: f32,
    pub downloads: u64,
    pub hk_url: Option<String>,   // URL do pobrania pliku .hk
}
