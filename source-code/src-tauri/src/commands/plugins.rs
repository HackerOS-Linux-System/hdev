use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize)]
pub struct LoadedPlugin {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub path: String,
    pub active: bool,
    pub error: Option<String>,
    pub syntax_extensions: Vec<String>,
    pub hooks: HashMap<String, String>,
}

/// Skanuje `~/.cache/HackerOS/hdev/plugins/*.hk` i parsuje kazdy plik jako
/// prosty format INI (sekcje `[metadata]`, `[syntax]`, `[hooks]`) —
/// dokladnie taki, jaki opisuje README.md hdev.
#[tauri::command]
pub fn plugins_scan(app: AppHandle) -> Vec<LoadedPlugin> {
    let plugins_dir = app.state::<crate::state::AppState>().plugins_dir.clone();
    let mut out = Vec::new();

    let Ok(entries) = fs::read_dir(&plugins_dir) else {
        return out;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("hk") {
            continue;
        }
        let id = path
            .file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        match fs::read_to_string(&path) {
            Ok(content) => {
                let sections = parse_hk_ini(&content);
                let meta = sections.get("metadata").cloned().unwrap_or_default();
                let syntax = sections.get("syntax").cloned().unwrap_or_default();
                let hooks = sections.get("hooks").cloned().unwrap_or_default();

                let extensions = syntax
                    .get("extensions")
                    .map(|v| v.split(',').map(|s| s.trim().to_string()).collect())
                    .unwrap_or_default();

                out.push(LoadedPlugin {
                    id: id.clone(),
                    name: meta.get("name").cloned().unwrap_or(id),
                    version: meta.get("version").cloned().unwrap_or_else(|| "0.1.0".into()),
                    author: meta.get("author").cloned().unwrap_or_default(),
                    description: meta.get("description").cloned().unwrap_or_default(),
                    path: path.to_string_lossy().to_string(),
                    active: true,
                    error: None,
                    syntax_extensions: extensions,
                    hooks,
                });
            }
            Err(e) => out.push(LoadedPlugin {
                id: id.clone(),
                name: id,
                version: "0.0.0".into(),
                author: String::new(),
                description: String::new(),
                path: path.to_string_lossy().to_string(),
                active: false,
                error: Some(e.to_string()),
                syntax_extensions: Vec::new(),
                hooks: HashMap::new(),
            }),
        }
    }

    out
}

/// Bardzo lekki parser formatu `.hk` (podzbior INI: `[sekcja]` + `klucz = "wartosc"`).
fn parse_hk_ini(content: &str) -> HashMap<String, HashMap<String, String>> {
    let mut sections: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut current = String::from("root");

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            current = line[1..line.len() - 1].trim().to_lowercase();
            sections.entry(current.clone()).or_default();
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = value.trim().trim_matches('"').to_string();
            sections.entry(current.clone()).or_default().insert(key, value);
        }
    }

    sections
}
