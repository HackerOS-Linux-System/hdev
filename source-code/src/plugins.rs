/// Plugin system for hdev
/// .hk files are loaded from ~/.cache/HackerOS/hdev/plugins/
/// This is a placeholder implementation — full support via marketplace.

use std::path::PathBuf;
use std::fs;

#[derive(Debug, Clone)]
pub struct LoadedPlugin {
    pub id: String,
    pub name: String,
    pub version: String,
    pub path: PathBuf,
    pub active: bool,
    pub error: Option<String>,
}

pub struct PluginManager {
    pub loaded: Vec<LoadedPlugin>,
    pub plugin_dir: PathBuf,
}

impl PluginManager {
    pub fn new() -> Self {
        let dir = crate::config::HdevConfig::config_dir().join("plugins");
        let _ = fs::create_dir_all(&dir);
        let mut mgr = Self { loaded: Vec::new(), plugin_dir: dir };
        mgr.scan();
        mgr
    }

    pub fn scan(&mut self) {
        self.loaded.clear();
        if let Ok(entries) = fs::read_dir(&self.plugin_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("hk") {
                    let name = path.file_stem()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    // Parse basic metadata from hk file
                    let (version, error) = parse_hk_meta(&path);
                    self.loaded.push(LoadedPlugin {
                        id: name.clone(),
                        name: name.clone(),
                        version,
                        path,
                        active: error.is_none(),
                        error,
                    });
                }
            }
        }
    }

    pub fn plugin_count(&self) -> usize {
        self.loaded.len()
    }

    pub fn active_count(&self) -> usize {
        self.loaded.iter().filter(|p| p.active).count()
    }
}

fn parse_hk_meta(path: &PathBuf) -> (String, Option<String>) {
    match fs::read_to_string(path) {
        Ok(content) => {
            // Try to find version = "..." in hk file
            let version = content.lines()
                .find(|l| l.trim_start().starts_with("version"))
                .and_then(|l| l.split('=').nth(1))
                .map(|v| v.trim().trim_matches('"').to_string())
                .unwrap_or_else(|| "0.1.0".to_string());
            (version, None)
        }
        Err(e) => ("0.0.0".to_string(), Some(e.to_string())),
    }
}
