use std::io;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::config::{HdevConfig, SessionData};
use crate::editor::EditorBuffer;
use crate::filetree::FileTree;
use crate::keybinds::{map_key, Action};
use crate::marketplace::Marketplace;
use crate::plugins::PluginManager;
use crate::terminal_panel::TerminalPanel;
use crate::ui;
use crate::welcome::{WelcomeAction, WelcomeScreen};

#[derive(Debug, Clone, PartialEq)]
pub enum AppScreen {
    Welcome,
    Editor,
    Marketplace,
    Settings,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    NewFileName,
    SaveAs,
    OpenPath,
    Search,
    Command,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatusKind {
    Info, Ok, Error, Warn,
}

/// Z jakiego ekranu przyszedł użytkownik do Settings/Marketplace
/// — żeby Esc wracał tam, a nie do Welcome/filetree
#[derive(Debug, Clone, PartialEq)]
pub enum PrevScreen {
    Editor,
    Welcome,
}

pub struct App {
    pub screen:              AppScreen,
    pub prev_screen:         PrevScreen,
    pub config:              HdevConfig,
    pub buffers:             Vec<EditorBuffer>,
    pub active_buffer:       usize,
    pub file_tree:           FileTree,
    pub terminal:            TerminalPanel,
    pub marketplace:         Marketplace,
    pub plugins:             PluginManager,
    pub welcome:             Option<WelcomeScreen>,
    pub show_file_tree:      bool,
    pub show_terminal:       bool,
    pub focus_terminal:      bool,
    pub input_mode:          InputMode,
    pub dialog_input:        String,
    pub status_msg:          String,
    pub status_kind:         StatusKind,
    pub status_time:         Instant,
    pub show_help:           bool,
    pub show_confirm_delete: bool,
    pub settings_selected:   usize,
    pub quit:                bool,
}

impl App {
    pub fn new(path: Option<&str>) -> Result<Self> {
        let config   = HdevConfig::load()?;
        let plugins  = PluginManager::new();
        let mut file_tree = FileTree::new();
        let mut buffers   = Vec::new();

        let screen = if let Some(p) = path {
            let pb = PathBuf::from(p);
            if pb.is_dir() {
                file_tree.load(&pb);
                AppScreen::Editor
            } else if pb.is_file() {
                if let Ok(buf) = EditorBuffer::from_file(pb.clone()) { buffers.push(buf); }
                if let Some(parent) = pb.parent() { file_tree.load(parent); }
                AppScreen::Editor
            } else {
                AppScreen::Welcome
            }
        } else {
            AppScreen::Welcome
        };

        // Przywróć pliki z poprzedniej sesji (tylko jeśli otwieramy edytor bez argumentu)
        if screen == AppScreen::Editor && buffers.is_empty() {
            if let Ok(session) = SessionData::load() {
                for f in &session.open_files {
                    let pb = PathBuf::from(f);
                    if pb.exists() {
                        if let Ok(buf) = EditorBuffer::from_file(pb) { buffers.push(buf); }
                    }
                }
            }
        }

        // Ustal katalog roboczy dla drzewa
        if file_tree.root.is_none() {
            let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            file_tree.load(&cwd);
        }

        let welcome = if screen == AppScreen::Welcome {
            Some(WelcomeScreen::new(&config))
        } else { None };

        // Przywróć historię komend terminala
        let mut terminal = TerminalPanel::new();
        if let Ok(session) = SessionData::load() {
            terminal.restore_history(&session.terminal_history);
        }

        Ok(Self {
            screen,
            prev_screen: PrevScreen::Editor,
            show_file_tree: config.show_file_tree,
            buffers,
            active_buffer: 0,
            file_tree,
            terminal,
            marketplace: Marketplace::new(),
           plugins,
           welcome,
           show_terminal: false,
           focus_terminal: false,
           input_mode: InputMode::Normal,
           dialog_input: String::new(),
           status_msg: String::new(),
           status_kind: StatusKind::Info,
           status_time: Instant::now(),
           show_help: false,
           show_confirm_delete: false,
           settings_selected: 0,
           quit: false,
           config,
        })
    }

    pub fn current_buffer(&self) -> Option<&EditorBuffer> {
        self.buffers.get(self.active_buffer)
    }

