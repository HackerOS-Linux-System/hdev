use serde::Serialize;
use std::process::Stdio;
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

/// Zdarzenie strumieniowane do frontendu podczas wykonywania komendy terminala.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TerminalEvent {
    Stdout { session_id: String, line: String },
    Stderr { session_id: String, line: String },
    Exit { session_id: String, code: i32 },
    Error { session_id: String, message: String },
}

/// Wykonuje pojedyncza komende w podanym katalogu, uzywajac skonfigurowanej
/// powloki (domyslnie `hsh`, z fallbackiem na `sh` — tak jak wersja TUI).
/// Wynik jest strumieniowany linia-po-linii jako zdarzenia `terminal://event`,
/// wiec dlugo dzialajace komendy (np. `cargo build`) pokazuja postep na biezaco.
#[tauri::command]
pub async fn terminal_run(
    app: AppHandle,
    session_id: String,
    command: String,
    cwd: String,
    shell: String,
) -> Result<(), String> {
    // Wbudowane komendy obslugiwane przez hdev, nie przez powloke.
    let trimmed = command.trim();
    if trimmed == "clear" || trimmed == "cls" {
        let _ = app.emit(
            "terminal://event",
            TerminalEvent::Exit {
                session_id: session_id.clone(),
                code: -1, // -1 sygnalizuje frontendowi "wyczysc ekran", zamiast normalnego zakonczenia
            },
        );
        return Ok(());
    }

    let shell_bin = if which(&shell) { shell.clone() } else { "sh".to_string() };

    let mut child = Command::new(&shell_bin)
        .arg("-c")
        .arg(&command)
        .current_dir(&cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Nie mozna uruchomic {shell_bin}: {e}"))?;

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let app_out = app.clone();
    let sid_out = session_id.clone();
    let out_task = tokio::spawn(async move {
        if let Some(stdout) = stdout {
            let mut lines = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = app_out.emit(
                    "terminal://event",
                    TerminalEvent::Stdout { session_id: sid_out.clone(), line },
                );
            }
        }
    });

    let app_err = app.clone();
    let sid_err = session_id.clone();
    let err_task = tokio::spawn(async move {
        if let Some(stderr) = stderr {
            let mut lines = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = app_err.emit(
                    "terminal://event",
                    TerminalEvent::Stderr { session_id: sid_err.clone(), line },
                );
            }
        }
    });

    let _ = out_task.await;
    let _ = err_task.await;

    let status = child.wait().await.map_err(|e| e.to_string())?;
    let _ = app.emit(
        "terminal://event",
        TerminalEvent::Exit {
            session_id,
            code: status.code().unwrap_or(-1),
        },
    );

    Ok(())
}

fn which(bin: &str) -> bool {
    std::env::var_os("PATH")
        .map(|paths| {
            std::env::split_paths(&paths).any(|dir| dir.join(bin).is_file())
        })
        .unwrap_or(false)
}
