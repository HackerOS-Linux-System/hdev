use ratatui::style::Color;
use serde::{Deserialize, Serialize};

// ── Typy danych ───────────────────────────────────────────────────────────────

/// Wpis z pliku marketplace.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplacePlugin {
    pub name:        String,
    pub description: String,
    pub download:    String,   // URL do pliku .hk
    #[serde(default)]
    pub author:      String,
    #[serde(default)]
    pub version:     String,
    #[serde(default)]
    pub category:    String,
    #[serde(default)]
    pub tags:        Vec<String>,
}

impl MarketplacePlugin {
    pub fn is_installed(&self, installed: &[String]) -> bool {
        installed.iter().any(|i| i == &self.name)
    }

    pub fn category_color(&self) -> Color {
        match self.category.to_lowercase().as_str() {
            "language" | "lang"   => Color::Rgb(80,  200, 255),
            "theme"                => Color::Rgb(200, 100, 255),
            "formatter"            => Color::Rgb(100, 255, 100),
            "linter"               => Color::Rgb(255, 220, 50),
            "git"                  => Color::Rgb(255, 100, 50),
            "productivity"         => Color::Rgb(100, 220, 180),
            "hackeros"             => Color::Rgb(0,   255, 100),
            _                      => Color::Rgb(160, 160, 160),
        }
    }

