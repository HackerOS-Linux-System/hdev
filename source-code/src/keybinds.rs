use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    NewFile, OpenFile, SaveFile, SaveFileAs, CloseFile, DeleteFile,
    CursorUp, CursorDown, CursorLeft, CursorRight,
    CursorLineStart, CursorLineEnd, CursorFileStart, CursorFileEnd,
    CursorWordForward, CursorWordBackward, PageUp, PageDown,
    InsertChar(char), InsertNewline, InsertTab,
    DeleteBackward, DeleteForward, DeleteLine, DuplicateLine,
    Undo, Redo,
    ToggleTerminal, ToggleFileTree, RefreshEnv,
    OpenMarketplace, OpenSettings, ToggleHelp,
    NextTab, PrevTab,
    StartSearch, SearchNext, SearchPrev,
    OpenCommand,
    Escape, None,
}

pub fn map_key(event: KeyEvent) -> Action {
    let ctrl  = event.modifiers.contains(KeyModifiers::CONTROL);
    let shift = event.modifiers.contains(KeyModifiers::SHIFT);
    let alt   = event.modifiers.contains(KeyModifiers::ALT);

    match event.code {
        KeyCode::Char('t') if ctrl              => Action::NewFile,
        KeyCode::Char('w') if ctrl              => Action::DeleteFile,
        KeyCode::Char('s') if ctrl && shift     => Action::SaveFileAs,
        KeyCode::Char('s') if ctrl              => Action::SaveFile,
        KeyCode::Char('o') if ctrl              => Action::OpenFile,
        KeyCode::Char('b') if ctrl              => Action::ToggleTerminal,
        KeyCode::Char('r') if ctrl              => Action::RefreshEnv,
        KeyCode::Char('m') if ctrl              => Action::OpenMarketplace,
        KeyCode::Char(',') if ctrl              => Action::OpenSettings,
        KeyCode::Char('h') if ctrl              => Action::ToggleHelp,
        KeyCode::Char('f') if ctrl              => Action::StartSearch,
        KeyCode::Char('n') if ctrl              => Action::NextTab,
        KeyCode::Char('p') if ctrl              => Action::PrevTab,
        KeyCode::Char('z') if ctrl && shift     => Action::Redo,
        KeyCode::Char('z') if ctrl              => Action::Undo,
        KeyCode::Char('y') if ctrl              => Action::Redo,
        KeyCode::Char('d') if ctrl              => Action::DuplicateLine,
        KeyCode::Char(':') if ctrl              => Action::OpenCommand,
        KeyCode::Up                             => Action::CursorUp,
        KeyCode::Down                           => Action::CursorDown,
        KeyCode::Left  if ctrl                  => Action::CursorWordBackward,
        KeyCode::Right if ctrl                  => Action::CursorWordForward,
        KeyCode::Left                           => Action::CursorLeft,
        KeyCode::Right                          => Action::CursorRight,
        KeyCode::Home                           => Action::CursorLineStart,
        KeyCode::End                            => Action::CursorLineEnd,
        KeyCode::PageUp                         => Action::PageUp,
        KeyCode::PageDown                       => Action::PageDown,
        KeyCode::Enter                          => Action::InsertNewline,
        KeyCode::Tab                            => Action::InsertTab,
        KeyCode::Backspace                      => Action::DeleteBackward,
        KeyCode::Delete                         => Action::DeleteForward,
        KeyCode::Esc                            => Action::Escape,
        KeyCode::Char(c) if !ctrl && !alt       => Action::InsertChar(c),
        _                                       => Action::None,
    }
}

pub fn keybind_help() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Ctrl+H",       "Pomoc (ta lista)"),
        ("Ctrl+T",       "Nowy plik"),
        ("Ctrl+W",       "Zamknij / usuń plik"),
        ("Ctrl+S",       "Zapisz"),
        ("Ctrl+Shift+S", "Zapisz jako"),
        ("Ctrl+O",       "Otwórz plik / folder"),
        ("Ctrl+B",       "Pokaż / ukryj terminal"),
        ("Ctrl+R",       "Odśwież drzewo plików"),
        ("Ctrl+M",       "Marketplace"),
        ("Ctrl+,",       "Ustawienia"),
        ("Ctrl+F",       "Szukaj w pliku"),
        ("Ctrl+Z",       "Cofnij (Undo)"),
        ("Ctrl+Y",       "Ponów (Redo)"),
        ("Ctrl+D",       "Duplikuj linię"),
        ("Ctrl+N",       "Następna zakładka"),
        ("Ctrl+P",       "Poprzednia zakładka"),
        ("Home / End",   "Początek / koniec linii"),
        ("Ctrl+←/→",     "Skocz o słowo"),
        ("PgUp/PgDn",    "Przewiń stronę"),
        ("Esc",          "Zamknij panel / wyjdź z trybu"),
    ]
}
