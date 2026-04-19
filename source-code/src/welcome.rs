use crate::config::HdevConfig;

#[derive(Debug, Clone, PartialEq)]
pub enum WelcomeAction {
    OpenEditor,
    OpenTerminal,
    OpenMarketplace,
    OpenSettings,
    OpenRecentFile(String),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WelcomeItem {
    NewFile,
    OpenFolder,
    Terminal,
    Marketplace,
    Settings,
    RecentFile(String),
}

impl WelcomeItem {
    pub fn label(&self) -> String {
        match self {
            WelcomeItem::NewFile      => " New File          Ctrl+T".to_string(),
            WelcomeItem::OpenFolder   => " Open Folder       Ctrl+O".to_string(),
            WelcomeItem::Terminal     => " Terminal          Ctrl+B".to_string(),
            WelcomeItem::Marketplace  => " Marketplace       Ctrl+M".to_string(),
            WelcomeItem::Settings     => " Settings          Ctrl+,".to_string(),
            WelcomeItem::RecentFile(p) => {
                let name = std::path::Path::new(p)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(p);
                format!(" {}", name)
            }
        }
    }
    pub fn icon(&self) -> &'static str {
        match self {
            WelcomeItem::NewFile      => "◈",
            WelcomeItem::OpenFolder   => "⊞",
            WelcomeItem::Terminal     => "❯",
            WelcomeItem::Marketplace  => "◎",
            WelcomeItem::Settings     => "⚙",
            WelcomeItem::RecentFile(_)=> "◷",
        }
    }
}

pub struct WelcomeScreen {
    pub items: Vec<WelcomeItem>,
    pub selected: usize,
}

impl WelcomeScreen {
    pub fn new(config: &HdevConfig) -> Self {
        let mut items = vec![
            WelcomeItem::NewFile,
            WelcomeItem::OpenFolder,
            WelcomeItem::Terminal,
            WelcomeItem::Marketplace,
            WelcomeItem::Settings,
        ];
        for recent in config.recent_files.iter().take(5) {
            items.push(WelcomeItem::RecentFile(recent.clone()));
        }
        Self { items, selected: 0 }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 { self.selected -= 1; }
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.items.len() { self.selected += 1; }
    }

    pub fn select(&self) -> WelcomeAction {
        match &self.items[self.selected] {
            WelcomeItem::NewFile      => WelcomeAction::OpenEditor,
            WelcomeItem::OpenFolder   => WelcomeAction::OpenEditor,
            WelcomeItem::Terminal     => WelcomeAction::OpenTerminal,
            WelcomeItem::Marketplace  => WelcomeAction::OpenMarketplace,
            WelcomeItem::Settings     => WelcomeAction::OpenSettings,
            WelcomeItem::RecentFile(p)=> WelcomeAction::OpenRecentFile(p.clone()),
        }
    }
}
