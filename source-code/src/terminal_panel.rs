use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct TerminalLine {
    pub text: String,
    pub kind: TermLineKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TermLineKind {
    Output,
    Error,
    Input,
    Info,
}

pub struct TerminalPanel {
    pub history:         Vec<TerminalLine>,
    pub input:           String,
    /// Pozycja kursora w znakach (nie bajtach)
    pub cursor:          usize,
    pub cmd_history:     Vec<String>,
    pub cmd_history_idx: usize,
    /// 0 = pokazuj dół (najnowsze). Rośnie ku górze.
    pub scroll_offset:   usize,
    pub cwd:             String,
}

impl TerminalPanel {
    pub fn new() -> Self {
        let cwd = std::env::current_dir()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
        let mut t = Self {
            history:         Vec::new(),
            input:           String::new(),
            cursor:          0,
            cmd_history:     Vec::new(),
            cmd_history_idx: 0,
            scroll_offset:   0,
            cwd:             cwd.clone(),
        };
        t.push_info("hdev terminal  —  Ctrl+B zamknij  •  Esc = wróć do edytora");
        t.push_info(&format!("cwd: {}", cwd));
        t.push_info("──────────────────────────────────────────────────────────");
        t
    }

    pub fn restore_history(&mut self, hist: &[String]) {
        self.cmd_history = hist.to_vec();
    }

    fn push_line(&mut self, text: &str, kind: TermLineKind) {
        // Każdą linię dzielimy po \n i dodajemy osobno
        for line in text.split('\n') {
            let clean = strip_ansi(line);
            self.history.push(TerminalLine { text: clean, kind: kind.clone() });
        }
        self.scroll_offset = 0; // automatycznie na dół
    }

    pub fn push_info(&mut self, text: &str) {
        self.push_line(text, TermLineKind::Info);
    }

    pub fn push_output(&mut self, text: &str) {
        self.push_line(text, TermLineKind::Output);
    }

    pub fn push_error(&mut self, text: &str) {
        self.push_line(text, TermLineKind::Error);
    }

    pub fn execute(&mut self) {
        let cmd = self.input.trim().to_string();
        if cmd.is_empty() { return; }

        // Pokaż wpisaną komendę
        self.history.push(TerminalLine {
            text: format!("❯ {}", cmd),
                          kind: TermLineKind::Input,
        });

        // Historia komend
        if self.cmd_history.first().map(|s| s.as_str()) != Some(cmd.as_str()) {
            self.cmd_history.insert(0, cmd.clone());
            self.cmd_history.truncate(500);
        }
        self.cmd_history_idx = 0;
        self.input.clear();
        self.cursor = 0;
        self.scroll_offset = 0;

        // ── Wbudowane: cd ──────────────────────────────────────────────
        if cmd == "cd" || cmd.starts_with("cd ") {
            let dir = if cmd == "cd" {
                "~".to_string()
            } else {
                cmd[3..].trim().to_string()
            };
            let new_path = if dir == "~" {
                dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/"))
            } else if dir.starts_with('/') {
                std::path::PathBuf::from(&dir)
            } else {
                std::path::Path::new(&self.cwd).join(&dir)
            };
            match std::env::set_current_dir(&new_path) {
                Ok(_) => {
                    self.cwd = new_path
                    .canonicalize()
                    .unwrap_or(new_path)
                    .to_string_lossy()
                    .to_string();
                    self.push_info(&format!("→ {}", self.cwd));
                }
                Err(e) => self.push_error(&format!("cd: {}", e)),
            }
            return;
        }

        // ── Wbudowane: clear / cls ──────────────────────────────────────
        if cmd == "clear" || cmd == "cls" {
            self.history.clear();
            self.push_info("Terminal wyczyszczony.");
            return;
        }

        // ── Wykonaj komendę ─────────────────────────────────────────────
        let shell = if which_exists("hsh") { "hsh" } else { "sh" };

        match Command::new(shell)
        .arg("-c")
        .arg(&cmd)
        .current_dir(&self.cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                if !stdout.trim_end().is_empty() { self.push_output(&stdout); }
                if !stderr.trim_end().is_empty() { self.push_error(&stderr); }
                if stdout.trim().is_empty() && stderr.trim().is_empty() {
                    if out.status.success() {
                        self.push_info("✓");
                    } else {
                        self.push_error(&format!("exit {}", out.status.code().unwrap_or(-1)));
                    }
                }
            }
            Err(e) => self.push_error(&format!("Błąd: {}", e)),
        }
    }

    // ── Edycja inputu (wszystko przez char-indexy, nigdy bajty) ────────
    pub fn insert_char(&mut self, c: char) {
        let byte = char_idx_to_byte(&self.input, self.cursor);
        self.input.insert(byte, c);
        self.cursor += 1;
    }

    pub fn delete_char_before(&mut self) {
        if self.cursor == 0 { return; }
        self.cursor -= 1;
        let byte = char_idx_to_byte(&self.input, self.cursor);
        self.input.remove(byte);
    }

    pub fn delete_char_at(&mut self) {
        let len_chars = self.input.chars().count();
        if self.cursor >= len_chars { return; }
        let byte = char_idx_to_byte(&self.input, self.cursor);
        self.input.remove(byte);
    }

    pub fn move_left(&mut self) {
        if self.cursor > 0 { self.cursor -= 1; }
    }

    pub fn move_right(&mut self) {
        if self.cursor < self.input.chars().count() { self.cursor += 1; }
    }

    pub fn move_home(&mut self) { self.cursor = 0; }

    pub fn move_end(&mut self) { self.cursor = self.input.chars().count(); }

    pub fn history_up(&mut self) {
        if self.cmd_history_idx < self.cmd_history.len() {
            self.cmd_history_idx += 1;
            let idx = self.cmd_history_idx - 1;
            self.input = self.cmd_history[idx].clone();
            self.cursor = self.input.chars().count();
        }
    }

    pub fn history_down(&mut self) {
        if self.cmd_history_idx == 0 { return; }
        self.cmd_history_idx -= 1;
        if self.cmd_history_idx == 0 {
            self.input.clear();
            self.cursor = 0;
        } else {
            let idx = self.cmd_history_idx - 1;
            self.input = self.cmd_history[idx].clone();
            self.cursor = self.input.chars().count();
        }
    }

    pub fn scroll_up(&mut self, n: usize) {
        let max = self.history.len().saturating_sub(1);
        self.scroll_offset = (self.scroll_offset + n).min(max);
    }

    pub fn scroll_down(&mut self, n: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(n);
    }

    /// Zwraca [start, end) linii historii do wyświetlenia w oknie o wys. `view_h`
    pub fn visible_range(&self, view_h: usize) -> (usize, usize) {
        let total = self.history.len();
        if total == 0 || view_h == 0 { return (0, 0); }
        let end   = total.saturating_sub(self.scroll_offset);
        let start = end.saturating_sub(view_h);
        (start, end.min(total))
    }

    pub fn prompt(&self) -> String {
        let home = dirs::home_dir()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
        let display = self.cwd.replace(&home, "~");
        format!("{} ❯ ", display)
    }

    /// Tekst przed kursorem
    pub fn input_before_cursor(&self) -> String {
        self.input.chars().take(self.cursor).collect()
    }

    /// Znak pod kursorem (lub spacja gdy na końcu)
    pub fn input_cursor_char(&self) -> String {
        self.input.chars().nth(self.cursor)
        .map(|c| c.to_string())
        .unwrap_or_else(|| " ".to_string())
    }

    /// Tekst po kursorze
    pub fn input_after_cursor(&self) -> String {
        let len = self.input.chars().count();
        if self.cursor + 1 >= len {
            String::new()
        } else {
            self.input.chars().skip(self.cursor + 1).collect()
        }
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Bezpieczna konwersja indeksu znakowego na bajtowy
fn char_idx_to_byte(s: &str, char_idx: usize) -> usize {
    s.char_indices()
    .nth(char_idx)
    .map(|(b, _)| b)
    .unwrap_or(s.len())
}

fn which_exists(bin: &str) -> bool {
    Command::new("which")
    .arg(bin)
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .status()
    .map(|s| s.success())
    .unwrap_or(false)
}

/// Usuwa sekwencje ANSI escape (\x1b[...m, \x1b[...A, itp.)
fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Konsumuj do końca sekwencji
            match chars.peek() {
                Some('[') => {
                    chars.next(); // consume '['
                    // Konsumuj cyfry, ';' itp. aż do litery
                    for nc in chars.by_ref() {
                        if nc.is_ascii_alphabetic() { break; }
                    }
                }
                Some(']') => {
                    chars.next();
                    for nc in chars.by_ref() {
                        if nc == '\x07' || nc == '\x1b' { break; }
                    }
                }
                _ => {} // nieznana sekwencja — pomiń
            }
        } else if c == '\r' {
            // Ignoruj CR
        } else {
            out.push(c);
        }
    }
    out
}
