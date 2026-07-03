use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub is_symlink: bool,
    pub size: u64,
    pub modified: Option<i64>,
}

/// Wczytuje jeden poziom katalogu (uzywane do leniwego rozwijania drzewa plikow).
/// Katalogi sa sortowane przed plikami, oba alfabetycznie (case-insensitive).
#[tauri::command]
pub fn fs_read_dir(path: String) -> Result<Vec<FsEntry>, String> {
    let dir = fs::read_dir(&path).map_err(|e| format!("Nie mozna odczytac {path}: {e}"))?;

    let mut entries = Vec::new();
    for item in dir.flatten() {
        let meta = item.metadata().map_err(|e| e.to_string())?;
        let name = item.file_name().to_string_lossy().to_string();
        // Pomijamy ukryte pliki systemowe VCS, zachowujemy dotfiles uzytkownika widoczne
        // (spojnie z filetree.rs w wersji TUI, ktore takze je pokazuje).
        if name == ".git" || name == "node_modules" || name == "target" {
            continue;
        }
        entries.push(FsEntry {
            name,
            path: item.path().to_string_lossy().to_string(),
            is_dir: meta.is_dir(),
            is_symlink: meta.file_type().is_symlink(),
            size: meta.len(),
            modified: meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs() as i64),
        });
    }

    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(entries)
}

#[tauri::command]
pub fn fs_read_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| format!("Nie mozna odczytac {path}: {e}"))
}

#[tauri::command]
pub fn fs_write_file(path: String, contents: String) -> Result<(), String> {
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(&path, contents).map_err(|e| format!("Nie mozna zapisac {path}: {e}"))
}

#[tauri::command]
pub fn fs_create_file(path: String) -> Result<(), String> {
    if Path::new(&path).exists() {
        return Err("Plik juz istnieje".into());
    }
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(&path, "").map_err(|e| e.to_string())
}

#[tauri::command]
pub fn fs_create_dir(path: String) -> Result<(), String> {
    fs::create_dir_all(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn fs_delete(path: String) -> Result<(), String> {
    let p = Path::new(&path);
    if p.is_dir() {
        fs::remove_dir_all(p).map_err(|e| e.to_string())
    } else {
        fs::remove_file(p).map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn fs_rename(from: String, to: String) -> Result<(), String> {
    fs::rename(&from, &to).map_err(|e| format!("Nie mozna zmienic nazwy: {e}"))
}

#[tauri::command]
pub fn fs_exists(path: String) -> bool {
    Path::new(&path).exists()
}

#[tauri::command]
pub fn fs_is_dir(path: String) -> bool {
    Path::new(&path).is_dir()
}
