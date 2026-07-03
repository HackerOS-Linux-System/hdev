use std::path::PathBuf;
use std::sync::Mutex;

/// Globalny stan aplikacji, wspoldzielony miedzy komendami Tauri.
pub struct AppState {
    pub config_dir: PathBuf,
    pub plugins_dir: PathBuf,
    /// Numer sekwencyjny sesji terminala -> uchwyt do procesu (do zabicia / stdin).
    pub terminal_sessions: Mutex<std::collections::HashMap<String, TerminalHandle>>,
}

pub struct TerminalHandle {
    pub child: tokio::process::Child,
}

impl AppState {
    pub fn new() -> Self {
        let base = directories::ProjectDirs::from("os", "HackerOS", "hdev")
            .map(|d| d.cache_dir().to_path_buf())
            .unwrap_or_else(|| {
                // Zgodnie z oryginalnym hdev (wersja TUI): ~/.cache/HackerOS/hdev
                dirs_home().join(".cache").join("HackerOS").join("hdev")
            });

        // Utrzymujemy dokladnie te sama sciezke co wersja TUI, dla wspoldzielonej
        // konfiguracji / sesji / pluginow miedzy obiema wersjami hdev.
        let config_dir = dirs_home().join(".cache").join("HackerOS").join("hdev");
        let plugins_dir = config_dir.join("plugins");

        let _ = std::fs::create_dir_all(&config_dir);
        let _ = std::fs::create_dir_all(&plugins_dir);

        // `base` jest tu nieuzywane bezposrednio, ale zachowane gdyby w przyszlosci
        // trzeba bylo wrocic do sciezek platformowych (directories crate).
        let _ = base;

        Self {
            config_dir,
            plugins_dir,
            terminal_sessions: Mutex::new(std::collections::HashMap::new()),
        }
    }
}

fn dirs_home() -> PathBuf {
    directories::UserDirs::new()
        .map(|d| d.home_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}
