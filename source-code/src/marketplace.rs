use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub category: PluginCategory,
    pub installed: bool,
    pub rating: f32,
    pub downloads: u64,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PluginCategory {
    Language,
    Theme,
    Formatter,
    Linter,
    Git,
    Productivity,
    HackerOS,
    Other,
}

impl PluginCategory {
    pub fn display(&self) -> &'static str {
        match self {
            PluginCategory::Language    => "Language",
            PluginCategory::Theme       => "Theme",
            PluginCategory::Formatter   => "Formatter",
            PluginCategory::Linter      => "Linter",
            PluginCategory::Git         => "Git",
            PluginCategory::Productivity=> "Productivity",
            PluginCategory::HackerOS    => "HackerOS",
            PluginCategory::Other       => "Other",
        }
    }
    pub fn color(&self) -> Color {
        match self {
            PluginCategory::Language    => Color::Rgb(80, 200, 255),
            PluginCategory::Theme       => Color::Rgb(200, 100, 255),
            PluginCategory::Formatter   => Color::Rgb(100, 255, 100),
            PluginCategory::Linter      => Color::Rgb(255, 220, 50),
            PluginCategory::Git         => Color::Rgb(255, 100, 50),
            PluginCategory::Productivity=> Color::Rgb(100, 220, 180),
            PluginCategory::HackerOS    => Color::Rgb(0, 255, 100),
            PluginCategory::Other       => Color::Gray,
        }
    }
}

pub struct Marketplace {
    pub plugins: Vec<Plugin>,
    pub selected: usize,
    pub filter: String,
    pub tab: MarketplaceTab,
    pub status_msg: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MarketplaceTab {
    All,
    Installed,
    HackerOS,
    Languages,
    Themes,
}

impl MarketplaceTab {
    pub fn all() -> Vec<Self> {
        vec![
            MarketplaceTab::All,
            MarketplaceTab::Installed,
            MarketplaceTab::HackerOS,
            MarketplaceTab::Languages,
            MarketplaceTab::Themes,
        ]
    }
    pub fn display(&self) -> &'static str {
        match self {
            MarketplaceTab::All       => " All ",
            MarketplaceTab::Installed => " Installed ",
            MarketplaceTab::HackerOS  => " HackerOS ",
            MarketplaceTab::Languages => " Languages ",
            MarketplaceTab::Themes    => " Themes ",
        }
    }
}

impl Marketplace {
    pub fn new() -> Self {
        let plugins = placeholder_plugins();
        Self {
            plugins,
            selected: 0,
            filter: String::new(),
            tab: MarketplaceTab::All,
            status_msg: "hdev Marketplace — Plugin support coming soon!".to_string(),
        }
    }

    pub fn visible_plugins(&self) -> Vec<&Plugin> {
        self.plugins.iter().filter(|p| {
            let tab_match = match &self.tab {
                MarketplaceTab::All => true,
                MarketplaceTab::Installed => p.installed,
                MarketplaceTab::HackerOS => p.category == PluginCategory::HackerOS,
                MarketplaceTab::Languages => p.category == PluginCategory::Language,
                MarketplaceTab::Themes => p.category == PluginCategory::Theme,
            };
            let filter_match = if self.filter.is_empty() {
                true
            } else {
                p.name.to_lowercase().contains(&self.filter.to_lowercase())
                || p.description.to_lowercase().contains(&self.filter.to_lowercase())
            };
            tab_match && filter_match
        }).collect()
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 { self.selected -= 1; }
    }

    pub fn move_down(&mut self) {
        let len = self.visible_plugins().len();
        if self.selected + 1 < len { self.selected += 1; }
    }

    pub fn toggle_install(&mut self) {
        let visible: Vec<String> = self.visible_plugins().iter().map(|p| p.id.clone()).collect();
        if let Some(id) = visible.get(self.selected) {
            if let Some(plugin) = self.plugins.iter_mut().find(|p| &p.id == id) {
                if plugin.installed {
                    plugin.installed = false;
                    self.status_msg = format!("⊖ Uninstalled: {} (placeholder — restart required)", plugin.name);
                } else {
                    plugin.installed = true;
                    self.status_msg = format!("⊕ Installed: {} (placeholder — restart required)", plugin.name);
                }
            }
        }
    }

    pub fn next_tab(&mut self) {
        let tabs = MarketplaceTab::all();
        let idx = tabs.iter().position(|t| t == &self.tab).unwrap_or(0);
        self.tab = tabs[(idx + 1) % tabs.len()].clone();
        self.selected = 0;
    }