    pub fn category_display(&self) -> String {
        if self.category.is_empty() { "plugin".to_string() } else { self.category.clone() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MarketplaceJson {
    marketplace: Vec<MarketplacePlugin>,
}

// ── Stany ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum MarketplaceTab {
    All,
    Installed,
    NotInstalled,
}

impl MarketplaceTab {
    pub fn all() -> Vec<Self> {
        vec![Self::All, Self::Installed, Self::NotInstalled]
    }
    pub fn label(&self) -> &'static str {
        match self {
            Self::All          => " Wszystkie ",
            Self::Installed    => " Zainstalowane ",
            Self::NotInstalled => " Dostępne ",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MarketplaceState {
    Browsing,
    Downloading { name: String, progress: u8 },
    Error(String),
}

pub struct Marketplace {
    pub plugins:    Vec<MarketplacePlugin>,
    pub installed:  Vec<String>,        // nazwy zainstalowanych
    pub selected:   usize,
    pub tab:        MarketplaceTab,
    pub filter:     String,
    pub state:      MarketplaceState,
    pub status_msg: String,
    pub json_url:   String,
    pub loaded:     bool,
}

impl Marketplace {
    pub fn new() -> Self {
        Self {
            plugins:    Vec::new(),
            installed:  Vec::new(),
            selected:   0,
            tab:        MarketplaceTab::All,
            filter:     String::new(),
            state:      MarketplaceState::Browsing,
            status_msg: "Ładowanie listy pluginów...".to_string(),
            json_url:   "https://raw.githubusercontent.com/HackerOS-Linux-System/hdev/main/community/marketplace.json".to_string(),
            loaded:     false,
        }
    }

    /// Załaduj marketplace.json z URL lub fallback lokalny
    pub fn load_from_url(&mut self) {
        // Próbuj curl/wget do pobrania JSON
        let json = try_fetch_json(&self.json_url);
        match json {
            Some(data) => self.parse_json(&data),
            None => {
                // Fallback — przykładowe dane gdy brak internetu
                self.plugins = fallback_plugins();
                self.status_msg = "Brak połączenia — wyświetlam przykładowe dane. Sprawdź internet.".to_string();
                self.loaded = true;
            }
        }
    }

    /// Parsuj JSON z marketplace
    pub fn parse_json(&mut self, json: &str) {
        match serde_json::from_str::<MarketplaceJson>(json) {
            Ok(data) => {
                self.plugins    = data.marketplace;
                self.status_msg = format!("Załadowano {} pluginów z marketplace.", self.plugins.len());
                self.loaded     = true;
            }
            Err(e) => {
                self.plugins    = fallback_plugins();
                self.status_msg = format!("Błąd parsowania JSON: {}. Fallback.", e);
                self.loaded     = true;
            }
        }
    }

    pub fn visible_plugins(&self) -> Vec<&MarketplacePlugin> {
        self.plugins.iter().filter(|p| {
            let tab_ok = match &self.tab {
                MarketplaceTab::All          => true,
                MarketplaceTab::Installed    => p.is_installed(&self.installed),
                                   MarketplaceTab::NotInstalled => !p.is_installed(&self.installed),
            };
            let filter_ok = if self.filter.is_empty() { true } else {
                let f = self.filter.to_lowercase();
                p.name.to_lowercase().contains(&f)
                || p.description.to_lowercase().contains(&f)
                || p.category.to_lowercase().contains(&f)
            };
            tab_ok && filter_ok
        }).collect()
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 { self.selected -= 1; }
    }

    pub fn move_down(&mut self) {
        let len = self.visible_plugins().len();
        if self.selected + 1 < len { self.selected += 1; }
    }

    pub fn next_tab(&mut self) {
        let tabs = MarketplaceTab::all();
        let idx  = tabs.iter().position(|t| t == &self.tab).unwrap_or(0);
        self.tab      = tabs[(idx + 1) % tabs.len()].clone();
        self.selected = 0;
    }

    pub fn prev_tab(&mut self) {
        let tabs = MarketplaceTab::all();
        let idx  = tabs.iter().position(|t| t == &self.tab).unwrap_or(0);
        self.tab      = tabs[(idx + tabs.len() - 1) % tabs.len()].clone();
        self.selected = 0;
    }

    /// Zainstaluj wybrany plugin (.hk pobrany z download URL)
    pub fn install_selected(&mut self) -> Result<String, String> {
        let visible: Vec<String> = self.visible_plugins().iter().map(|p| p.name.clone()).collect();
        let name = match visible.get(self.selected) {
            Some(n) => n.clone(),
            None    => return Err("Brak wybranego pluginu.".to_string()),
        };

        if self.installed.contains(&name) {
            // Odinstaluj
            self.installed.retain(|n| n != &name);
            let plugin_path = crate::config::HdevConfig::plugins_dir().join(format!("{}.hk", name));
            let _ = std::fs::remove_file(&plugin_path);
            self.status_msg = format!("Odinstalowano: {}", name);
            return Ok(format!("Odinstalowano: {}", name));
        }

        // Pobierz URL pluginu
        let url = match self.plugins.iter().find(|p| p.name == name) {
            Some(p) => p.download.clone(),
            None    => return Err("Plugin nie znaleziony.".to_string()),
        };

        // Pobierz plik .hk
        let plugin_dir = crate::config::HdevConfig::plugins_dir();
        let _ = std::fs::create_dir_all(&plugin_dir);
        let dest = plugin_dir.join(format!("{}.hk", sanitize_name(&name)));

        match download_file(&url, &dest) {
            Ok(_) => {
                self.installed.push(name.clone());
                self.status_msg = format!("OK Zainstalowano: {}", name);
                Ok(format!("Zainstalowano: {}", name))
            }
            Err(e) => {
                self.status_msg = format!("ERR Błąd pobierania: {}", e);
                Err(format!("Błąd: {}", e))
            }
        }
    }
}

// ── Pobieranie plików ─────────────────────────────────────────────────────────

fn try_fetch_json(url: &str) -> Option<String> {
    // Próbuj curl
    let curl = std::process::Command::new("curl")
    .args(["-s", "--max-time", "8", "--fail", url])
    .output();

    if let Ok(out) = curl {
        if out.status.success() {
            let body = String::from_utf8_lossy(&out.stdout).to_string();
            if !body.trim().is_empty() && body.trim().starts_with('{') {
                return Some(body);
            }
        }
    }

    // Próbuj wget
    let wget = std::process::Command::new("wget")
    .args(["-q", "-O", "-", "--timeout=8", url])
    .output();

    if let Ok(out) = wget {
        if out.status.success() {
            let body = String::from_utf8_lossy(&out.stdout).to_string();
            if !body.trim().is_empty() {
                return Some(body);
            }
        }
    }

    None
}

fn download_file(url: &str, dest: &std::path::Path) -> Result<(), String> {
    let dest_str = dest.to_string_lossy().to_string();

    // Próbuj curl
    let curl = std::process::Command::new("curl")
    .args(["-s", "--max-time", "30", "--fail", "-o", &dest_str, url])
    .status();

    if let Ok(s) = curl {
        if s.success() { return Ok(()); }
    }

    // Próbuj wget
    let wget = std::process::Command::new("wget")
    .args(["-q", "-O", &dest_str, url])
    .status();

    if let Ok(s) = wget {
        if s.success() { return Ok(()); }
    }

    Err(format!("Nie można pobrać: {}. Sprawdź curl lub wget i połączenie.", url))
}

fn sanitize_name(name: &str) -> String {
    name.chars()
    .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
    .collect()
}

/// Dane przykładowe gdy brak internetu
fn fallback_plugins() -> Vec<MarketplacePlugin> {
    vec![
        MarketplacePlugin {
            name:        "hl-enhanced".to_string(),
            description: "Rozszerzone wsparcie Hacker Lang: snippety, linter, REPL.".to_string(),
            download:    "https://raw.githubusercontent.com/HackerOS-Linux-System/hdev/main/community/plugins/hl-enhanced.hk".to_string(),
            author:      "HackerOS Team".to_string(),
            version:     "1.0.0".to_string(),
            category:    "hackeros".to_string(),
            tags:        vec!["hacker-lang".to_string()],
        },
        MarketplacePlugin {
            name:        "hsh-runner".to_string(),
            description: "Uruchamiaj skrypty hsh bezpośrednio z edytora.".to_string(),
            download:    "https://raw.githubusercontent.com/HackerOS-Linux-System/hdev/main/community/plugins/hsh-runner.hk".to_string(),
            author:      "HackerOS Team".to_string(),
            version:     "0.9.0".to_string(),
            category:    "hackeros".to_string(),
            tags:        vec!["hsh".to_string()],
        },
    ]
}
