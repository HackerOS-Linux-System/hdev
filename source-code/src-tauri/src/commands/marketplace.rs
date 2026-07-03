use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketplacePlugin {
    pub name: String,
    pub description: String,
    pub download: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct MarketplaceJson {
    marketplace: Vec<MarketplacePlugin>,
}

/// Pobiera i parsuje `marketplace.json` spod danego URL (domyslnie
/// community/marketplace.json z repo HackerOS-Linux-System/hdev).
#[tauri::command]
pub async fn marketplace_fetch(url: String) -> Result<Vec<MarketplacePlugin>, String> {
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Blad pobierania marketplace: {e}"))?;
    let text = resp.text().await.map_err(|e| e.to_string())?;
    let parsed: MarketplaceJson =
        serde_json::from_str(&text).map_err(|e| format!("Nieprawidlowy JSON marketplace: {e}"))?;
    Ok(parsed.marketplace)
}

/// Pobiera plik `.hk` pluginu i zapisuje go w
/// `~/.cache/HackerOS/hdev/plugins/<nazwa>.hk`.
#[tauri::command]
pub async fn marketplace_install(app: AppHandle, plugin: MarketplacePlugin) -> Result<String, String> {
    let resp = reqwest::get(&plugin.download)
        .await
        .map_err(|e| format!("Blad pobierania pluginu: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("Serwer zwrocil status {}", resp.status()));
    }
    let bytes = resp.bytes().await.map_err(|e| e.to_string())?;

    let plugins_dir = app.state::<crate::state::AppState>().plugins_dir.clone();
    fs::create_dir_all(&plugins_dir).map_err(|e| e.to_string())?;

    let safe_name = plugin
        .name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '-' })
        .collect::<String>();
    let dest = plugins_dir.join(format!("{safe_name}.hk"));
    fs::write(&dest, bytes).map_err(|e| e.to_string())?;

    Ok(dest.to_string_lossy().to_string())
}

#[tauri::command]
pub fn marketplace_uninstall(app: AppHandle, name: String) -> Result<(), String> {
    let plugins_dir = app.state::<crate::state::AppState>().plugins_dir.clone();
    let safe_name = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '-' })
        .collect::<String>();
    let path = plugins_dir.join(format!("{safe_name}.hk"));
    if path.exists() {
        fs::remove_file(path).map_err(|e| e.to_string())?;
    }
    Ok(())
}