    pub fn prev_tab(&mut self) {
        let tabs = MarketplaceTab::all();
        let idx = tabs.iter().position(|t| t == &self.tab).unwrap_or(0);
        self.tab = tabs[(idx + tabs.len() - 1) % tabs.len()].clone();
        self.selected = 0;
    }
}

fn placeholder_plugins() -> Vec<Plugin> {
    vec![
        Plugin {
            id: "hl-support".to_string(),
            name: "Hacker Lang Enhanced".to_string(),
            description: "Advanced Hacker Lang support: snippets, REPL integration, linter overlay.".to_string(),
            version: "1.0.0".to_string(),
            author: "HackerOS Team".to_string(),
            category: PluginCategory::HackerOS,
            installed: false,
            rating: 5.0,
            downloads: 1200,
            tags: vec!["hacker-lang".to_string(), "hl".to_string()],
        },
        Plugin {
            id: "hsh-runner".to_string(),
            name: "HSH Runner".to_string(),
            description: "Run hsh scripts directly from hdev with one keystroke.".to_string(),
            version: "0.9.0".to_string(),
            author: "HackerOS Team".to_string(),
            category: PluginCategory::HackerOS,
            installed: false,
            rating: 4.8,
            downloads: 980,
            tags: vec!["hsh".to_string(), "runner".to_string()],
        },
        Plugin {
            id: "theme-matrix".to_string(),
            name: "Matrix Theme".to_string(),
            description: "Green-on-black matrix-inspired color scheme.".to_string(),
            version: "2.1.0".to_string(),
            author: "h4ck3r_th3m3r".to_string(),
            category: PluginCategory::Theme,
            installed: false,
            rating: 4.5,
            downloads: 3400,
            tags: vec!["theme".to_string(), "dark".to_string()],
        },
        Plugin {
            id: "git-integration".to_string(),
            name: "Git Panel".to_string(),
            description: "See git status, stage/unstage files, commit — all from hdev.".to_string(),
            version: "1.3.0".to_string(),
            author: "devtools_inc".to_string(),
            category: PluginCategory::Git,
            installed: false,
            rating: 4.7,
            downloads: 5600,
            tags: vec!["git".to_string(), "vcs".to_string()],
        },
        Plugin {
            id: "rust-analyzer-tui".to_string(),
            name: "Rust Analyzer TUI".to_string(),
            description: "Rust language server integration with inline diagnostics.".to_string(),
            version: "0.4.2".to_string(),
            author: "rust_tools".to_string(),
            category: PluginCategory::Language,
            installed: false,
            rating: 4.9,
            downloads: 8900,
            tags: vec!["rust".to_string(), "lsp".to_string()],
        },
        Plugin {
            id: "hsharp-tools".to_string(),
            name: "H# Tools".to_string(),
            description: "H# language support: formatter, snippet pack, type hints.".to_string(),
            version: "0.2.0".to_string(),
            author: "HackerOS Team".to_string(),
            category: PluginCategory::HackerOS,
            installed: false,
            rating: 4.6,
            downloads: 420,
            tags: vec!["h#".to_string(), "hsharp".to_string()],
        },
        Plugin {
            id: "autopairs".to_string(),
            name: "Auto Pairs".to_string(),
            description: "Automatically close brackets, quotes, and tags.".to_string(),
            version: "1.1.0".to_string(),
            author: "editor_tools".to_string(),
            category: PluginCategory::Productivity,
            installed: false,
            rating: 4.8,
            downloads: 12000,
            tags: vec!["autopairs".to_string(), "brackets".to_string()],
        },
        Plugin {
            id: "theme-cyberpunk".to_string(),
            name: "Cyberpunk Theme".to_string(),
            description: "Neon pink and cyan cyberpunk aesthetic.".to_string(),
            version: "3.0.0".to_string(),
            author: "neon_dev".to_string(),
            category: PluginCategory::Theme,
            installed: false,
            rating: 4.3,
            downloads: 2800,
            tags: vec!["theme".to_string(), "cyberpunk".to_string()],
        },
        Plugin {
            id: "hk-editor".to_string(),
            name: "HK Plugin Editor".to_string(),
            description: "Schema validation, autocompletion, and preview for .hk plugin files.".to_string(),
            version: "0.1.0".to_string(),
            author: "HackerOS Team".to_string(),
            category: PluginCategory::HackerOS,
            installed: false,
            rating: 4.4,
            downloads: 210,
            tags: vec!["hk".to_string(), "plugins".to_string()],
        },
    ]
}