    pub fn current_buffer_mut(&mut self) -> Option<&mut EditorBuffer> {
        self.buffers.get_mut(self.active_buffer)
    }

    pub fn set_status(&mut self, msg: &str, kind: StatusKind) {
        self.status_msg  = msg.to_string();
        self.status_kind = kind;
        self.status_time = Instant::now();
    }

    // ── Główna pętla ──────────────────────────────────────────────────────────
    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend  = CrosstermBackend::new(stdout);
        let mut term = Terminal::new(backend)?;

        loop {
            term.draw(|f| ui::draw(f, self))?;

            if event::poll(Duration::from_millis(40))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key);
                }
            }

            // Auto-reset statusu po 4 sekundach (nie dla błędów)
            if self.status_time.elapsed() > Duration::from_secs(4)
                && self.status_kind != StatusKind::Error
                {
                    self.status_msg  = self.default_status();
                    self.status_kind = StatusKind::Info;
                    // Reset zegara na 9999s żeby nie resetować w kółko
                    self.status_time = Instant::now() + Duration::from_secs(9999);
                }

                if self.quit { break; }
        }

        self.save_session();
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn save_session(&self) {
        let open_files: Vec<String> = self.buffers.iter()
        .filter_map(|b| b.path.as_ref().map(|p| p.to_string_lossy().to_string()))
        .collect();
        let term_hist = self.terminal.cmd_history.clone();
        let session = SessionData {
            open_files,
            active_file: self.current_buffer()
            .and_then(|b| b.path.as_ref())
            .map(|p| p.to_string_lossy().to_string()),
            panel_state: "editor".to_string(),
            terminal_history: term_hist,
        };
        let _ = session.save();
        let _ = self.config.save();
    }

    fn default_status(&self) -> String {
        match &self.screen {
            AppScreen::Welcome      => "↑↓ Nawiguj · Enter Wybierz · Ctrl+Q Wyjdź".to_string(),
            AppScreen::Editor       => {
                if let Some(b) = self.current_buffer() {
                    format!("Ctrl+S Zapisz · Ctrl+B Terminal · Ln {} Col {}",
                        b.cursor_row + 1, b.cursor_col + 1)
                } else {
                    "Ctrl+T Nowy plik · Ctrl+O Otwórz folder".to_string()
                }
            }
            AppScreen::Marketplace  => "Tab kategoria · ↑↓ nawiguj · Enter instaluj · Esc wróć".to_string(),
            AppScreen::Settings     => "↑↓ nawiguj · ←→ zmień wartość · Esc zapisz i wróć".to_string(),
        }
    }

    // ── Dispatcher klawiszy ──────────────────────────────────────────────────
    fn handle_key(&mut self, key: KeyEvent) {
        // Ctrl+Q — wyjdź zawsze
        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('q') {
            self.quit = true;
            return;
        }

        // Overlaye (dialogi input) mają najwyższy priorytet
        match &self.input_mode {
            InputMode::NewFileName | InputMode::SaveAs | InputMode::OpenPath => {
                self.handle_dialog_input(key);
                return;
            }
            InputMode::Search  => { self.handle_search_input(key); return; }
            InputMode::Command => { self.handle_command_input(key); return; }
            InputMode::Normal  => {}
        }

        // Dialog potwierdzenia usunięcia
        if self.show_confirm_delete {
            match key.code {
                KeyCode::Enter => {
                    if let Err(e) = self.file_tree.delete_selected() {
                        self.set_status(&format!("Błąd usuwania: {}", e), StatusKind::Error);
                    } else {
                        self.set_status("Plik usunięty.", StatusKind::Ok);
                    }
                    self.show_confirm_delete = false;
                }
                KeyCode::Esc => { self.show_confirm_delete = false; }
                _ => {}
            }
            return;
        }

        // Help overlay — Ctrl+H
        if self.show_help {
            match key.code {
                KeyCode::Esc => self.show_help = false,
                _ => if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('h') {
                    self.show_help = false;
                },
            }
            return;
        }

        // Ctrl+H = toggle help (na każdym ekranie)
        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('h') {
            self.show_help = !self.show_help;
            return;
        }

        match &self.screen {
            AppScreen::Welcome     => self.handle_welcome_key(key),
            AppScreen::Editor      => {
                if self.focus_terminal {
                    self.handle_terminal_key(key);
                } else {
                    self.handle_editor_key(key);
                }
            }
            AppScreen::Marketplace => self.handle_marketplace_key(key),
            AppScreen::Settings    => self.handle_settings_key(key),
        }
    }

    // ── Welcome ───────────────────────────────────────────────────────────────
    fn handle_welcome_key(&mut self, key: KeyEvent) {
        let w = match &mut self.welcome { Some(w) => w, None => return };
        match key.code {
            KeyCode::Up    => w.move_up(),
            KeyCode::Down  => w.move_down(),
            KeyCode::Enter => {
                let action = w.select();
                self.handle_welcome_action(action);
            }
            _ => {}
        }
    }

    fn handle_welcome_action(&mut self, action: WelcomeAction) {
        match action {
            WelcomeAction::OpenEditor => {
                self.prev_screen = PrevScreen::Editor;
                self.screen = AppScreen::Editor;
            }
            WelcomeAction::OpenTerminal => {
                self.prev_screen = PrevScreen::Editor;
                self.screen = AppScreen::Editor;
                self.show_terminal = true;
                self.focus_terminal = true;
            }
            WelcomeAction::OpenMarketplace => {
                self.prev_screen = PrevScreen::Welcome;
                self.screen = AppScreen::Marketplace;
            }
            WelcomeAction::OpenSettings => {
                self.prev_screen = PrevScreen::Welcome;
                self.screen = AppScreen::Settings;
            }
            WelcomeAction::OpenRecentFile(path) => {
                let pb = PathBuf::from(&path);
                match EditorBuffer::from_file(pb.clone()) {
                    Ok(buf) => {
                        self.buffers.push(buf);
                        self.active_buffer = self.buffers.len() - 1;
                        if let Some(parent) = pb.parent() { self.file_tree.load(parent); }
                        self.prev_screen = PrevScreen::Editor;
                        self.screen = AppScreen::Editor;
                        self.config.add_recent_file(&path);
                        self.set_status(&format!("Otwarto: {}", path), StatusKind::Ok);
                    }
                    Err(e) => self.set_status(&format!("Błąd: {}", e), StatusKind::Error),
                }
            }
            WelcomeAction::None => {}
        }
    }

    // ── Editor ───────────────────────────────────────────────────────────────
    fn handle_editor_key(&mut self, key: KeyEvent) {
        let ctrl  = key.modifiers.contains(KeyModifiers::CONTROL);
        let shift = key.modifiers.contains(KeyModifiers::SHIFT);

        // Ctrl+Shift+S = Save As
        if ctrl && shift && key.code == KeyCode::Char('s') { self.cmd_save_as(); return; }

        if ctrl {
            match key.code {
                KeyCode::Char('t') => { self.cmd_new_file(); return; }
                KeyCode::Char('w') => { self.cmd_delete_file(); return; }
                KeyCode::Char('s') => { self.cmd_save(); return; }
                KeyCode::Char('o') => { self.cmd_open_path(); return; }
                KeyCode::Char('b') => {
                    self.toggle_terminal();
                    return;
                }
                KeyCode::Char('r') => {
                    self.file_tree.refresh();
                    self.set_status("Drzewo plików odświeżone.", StatusKind::Ok);
                    return;
                }
                KeyCode::Char('m') => {
                    self.prev_screen = PrevScreen::Editor;
                    self.screen = AppScreen::Marketplace;
                    return;
                }
                KeyCode::Char(',') => {
                    self.prev_screen = PrevScreen::Editor;
                    self.screen = AppScreen::Settings;
                    return;
                }
                KeyCode::Char('f') => {
                    self.input_mode = InputMode::Search;
                    self.dialog_input.clear();
                    return;
                }
                KeyCode::Char('n') => {
                    if !self.buffers.is_empty() {
                        self.active_buffer = (self.active_buffer + 1) % self.buffers.len();
                    }
                    return;
                }
                KeyCode::Char('p') => {
                    if !self.buffers.is_empty() {
                        if self.active_buffer == 0 { self.active_buffer = self.buffers.len() - 1; }
                        else { self.active_buffer -= 1; }
                    }
                    return;
                }
                _ => {}
            }
        }

        // Alt+cyfra → zakładka
        if key.modifiers == KeyModifiers::ALT {
            if let KeyCode::Char(c) = key.code {
                if let Some(n) = c.to_digit(10) {
                    let idx = (n as usize).saturating_sub(1);
                    if idx < self.buffers.len() { self.active_buffer = idx; }
                    return;
                }
            }
        }

        // Enter w drzewie plików (gdy brak fokusa na buforze)
        if key.code == KeyCode::Enter && self.buffers.is_empty() {
            self.open_selected_from_tree();
            return;
        }

        let action = map_key(key);
        match action {
            Action::InsertChar(c)   => { if let Some(b) = self.current_buffer_mut() { b.insert_char(c); self.auto_pair_close(c); self.scroll_buf(); } }
            Action::InsertNewline   => { if let Some(b) = self.current_buffer_mut() { b.insert_newline(); self.scroll_buf(); } }
            Action::InsertTab       => { if let Some(b) = self.current_buffer_mut() { b.insert_tab(); self.scroll_buf(); } }
            Action::DeleteBackward  => { if let Some(b) = self.current_buffer_mut() { b.delete_char_before(); self.scroll_buf(); } }
            Action::DeleteForward   => { if let Some(b) = self.current_buffer_mut() { b.delete_char_at(); self.scroll_buf(); } }
            Action::DeleteLine      => { if let Some(b) = self.current_buffer_mut() { b.delete_line(); self.scroll_buf(); } }
            Action::DuplicateLine   => { if let Some(b) = self.current_buffer_mut() { b.duplicate_line(); self.scroll_buf(); } }
            Action::CursorUp        => { if let Some(b) = self.current_buffer_mut() { b.move_cursor_up();    self.scroll_buf(); } }
            Action::CursorDown      => { if let Some(b) = self.current_buffer_mut() { b.move_cursor_down();  self.scroll_buf(); } }
            Action::CursorLeft      => { if let Some(b) = self.current_buffer_mut() { b.move_cursor_left();  self.scroll_buf(); } }
            Action::CursorRight     => { if let Some(b) = self.current_buffer_mut() { b.move_cursor_right(); self.scroll_buf(); } }
            Action::CursorLineStart => { if let Some(b) = self.current_buffer_mut() { b.move_cursor_line_start(); } }
            Action::CursorLineEnd   => { if let Some(b) = self.current_buffer_mut() { b.move_cursor_line_end(); } }
            Action::CursorFileStart => { if let Some(b) = self.current_buffer_mut() { b.goto_first_line(); } }
            Action::CursorFileEnd   => { if let Some(b) = self.current_buffer_mut() { b.goto_last_line(); } }
            Action::CursorWordForward  => { if let Some(b) = self.current_buffer_mut() { b.move_word_forward();  } }
            Action::CursorWordBackward => { if let Some(b) = self.current_buffer_mut() { b.move_word_backward(); } }
            Action::PageUp   => { if let Some(b) = self.current_buffer_mut() { b.page_up(20);   self.scroll_buf(); } }
            Action::PageDown => { if let Some(b) = self.current_buffer_mut() { b.page_down(20); self.scroll_buf(); } }
            Action::Undo => { if let Some(b) = self.current_buffer_mut() { b.undo(); } self.set_status("Undo", StatusKind::Info); }
            Action::Redo => { if let Some(b) = self.current_buffer_mut() { b.redo(); } self.set_status("Redo", StatusKind::Info); }
            Action::SaveFile   => self.cmd_save(),
            Action::SaveFileAs => self.cmd_save_as(),
            Action::NewFile    => self.cmd_new_file(),
            Action::DeleteFile => self.cmd_delete_file(),
            Action::OpenCommand => {
                self.input_mode = InputMode::Command;
                self.dialog_input.clear();
            }
            Action::Escape => {
                // Esc NIE zmienia katalogu / ekranu — tylko zamyka drobne stany
                if self.show_terminal && self.focus_terminal {
                    self.focus_terminal = false;
                } else if self.show_terminal {
                    self.toggle_terminal();
                }
            }
            _ => {}
        }
    }

    fn toggle_terminal(&mut self) {
        if self.show_terminal {
            self.show_terminal   = false;
            self.focus_terminal  = false;
        } else {
            self.show_terminal  = true;
            self.focus_terminal = true;
        }
    }

    fn open_selected_from_tree(&mut self) {
        if let Some(path) = self.file_tree.selected_path() {
            if path.is_dir() {
                self.file_tree.toggle_expand();
            } else {
                match EditorBuffer::from_file(path.clone()) {
                    Ok(buf) => {
                        let name = path.to_string_lossy().to_string();
                        self.config.add_recent_file(&name);
                        self.buffers.push(buf);
                        self.active_buffer = self.buffers.len() - 1;
                    }
                    Err(e) => self.set_status(&format!("Błąd: {}", e), StatusKind::Error),
                }
            }
        }
    }

    // ── Terminal ──────────────────────────────────────────────────────────────
    fn handle_terminal_key(&mut self, key: KeyEvent) {
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

        // Ctrl+B — zamknij terminal
        if ctrl && key.code == KeyCode::Char('b') {
            self.toggle_terminal();
            return;
        }
        // Ctrl+C — wyczyść wejście
        if ctrl && key.code == KeyCode::Char('c') {
            self.terminal.input.clear();
            self.terminal.cursor = 0;
            self.terminal.push_info("^C");
            return;
        }

        match key.code {
            // Esc: oddaj fokus edytorowi, ale zostaw terminal otwarty
            KeyCode::Esc       => { self.focus_terminal = false; }
            KeyCode::Enter     => self.terminal.execute(),
            KeyCode::Backspace => self.terminal.delete_char_before(),
            KeyCode::Delete    => self.terminal.delete_char_at(),
            KeyCode::Left      => self.terminal.move_left(),
            KeyCode::Right     => self.terminal.move_right(),
            KeyCode::Home      => self.terminal.move_home(),
            KeyCode::End       => self.terminal.move_end(),
            KeyCode::Up        => self.terminal.history_up(),
            KeyCode::Down      => self.terminal.history_down(),
            KeyCode::PageUp    => self.terminal.scroll_up(5),
            KeyCode::PageDown  => self.terminal.scroll_down(5),
            KeyCode::Char(c) if !ctrl => self.terminal.insert_char(c),
            _ => {}
        }
    }

    // ── Dialog inputs ─────────────────────────────────────────────────────────
    fn handle_dialog_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.dialog_input.clear();
            }
            KeyCode::Enter => {
                let input = self.dialog_input.clone();
                let mode = std::mem::replace(&mut self.input_mode, InputMode::Normal);
                self.dialog_input.clear();
                match mode {
                    InputMode::NewFileName => {
                        match self.file_tree.create_file(&input) {
                            Ok(path) => {
                                match EditorBuffer::from_file(path) {
                                    Ok(buf) => {
                                        self.buffers.push(buf);
                                        self.active_buffer = self.buffers.len() - 1;
                                        self.set_status(&format!("Utworzono: {}", input), StatusKind::Ok);
                                    }
                                    Err(e) => self.set_status(&format!("Błąd: {}", e), StatusKind::Error),
                                }
                            }
                            Err(e) => self.set_status(&format!("Błąd: {}", e), StatusKind::Error),
                        }
                    }
                    InputMode::SaveAs => {
                        let pb = std::path::PathBuf::from(&input);
                        if let Some(buf) = self.current_buffer_mut() {
                            match buf.save_as(pb) {
                                Ok(_) => self.set_status(&format!("Zapisano jako: {}", input), StatusKind::Ok),
                                Err(e) => self.set_status(&format!("Błąd: {}", e), StatusKind::Error),
                            }
                        }
                    }
                    InputMode::OpenPath => {
                        self.open_path_str(&input);
                    }
                    _ => {}
                }
            }
            KeyCode::Backspace => { self.dialog_input.pop(); }
            KeyCode::Char(c)   => { self.dialog_input.push(c); }
            _ => {}
        }
    }

    fn handle_search_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.dialog_input.clear();
                if let Some(buf) = self.current_buffer_mut() { buf.search_matches.clear(); }
            }
            KeyCode::Enter | KeyCode::Down => {
                if let Some(buf) = self.current_buffer_mut() { buf.search_next(); }
            }
            KeyCode::Up => {
                if let Some(buf) = self.current_buffer_mut() { buf.search_prev(); }
            }
            KeyCode::Backspace => {
                self.dialog_input.pop();
                let q = self.dialog_input.clone();
                if let Some(buf) = self.current_buffer_mut() { buf.search(&q); }
            }
            KeyCode::Char(c) => {
                self.dialog_input.push(c);
                let q = self.dialog_input.clone();
                if let Some(buf) = self.current_buffer_mut() { buf.search(&q); }
            }
            _ => {}
        }
    }

    fn handle_command_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.dialog_input.clear();
            }
            KeyCode::Enter => {
                let cmd = self.dialog_input.clone();
                self.input_mode = InputMode::Normal;
                self.dialog_input.clear();
                self.exec_command(&cmd);
            }
            KeyCode::Backspace => { self.dialog_input.pop(); }
            KeyCode::Char(c)   => { self.dialog_input.push(c); }
            _ => {}
        }
    }

    fn exec_command(&mut self, cmd: &str) {
        match cmd.trim() {
            "q" | "quit" => self.quit = true,
            "w" | "write" => self.cmd_save(),
            "wq" => { self.cmd_save(); self.quit = true; }
            s => {
                // ":e <path>"
                if let Some(path) = s.strip_prefix("e ") {
                    self.open_path_str(path.trim());
                    return;
                }
                // ":123" → goto line
                if let Ok(n) = s.parse::<usize>() {
                    if let Some(buf) = self.current_buffer_mut() { buf.goto_line(n.saturating_sub(1)); }
                    return;
                }
                self.set_status(&format!("Nieznana komenda: {}", cmd), StatusKind::Warn);
            }
        }
    }

    // ── Marketplace ───────────────────────────────────────────────────────────
    fn handle_marketplace_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up      => self.marketplace.move_up(),
            KeyCode::Down    => self.marketplace.move_down(),
            KeyCode::Enter   => self.marketplace.toggle_install(),
            KeyCode::Tab     => self.marketplace.next_tab(),
            KeyCode::BackTab => self.marketplace.prev_tab(),
            KeyCode::Esc     => self.return_from_overlay(),
            _ => {}
        }
    }

    // ── Settings ──────────────────────────────────────────────────────────────
    fn handle_settings_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up    => { if self.settings_selected > 0 { self.settings_selected -= 1; } }
            KeyCode::Down  => { self.settings_selected += 1; }

            // ← → do zmiany wartości (motyw, jezyk, itp.)
            KeyCode::Left  => self.settings_change(-1),
            KeyCode::Right => self.settings_change(1),

            // Enter = toggle dla booleanów
            KeyCode::Enter => self.settings_toggle(),

            // Esc = zapisz i WRÓĆ do poprzedniego ekranu (NIE ładuj folderu!)
            KeyCode::Esc   => {
                let _ = self.config.save();
                // Zastosuj ustawienia do stanu aplikacji
                self.show_file_tree = self.config.show_file_tree;
                for buf in &mut self.buffers {
                    buf.tab_size = self.config.tab_size as usize;
                }
                self.set_status("Ustawienia zapisane.", StatusKind::Ok);
                self.return_from_overlay();
            }
            _ => {}
        }
    }

    /// Wróć do poprzedniego ekranu (Welcome lub Editor)
    fn return_from_overlay(&mut self) {
        self.screen = match self.prev_screen {
            PrevScreen::Welcome => AppScreen::Welcome,
            PrevScreen::Editor  => AppScreen::Editor,
        };
    }

    /// Zmień wartość ustawienia o delta (-1 lub +1)
    fn settings_change(&mut self, delta: i32) {
        // Indeksy odpowiadają wierszom edytowalnym w draw_settings
        match self.settings_selected {
            0 => { // Theme
                if delta > 0 { self.config.next_theme(); } else { self.config.prev_theme(); }
                self.set_status(&format!("Motyw: {}", self.config.theme), StatusKind::Info);
            }
            1 => { // Tab size
                let sizes = [2u8, 4, 8];
                let cur = self.config.tab_size;
                let idx = sizes.iter().position(|&s| s == cur).unwrap_or(1);
                let new_idx = ((idx as i32 + delta).rem_euclid(sizes.len() as i32)) as usize;
                self.config.tab_size = sizes[new_idx];
            }
            2 => { self.config.auto_save = !self.config.auto_save; }
            3 => { self.config.show_line_numbers = !self.config.show_line_numbers; }
            4 => { self.config.show_file_tree = !self.config.show_file_tree; self.show_file_tree = self.config.show_file_tree; }
            5 => { self.config.word_wrap = !self.config.word_wrap; }
            6 => { // Język domyślny
                if delta > 0 { self.config.next_language(); } else { self.config.prev_language(); }
                self.set_status(&format!("Język: {}", self.config.default_language_override), StatusKind::Info);
            }
            _ => {}
        }
    }

    fn settings_toggle(&mut self) {
        self.settings_change(1);
    }

    // ── File commands ─────────────────────────────────────────────────────────
    fn cmd_new_file(&mut self) {
        self.input_mode = InputMode::NewFileName;
        self.dialog_input.clear();
        self.set_status("Podaj nazwę pliku:", StatusKind::Info);
    }

    fn cmd_save(&mut self) {
        if let Some(buf) = self.current_buffer_mut() {
            if buf.path.is_some() {
                match buf.save() {
                    Ok(_) => {}
                    Err(e) => { self.set_status(&format!("Błąd zapisu: {}", e), StatusKind::Error); return; }
                }
                let name = buf.display_name();
                self.set_status(&format!("Zapisano: {}", name), StatusKind::Ok);
            } else {
                self.cmd_save_as();
            }
        }
    }

    fn cmd_save_as(&mut self) {
        self.input_mode = InputMode::SaveAs;
        self.dialog_input.clear();
        self.set_status("Zapisz jako (podaj ścieżkę):", StatusKind::Info);
    }

    fn cmd_open_path(&mut self) {
        self.input_mode = InputMode::OpenPath;
        self.dialog_input.clear();
        self.set_status("Otwórz plik lub folder:", StatusKind::Info);
    }

    fn cmd_delete_file(&mut self) {
        if self.file_tree.selected_path().is_some() {
            self.show_confirm_delete = true;
        } else if !self.buffers.is_empty() {
            self.buffers.remove(self.active_buffer);
            if self.active_buffer >= self.buffers.len() && !self.buffers.is_empty() {
                self.active_buffer = self.buffers.len() - 1;
            }
            self.set_status("Zakładka zamknięta.", StatusKind::Info);
        }
    }

    fn open_path_str(&mut self, path: &str) {
        let pb = PathBuf::from(path);
        if pb.is_dir() {
            self.file_tree.load(&pb);
            self.set_status(&format!("Otwarto folder: {}", path), StatusKind::Ok);
        } else if pb.is_file() || !pb.exists() {
            match EditorBuffer::from_file(pb.clone()) {
                Ok(buf) => {
                    self.config.add_recent_file(path);
                    self.buffers.push(buf);
                    self.active_buffer = self.buffers.len() - 1;
                    if let Some(parent) = pb.parent() {
                        if self.file_tree.root.is_none() { self.file_tree.load(parent); }
                    }
                    self.set_status(&format!("Otwarto: {}", path), StatusKind::Ok);
                }
                Err(e) => self.set_status(&format!("Błąd: {}", e), StatusKind::Error),
            }
        }
    }

    fn scroll_buf(&mut self) {
        if let Some(buf) = self.current_buffer_mut() {
            buf.scroll_to_cursor(40, 100);
        }
    }

    fn auto_pair_close(&mut self, c: char) {
        let pair = match c { '(' => Some(')'), '[' => Some(']'), '{' => Some('}'), _ => None };
        if let Some(close) = pair {
            if let Some(buf) = self.current_buffer_mut() {
                let row = buf.cursor_row;
                let col = buf.cursor_col;
                buf.lines[row].insert(col, close);
                // Kursor stoi przed zamknięciem — dobrze
            }
        }
    }
}
