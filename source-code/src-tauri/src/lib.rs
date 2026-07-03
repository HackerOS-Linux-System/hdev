mod commands;
mod state;

use commands::{config, fs, marketplace, plugins, terminal};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(state::AppState::new())
        .invoke_handler(tauri::generate_handler![
            // filesystem / drzewo plikow
            fs::fs_read_dir,
            fs::fs_read_file,
            fs::fs_write_file,
            fs::fs_create_file,
            fs::fs_create_dir,
            fs::fs_delete,
            fs::fs_rename,
            fs::fs_exists,
            fs::fs_is_dir,
            // konfiguracja / sesja
            config::config_load,
            config::config_save,
            config::session_load,
            config::session_save,
            config::config_themes,
            // terminal
            terminal::terminal_run,
            // marketplace
            marketplace::marketplace_fetch,
            marketplace::marketplace_install,
            marketplace::marketplace_uninstall,
            // pluginy
            plugins::plugins_scan,
        ])
        .run(tauri::generate_context!())
        .expect("blad podczas uruchamiania aplikacji hdev");
}
