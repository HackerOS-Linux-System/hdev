use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, BorderType, Paragraph, List, ListItem, ListState,
        Clear, Wrap,
    },
};

use crate::app::{App, AppScreen, InputMode, StatusKind};
use crate::editor::EditorMode;
use crate::highlight::Highlighter;
use crate::keybinds::keybind_help;
use crate::marketplace::MarketplaceTab;
use crate::utils::file_icon;

// ─── colour palette ──────────────────────────────────────────────────────────
const BG:       Color = Color::Rgb(13,  15,  20);
const BG2:      Color = Color::Rgb(20,  24,  32);
const BG3:      Color = Color::Rgb(28,  34,  46);
const BORDER:   Color = Color::Rgb(40,  50,  70);
const BORDER_ACTIVE: Color = Color::Rgb(0, 200, 150);
const FG:       Color = Color::Rgb(210, 215, 225);
const FG_DIM:   Color = Color::Rgb(100, 110, 130);
const ACCENT:   Color = Color::Rgb(0,   220, 150);
const ACCENT2:  Color = Color::Rgb(80,  160, 255);
const WARN:     Color = Color::Rgb(255, 180, 50);
const ERR:      Color = Color::Rgb(255, 80,  80);
const HL_LINE:  Color = Color::Rgb(25,  30,  42);
const LINE_NR:  Color = Color::Rgb(55,  65,  85);
const LINE_NR_ACTIVE: Color = Color::Rgb(130, 140, 160);
const TAB_BG:   Color = Color::Rgb(18,  22,  30);
const TAB_ACTIVE: Color = Color::Rgb(0, 200, 150);

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();

    // Fill background
    f.render_widget(
        Block::default().style(Style::default().bg(BG)),
        size,
    );

    match &app.screen {
        AppScreen::Welcome    => draw_welcome(f, size, app),
        AppScreen::Editor     => draw_main_layout(f, size, app),
        AppScreen::Marketplace=> draw_marketplace(f, size, app),
        AppScreen::Settings   => draw_settings(f, size, app),
    }

    // Overlays
    if app.input_mode == InputMode::NewFileName ||
       app.input_mode == InputMode::SaveAs ||
       app.input_mode == InputMode::OpenPath {
        draw_input_dialog(f, size, app);
    }
    if app.input_mode == InputMode::Search {
        draw_search_bar(f, size, app);
    }
    if app.input_mode == InputMode::Command {
        draw_command_bar(f, size, app);
    }
    if app.show_help {
        draw_help_overlay(f, size);
    }
    if app.screen == AppScreen::Editor && app.autocomplete.visible && !app.focus_terminal {
        draw_autocomplete(f, size, app);
    }
    if app.show_confirm_delete {
        draw_confirm_dialog(f, size, app);
    }
}

// ─── WELCOME ─────────────────────────────────────────────────────────────────
fn draw_welcome(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(area);

    // Statusbar top
    let title_bar = Paragraph::new(Line::from(vec![
        Span::styled("  hdev ", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled("v0.1.0 ", Style::default().fg(FG_DIM)),
        Span::styled("— HackerOS Code Editor", Style::default().fg(FG_DIM)),
    ])).style(Style::default().bg(BG2));
    f.render_widget(title_bar, chunks[0]);

    let main = chunks[1];

    // Center column
    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(56),
            Constraint::Fill(1),
        ])
        .split(main);

    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(26),
            Constraint::Fill(1),
        ])
        .split(horiz[1]);

    let panel = vert[1];

    // ASCII logo + title
    let logo_lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(" ██╗  ██╗██████╗ ███████╗██╗   ██╗", Style::default().fg(ACCENT)),
        ]),
        Line::from(vec![
            Span::styled(" ██║  ██║██╔══██╗██╔════╝██║   ██║", Style::default().fg(ACCENT)),
        ]),
        Line::from(vec![
            Span::styled(" ███████║██║  ██║█████╗  ██║   ██║", Style::default().fg(ACCENT)),
        ]),
        Line::from(vec![
            Span::styled(" ██╔══██║██║  ██║██╔══╝  ╚██╗ ██╔╝", Style::default().fg(Color::Rgb(0,180,120))),
        ]),
        Line::from(vec![
            Span::styled(" ██║  ██║██████╔╝███████╗ ╚████╔╝ ", Style::default().fg(Color::Rgb(0,150,100))),
        ]),
        Line::from(vec![
            Span::styled(" ╚═╝  ╚═╝╚═════╝ ╚══════╝  ╚═══╝  ", Style::default().fg(Color::Rgb(0,120,80))),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  HackerOS Code Editor", Style::default().fg(FG_DIM)),
            Span::styled("  ·  ", Style::default().fg(BORDER)),
            Span::styled("v0.1.0", Style::default().fg(FG_DIM)),
        ]),
        Line::from(""),
    ];

    // Menu items
    let welcome = app.welcome.as_ref().unwrap();
    let mut menu_lines: Vec<Line> = logo_lines;

    for (i, item) in welcome.items.iter().enumerate() {
        let is_sel = i == welcome.selected;
        let prefix = if is_sel { "▶ " } else { "  " };
        let style = if is_sel {
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(FG)
        };
        let _bg = if is_sel { BG3 } else { BG };
        let separator = if i == 4 && welcome.items.len() > 5 {
            // separator before recents
            vec![
                Line::from(Span::styled("  ─── Recent ────────────────────────", Style::default().fg(FG_DIM))),
            ]
        } else { vec![] };
        for s in separator { menu_lines.push(s); }
        menu_lines.push(Line::from(vec![
            Span::styled(format!("  {} ", item.icon()), Style::default().fg(ACCENT2)),
            Span::styled(format!("{}{}", prefix, item.label()), style),
        ]));
    }

    menu_lines.push(Line::from(""));
    menu_lines.push(Line::from(vec![
        Span::styled("  Ctrl+H Pomoc", Style::default().fg(FG_DIM)),
        Span::styled("  ·  ", Style::default().fg(BORDER)),
        Span::styled("↑↓ Navigate", Style::default().fg(FG_DIM)),
        Span::styled("  ·  ", Style::default().fg(BORDER)),
        Span::styled("Enter Select", Style::default().fg(FG_DIM)),
    ]));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER_ACTIVE))
        .style(Style::default().bg(BG2));

    let para = Paragraph::new(menu_lines)
        .block(block)
        .style(Style::default().bg(BG2));
    f.render_widget(para, panel);

    draw_statusbar(f, chunks[2], app);
}

// ─── MAIN LAYOUT ─────────────────────────────────────────────────────────────
fn draw_main_layout(f: &mut Frame, area: Rect, app: &App) {
    let top_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),   // title bar
            Constraint::Length(2),   // tab bar
            Constraint::Fill(1),     // main content
            Constraint::Length(1),   // status bar
        ])
        .split(area);

    draw_titlebar(f, top_chunks[0], app);
    draw_tabs(f, top_chunks[1], app);

    let main_area = top_chunks[2];

    // Split: file tree | editor | (maybe terminal)
    let show_tree = app.show_file_tree;
    let show_term = app.show_terminal;

    let main_chunks = if show_tree {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(28),
                Constraint::Fill(1),
            ])
            .split(main_area)
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1)])
            .split(main_area)
    };

    if show_tree {
        draw_file_tree(f, main_chunks[0], app);
        let editor_area = main_chunks[1];
        if show_term {
            let vert = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(2), Constraint::Length(16)])
                .split(editor_area);
            draw_editor(f, vert[0], app);
            draw_terminal(f, vert[1], app);
        } else {
            draw_editor(f, editor_area, app);
        }
    } else {
        let editor_area = main_chunks[0];
        if show_term {
            let vert = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(2), Constraint::Length(16)])
                .split(editor_area);
            draw_editor(f, vert[0], app);
            draw_terminal(f, vert[1], app);
        } else {
            draw_editor(f, editor_area, app);
        }
    }

    draw_statusbar(f, top_chunks[3], app);
}

fn draw_titlebar(f: &mut Frame, area: Rect, app: &App) {
    let current = app.current_buffer().map(|b| b.display_name()).unwrap_or_default();
    let modified = app.current_buffer().map(|b| b.modified).unwrap_or(false);
    let lang = app.current_buffer()
        .map(|b| b.language.display_name())
        .unwrap_or("Plain Text");

    let title = Line::from(vec![
        Span::styled(" hdev ", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled("│ ", Style::default().fg(BORDER)),
        Span::styled(&current, Style::default().fg(FG)),
        if modified { Span::styled(" ●", Style::default().fg(WARN)) } else { Span::raw("") },
        Span::styled("  │ ", Style::default().fg(BORDER)),
        Span::styled(lang, Style::default().fg(FG_DIM)),
    ]);
    f.render_widget(
        Paragraph::new(title).style(Style::default().bg(BG2)),
        area,
    );
}

fn draw_tabs(f: &mut Frame, area: Rect, app: &App) {
    if app.buffers.is_empty() {
        f.render_widget(
            Block::default().style(Style::default().bg(TAB_BG)),
            area,
        );
        return;
    }

    let mut line_spans = vec![Span::raw(" ")];
    for (i, buf) in app.buffers.iter().enumerate() {
        let is_active = i == app.active_buffer;
        let name = buf.display_name();
        let modified_mark = if buf.modified { "●" } else { " " };
        let lang_icon = buf.language.icon();
        let lang_col = buf.language.icon_color();

        if is_active {
            line_spans.push(Span::styled("▌", Style::default().fg(TAB_ACTIVE)));
            line_spans.push(Span::styled(
                format!(" {} ", lang_icon),
                Style::default().fg(lang_col).bg(BG3),
            ));
            line_spans.push(Span::styled(
                format!(" {} {}", name, modified_mark),
                Style::default().fg(FG).bg(BG3).add_modifier(Modifier::BOLD),
            ));
            line_spans.push(Span::styled("▐", Style::default().fg(TAB_ACTIVE)));
        } else {
            line_spans.push(Span::styled(
                format!("  {} {} {}  ", lang_icon, name, modified_mark),
                Style::default().fg(FG_DIM).bg(TAB_BG),
            ));
        }
        line_spans.push(Span::raw(" "));
    }

    let para = Paragraph::new(Line::from(line_spans))
        .style(Style::default().bg(TAB_BG));
    f.render_widget(para, area);
}

// ─── FILE TREE ───────────────────────────────────────────────────────────────
fn draw_file_tree(f: &mut Frame, area: Rect, app: &App) {
    let tree = &app.file_tree;
    let nodes = tree.visible_nodes();
    let selected = tree.selected;

    let title = app.file_tree.root.as_ref()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("explorer");

    let items: Vec<ListItem> = nodes.iter().enumerate().map(|(i, node)| {
        let is_sel = i == selected;
        let indent = "  ".repeat(node.depth);
        let icon = if node.is_dir {
            if node.expanded { "▾ " } else { "▸ " }
        } else {
            file_icon(&node.path)
        };

        let name_style = if is_sel {
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
        } else if node.is_dir {
            Style::default().fg(ACCENT2)
        } else {
            Style::default().fg(FG)
        };

        let row_bg = if is_sel { BG3 } else { BG2 };
        let line = Line::from(vec![
            Span::raw(indent),
            Span::styled(icon, Style::default().fg(if node.is_dir { ACCENT2 } else { FG_DIM })),
            Span::styled(&node.name, name_style),
        ]).style(Style::default().bg(row_bg));

        ListItem::new(line)
    }).collect();

    let block = Block::default()
        .borders(Borders::RIGHT)
        .border_style(Style::default().fg(BORDER))
        .title(Line::from(vec![
            Span::styled(" ⊞ ", Style::default().fg(ACCENT)),
            Span::styled(title, Style::default().fg(FG)),
            Span::raw(" "),
        ]))
        .style(Style::default().bg(BG2));

    let mut state = ListState::default();
    state.select(Some(selected));

    f.render_stateful_widget(
        List::new(items).block(block),
        area,
        &mut state,
    );
}

// ─── EDITOR ──────────────────────────────────────────────────────────────────
fn draw_editor(f: &mut Frame, area: Rect, app: &App) {
    if app.buffers.is_empty() {
        let msg = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![Span::styled("  No file open", Style::default().fg(FG_DIM))]),
            Line::from(""),
            Line::from(vec![Span::styled("  Ctrl+T  new file", Style::default().fg(FG_DIM))]),
            Line::from(vec![Span::styled("  Ctrl+O  open file/folder", Style::default().fg(FG_DIM))]),
            Line::from(vec![Span::styled("  Ctrl+H  pomoc (skróty)", Style::default().fg(FG_DIM))]),
        ])
        .block(Block::default().style(Style::default().bg(BG)));
        f.render_widget(msg, area);
        return;
    }

    let buf = match app.current_buffer() {
        Some(b) => b,
        None => return,
    };

    // Layout: line numbers | code
    let ln_width = format!("{}", buf.lines.len()).len().max(3) as u16 + 2;
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(ln_width), Constraint::Fill(1)])
        .split(area);

    let view_height = area.height as usize;
    let view_width = chunks[1].width as usize;

    let scroll_row = buf.scroll_row;
    let scroll_col = buf.scroll_col;

    // Line numbers
    let mut ln_lines: Vec<Line> = Vec::new();
    for i in 0..view_height {
        let row = scroll_row + i;
        if row < buf.lines.len() {
            let is_current = row == buf.cursor_row;
            let style = if is_current {
                Style::default().fg(LINE_NR_ACTIVE).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(LINE_NR)
            };
            let _row_bg = if is_current { HL_LINE } else { BG2 };
            ln_lines.push(Line::from(
                Span::styled(format!("{:>w$} ", row + 1, w = ln_width as usize - 1), style)
            ).style(Style::default().bg(BG)));
        } else {
            ln_lines.push(Line::from(
                Span::styled(" ~ ", Style::default().fg(Color::Rgb(40, 50, 70)))
            ).style(Style::default().bg(BG2)));
        }
    }

    let ln_block = Block::default().style(Style::default().bg(BG2));
    f.render_widget(Paragraph::new(ln_lines).block(ln_block), chunks[0]);

    // Code area
    let mut code_lines: Vec<Line> = Vec::new();
    let search_matches = &buf.search_matches;
    let search_query = &buf.search_query;

    for i in 0..view_height {
        let row = scroll_row + i;
        let is_current_line = row == buf.cursor_row;
        let line_bg = if is_current_line { HL_LINE } else { BG };

        if row >= buf.lines.len() {
            code_lines.push(Line::from("").style(Style::default().bg(line_bg)));
            continue;
        }

        let line_text = &buf.lines[row];
        let _highlighted = buf.get_highlighted_line(row);

        // Collect spans with scroll offset and cursor
        let mut spans: Vec<Span> = Vec::new();

        // Build a flat char-indexed style map
        let chars: Vec<char> = line_text.chars().collect();
        let mut char_styles: Vec<Style> = vec![Style::default().fg(FG); chars.len() + 1];

        // Apply syntax highlight styles
        let raw_spans = Highlighter::highlight_line(line_text, &buf.language);
        for (start, end, style) in &raw_spans {
            for ci in *start..(*end).min(chars.len()) {
                char_styles[ci] = *style;
            }
        }

        // Apply search highlights
        for (sr, sc) in search_matches {
            if *sr == row {
                let end = (*sc + search_query.len()).min(chars.len());
                for ci in *sc..end {
                    char_styles[ci] = Style::default().fg(Color::Black).bg(Color::Rgb(255, 200, 50));
                }
            }
        }

        // Render visible portion
        let visible_start = scroll_col;
        let visible_end = (scroll_col + view_width).min(chars.len());

        if visible_start >= chars.len() {
            code_lines.push(Line::from("").style(Style::default().bg(line_bg)));
            continue;
        }

        // Group consecutive chars with same style
        let mut ci = visible_start;
        while ci < visible_end {
            let style = char_styles[ci];
            let mut s = String::new();
            while ci < visible_end && char_styles[ci] == style {
                // Render cursor position
                if row == buf.cursor_row && ci == buf.cursor_col {
                    // flush current
                    if !s.is_empty() {
                        spans.push(Span::styled(s.clone(), style.bg(line_bg)));
                        s.clear();
                    }
                    let cursor_char = chars[ci];
                    spans.push(Span::styled(
                        cursor_char.to_string(),
                        Style::default().fg(BG).bg(ACCENT),
                    ));
                    ci += 1;
                    break;
                }
                s.push(chars[ci]);
                ci += 1;
            }
            if !s.is_empty() {
                spans.push(Span::styled(s, style.bg(line_bg)));
            }
        }

        // Cursor at EOL
        if row == buf.cursor_row && buf.cursor_col >= chars.len() {
            spans.push(Span::styled(" ", Style::default().fg(BG).bg(ACCENT)));
        }

        code_lines.push(Line::from(spans).style(Style::default().bg(line_bg)));
    }

    let block = Block::default()
        .style(Style::default().bg(BG));
    f.render_widget(Paragraph::new(code_lines).block(block), chunks[1]);
}

// ─── TERMINAL ────────────────────────────────────────────────────────────────
fn draw_terminal(f: &mut Frame, area: Rect, app: &App) {
    let term = &app.terminal;

    let scroll_hint = if term.scroll_offset > 0 {
        format!("  ▲ +{} ", term.scroll_offset)
    } else {
        String::new()
    };

    let block = Block::default()
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(if app.focus_terminal { BORDER_ACTIVE } else { BORDER }))
        .title(Line::from(vec![
            Span::styled(" ❯ ", Style::default().fg(ACCENT)),
            Span::styled("terminal", Style::default().fg(FG)),
            Span::styled("  Ctrl+B zamknij  Esc wróć do edytora", Style::default().fg(FG_DIM)),
            Span::styled(&scroll_hint, Style::default().fg(WARN)),
        ]))
        .style(Style::default().bg(BG2));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if inner.height < 2 { return; }

    let hist_h     = (inner.height as usize).saturating_sub(1);
    let hist_area  = Rect { x: inner.x, y: inner.y, width: inner.width, height: hist_h as u16 };
    let input_area = Rect { x: inner.x, y: inner.y + hist_h as u16, width: inner.width, height: 1 };

    // Widoczne linie historii
    let (start, end) = term.visible_range(hist_h);
    let max_w = inner.width as usize;

    let mut hist_lines: Vec<Line> = Vec::new();
    for line in &term.history[start..end] {
        use crate::terminal_panel::TermLineKind;
        let (prefix, style) = match &line.kind {
            TermLineKind::Output => ("",   Style::default().fg(FG)),
            TermLineKind::Error  => ("ERR ", Style::default().fg(ERR)),
            TermLineKind::Input  => ("",   Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
            TermLineKind::Info   => ("",   Style::default().fg(FG_DIM)),
        };
        let full = format!("{}{}", prefix, &line.text);
        // Bezpieczne zawijanie po znakach (nie bajtach!)
        let chars: Vec<char> = full.chars().collect();
        if chars.len() <= max_w {
            hist_lines.push(Line::from(Span::styled(full, style)));
        } else {
            for chunk in chars.chunks(max_w.max(1)) {
                let s: String = chunk.iter().collect();
                hist_lines.push(Line::from(Span::styled(s, style)));
            }
        }
    }

    // Wypełnij puste wiersze na górze
    while hist_lines.len() < hist_h {
        hist_lines.insert(0, Line::from(""));
    }
    // Obetnij od góry jeśli zawijanie dodało za dużo
    while hist_lines.len() > hist_h {
        hist_lines.remove(0);
    }

    f.render_widget(
        Paragraph::new(hist_lines).style(Style::default().bg(BG2)),
        hist_area,
    );

    // Linia wejściowa — kursor na podstawie metod panelu (char-safe)
    let prompt       = term.prompt();
    let before       = term.input_before_cursor();
    let cursor_ch    = term.input_cursor_char();
    let after        = term.input_after_cursor();

    let input_line = Line::from(vec![
        Span::styled(&prompt,    Style::default().fg(ACCENT).bg(BG2)),
        Span::styled(&before,    Style::default().fg(FG).bg(BG2)),
        Span::styled(&cursor_ch, Style::default().fg(BG).bg(ACCENT)),
        Span::styled(&after,     Style::default().fg(FG).bg(BG2)),
    ]);
    f.render_widget(
        Paragraph::new(input_line).style(Style::default().bg(BG2)),
        input_area,
    );
}


// ─── MARKETPLACE ─────────────────────────────────────────────────────────────
fn draw_marketplace(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),   // title
            Constraint::Length(3),   // tabs + filter
            Constraint::Fill(1),     // list + detail
            Constraint::Length(1),   // status
            Constraint::Length(1),   // keys
        ])
        .split(area);

    let market = &app.marketplace;

    // ── Title ──
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("  ◎ hdev Marketplace", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled("  —  pluginy dla HackerOS", Style::default().fg(FG_DIM)),
            if !market.filter.is_empty() {
                Span::styled(format!("  filter: {}", market.filter), Style::default().fg(WARN))
            } else { Span::raw("") },
        ])).style(Style::default().bg(BG2)),
        chunks[0],
    );

    // ── Tabs ──
    let tabs_text: Vec<Span> = MarketplaceTab::all().iter().map(|t| {
        if t == &market.tab {
            Span::styled(t.label(), Style::default().fg(ACCENT).add_modifier(Modifier::BOLD).bg(BG3))
        } else {
            Span::styled(t.label(), Style::default().fg(FG_DIM).bg(BG2))
        }
    }).collect();
    let tabs_block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(BORDER))
        .style(Style::default().bg(BG2));
    f.render_widget(Paragraph::new(Line::from(tabs_text)).block(tabs_block), chunks[1]);

    // ── Lista + detail ──
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(38), Constraint::Fill(1)])
        .split(chunks[2]);

    // Ładowanie
    if !market.loaded {
        let msg = Paragraph::new(Line::from(vec![
            Span::styled("   Ładowanie listy pluginów...", Style::default().fg(FG_DIM)),
        ])).style(Style::default().bg(BG));
        f.render_widget(msg, chunks[2]);
    } else {
        let plugins = market.visible_plugins();
        let items: Vec<ListItem> = plugins.iter().enumerate().map(|(i, p)| {
            let is_sel      = i == market.selected;
            let row_bg      = if is_sel { BG3 } else { BG };
            let name_style  = if is_sel {
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD).bg(row_bg)
            } else {
                Style::default().fg(FG).bg(row_bg)
            };
            let is_installed = p.is_installed(&market.installed);
            let installed_badge = if is_installed {
                Span::styled(" OK", Style::default().fg(Color::Rgb(0, 200, 100)))
            } else { Span::raw("") };

            let cat_color = p.category_color();
            let cat_label = p.category_display();
            let line1 = Line::from(vec![
                Span::styled(format!("  {:8} ", cat_label), Style::default().fg(cat_color).bg(row_bg)),
                Span::styled(&p.name, name_style),
                installed_badge,
            ]).style(Style::default().bg(row_bg));
            let author = if p.author.is_empty() { "—".to_string() } else { p.author.clone() };
            let line2 = Line::from(
                Span::styled(format!("    {}", author), Style::default().fg(FG_DIM).bg(row_bg))
            ).style(Style::default().bg(row_bg));
            ListItem::new(vec![line1, line2])
        }).collect();

        let list_title = format!(" {} pluginów ", plugins.len());
        let list_block = Block::default()
            .borders(Borders::RIGHT)
            .border_style(Style::default().fg(BORDER))
            .title(Span::styled(list_title, Style::default().fg(FG_DIM)))
            .style(Style::default().bg(BG));
        let mut state = ListState::default();
        state.select(Some(market.selected));
        f.render_stateful_widget(List::new(items).block(list_block), content_chunks[0], &mut state);

        // ── Detail ──
        if let Some(plugin) = plugins.get(market.selected) {
            let is_installed = plugin.is_installed(&market.installed);
            let ver = if plugin.version.is_empty() { "?".to_string() } else { plugin.version.clone() };
            let detail_lines = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(&plugin.name, Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
                    Span::styled(format!("  v{}", ver), Style::default().fg(FG_DIM)),
                ]),
                Line::from(vec![
                    Span::styled(format!("  {}  ", plugin.category_display()), Style::default().fg(plugin.category_color())),
                    Span::styled(if plugin.author.is_empty() { "—" } else { &plugin.author }, Style::default().fg(ACCENT2)),
                ]),
                Line::from(""),
                Line::from(Span::styled("  Opis", Style::default().fg(FG_DIM))),
                Line::from("  ─────────────────────────────────"),
                Line::from(Span::styled(format!("  {}", plugin.description), Style::default().fg(FG))),
                Line::from(""),
                Line::from(Span::styled("  Plik .hk:", Style::default().fg(FG_DIM))),
                Line::from(Span::styled(format!("  {}", plugin.download), Style::default().fg(Color::Rgb(100,160,255)))),
                Line::from(""),
                if !plugin.tags.is_empty() {
                    Line::from(Span::styled(format!("  #{}", plugin.tags.join("  #")), Style::default().fg(FG_DIM)))
                } else { Line::from("") },
                Line::from(""),
                if is_installed {
                    Line::from(Span::styled("  [ Enter → Odinstaluj ]", Style::default().fg(ERR).add_modifier(Modifier::BOLD)))
                } else {
                    Line::from(Span::styled("  [ Enter → Pobierz i zainstaluj .hk ]", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)))
                },
            ];
            f.render_widget(
                Paragraph::new(detail_lines).wrap(Wrap { trim: false }).style(Style::default().bg(BG)),
                content_chunks[1],
            );
        }
    }

    // ── Status ──
    let status_style = if market.status_msg.starts_with("ERR") || market.status_msg.starts_with("Błąd") {
        Style::default().fg(ERR).bg(BG2)
    } else if market.status_msg.starts_with("OK") {
        Style::default().fg(Color::Rgb(0,200,100)).bg(BG2)
    } else {
        Style::default().fg(FG_DIM).bg(BG2)
    };
    f.render_widget(
        Paragraph::new(Span::styled(format!("  {}", market.status_msg), status_style))
            .style(Style::default().bg(BG2)),
        chunks[3],
    );

    // ── Klawisze ──
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("  Tab/Shift+Tab ", Style::default().fg(ACCENT)),
            Span::styled("kategoria  ", Style::default().fg(FG_DIM)),
            Span::styled("↑↓ ", Style::default().fg(ACCENT)),
            Span::styled("nawiguj  ", Style::default().fg(FG_DIM)),
            Span::styled("Enter ", Style::default().fg(ACCENT)),
            Span::styled("instaluj/odinstaluj  ", Style::default().fg(FG_DIM)),
            Span::styled("litery ", Style::default().fg(ACCENT)),
            Span::styled("filtruj  ", Style::default().fg(FG_DIM)),
            Span::styled("Esc ", Style::default().fg(ACCENT)),
            Span::styled("wróć", Style::default().fg(FG_DIM)),
        ])).style(Style::default().bg(BG2)),
        chunks[4],
    );
}


// ─── SETTINGS ────────────────────────────────────────────────────────────────
fn bool_str(b: bool) -> String {
    if b { "OK tak".to_string() } else { "ERR nie".to_string() }
}

fn draw_settings(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(area);

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("   hdev Settings", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled("   — stored in ~/.cache/HackerOS/hdev/config.json", Style::default().fg(FG_DIM)),
        ])).style(Style::default().bg(BG2)),
        chunks[0],
    );

    let cfg    = &app.config;
    let sel    = app.settings_selected;

    let tab_size_str = format!("{}", cfg.tab_size);

    // Dane do wyświetlenia: (etykieta, wartość, czy_edytowalna, wskazówka)
    struct SettingRow { label: &'static str, value: String, editable: bool, hint: &'static str }

    let rows: Vec<SettingRow> = vec![
        // Sekcja General
        SettingRow { label: "── General ─────────────────────────", value: String::new(), editable: false, hint: "" },
        SettingRow { label: "  Motyw (Theme)",         value: format!("{}", cfg.theme),                              editable: true,  hint: "←→ zmień" },
        SettingRow { label: "  Rozmiar tabulacji",     value: tab_size_str.clone(),                                  editable: true,  hint: "←→ 2/4/8" },
        SettingRow { label: "  Auto Save",             value: bool_str(cfg.auto_save),                               editable: true,  hint: "←→ toggle" },
        SettingRow { label: "  Numeracja linii",       value: bool_str(cfg.show_line_numbers),                       editable: true,  hint: "←→ toggle" },
        SettingRow { label: "  Drzewo plików",         value: bool_str(cfg.show_file_tree),                          editable: true,  hint: "←→ toggle" },
        SettingRow { label: "  Zawijanie linii",       value: bool_str(cfg.word_wrap),                               editable: true,  hint: "←→ toggle" },
        SettingRow { label: "  Domyślny język",        value: format!("{}", cfg.default_language_override),          editable: true,  hint: "←→ zmień" },
        SettingRow { label: "  Autocomplete (Tab)",   value: bool_str(cfg.autocomplete_enabled),                           editable: true,  hint: "←→ toggle" },
        // Sekcja Terminal
        SettingRow { label: "── Terminal ────────────────────────", value: String::new(), editable: false, hint: "" },
        SettingRow { label: "  Shell",                 value: cfg.terminal_shell.clone(),                            editable: false, hint: "" },
        // Sekcja Marketplace
        SettingRow { label: "── Marketplace ─────────────────────", value: String::new(), editable: false, hint: "" },
        SettingRow { label: "  Marketplace JSON URL",  value: cfg.marketplace_url.clone(),                           editable: false, hint: "" },
        // Sekcja Paths
        SettingRow { label: "── Ścieżki ─────────────────────────", value: String::new(), editable: false, hint: "" },
        SettingRow { label: "  Plugin dir",            value: "~/.cache/HackerOS/hdev/plugins/".to_string(),         editable: false, hint: "" },
        SettingRow { label: "  Config",                value: "~/.cache/HackerOS/hdev/config.json".to_string(),      editable: false, hint: "" },
        SettingRow { label: "  Session",               value: "~/.cache/HackerOS/hdev/session.json".to_string(),     editable: false, hint: "" },
    ];

    let mut editable_idx = 0usize;
    let total_editable: usize = rows.iter().filter(|r| r.editable).count();
    let clamped_sel = sel.min(total_editable.saturating_sub(1));

    let lines: Vec<Line> = rows.iter().map(|row| {
        if row.value.is_empty() && !row.editable {
            // Sekcja nagłówkowa
            return Line::from(vec![
                Span::raw("  "),
                Span::styled(row.label, Style::default().fg(FG_DIM).add_modifier(Modifier::BOLD)),
            ]);
        }

        let is_sel = row.editable && editable_idx == clamped_sel;
        if row.editable { editable_idx += 1; }

        let row_bg    = if is_sel { BG3 } else { BG };
        let key_style = if is_sel {
            Style::default().fg(ACCENT).bg(row_bg).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(FG).bg(row_bg)
        };
        let val_style = if is_sel {
            Style::default().fg(WARN).bg(row_bg).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(ACCENT2).bg(row_bg)
        };
        let hint_style = Style::default().fg(FG_DIM).bg(row_bg);

        Line::from(vec![
            Span::styled(format!("{:<38}", row.label), key_style),
            Span::styled(format!(" {:<28}", &row.value), val_style),
            if is_sel { Span::styled(format!("  {}", row.hint), hint_style) } else { Span::raw("") },
        ]).style(Style::default().bg(row_bg))
    }).collect();

    // Instrukcje na dole
    let mut all_lines = lines;
    all_lines.push(Line::from(""));
    all_lines.push(Line::from(vec![
        Span::styled("  Esc ", Style::default().fg(ACCENT)),
        Span::styled("zapisz i wróć   ", Style::default().fg(FG_DIM)),
        Span::styled("↑↓ ", Style::default().fg(ACCENT)),
        Span::styled("nawiguj   ", Style::default().fg(FG_DIM)),
        Span::styled("←→ / Enter ", Style::default().fg(ACCENT)),
        Span::styled("zmień wartość", Style::default().fg(FG_DIM)),
    ]));

    let block = Block::default().style(Style::default().bg(BG));
    f.render_widget(Paragraph::new(all_lines).block(block), chunks[1]);

    draw_statusbar(f, chunks[2], app);
}

// ─── STATUS BAR ──────────────────────────────────────────────────────────────
fn draw_statusbar(f: &mut Frame, area: Rect, app: &App) {
    let buf = app.current_buffer();

    let left = if let Some(b) = buf {
        let mode_str = match &b.mode {
            EditorMode::Normal  => "  NORMAL ",
            EditorMode::Insert  => "  INSERT ",
            EditorMode::Visual  => "  VISUAL ",
            EditorMode::Command => " COMMAND ",
            EditorMode::Search  => "  SEARCH ",
        };
        let mode_color = match &b.mode {
            EditorMode::Normal  => ACCENT,
            EditorMode::Insert  => ACCENT2,
            EditorMode::Visual  => Color::Rgb(255, 150, 50),
            EditorMode::Command => Color::Rgb(200, 100, 255),
            EditorMode::Search  => WARN,
        };
        vec![
            Span::styled(mode_str, Style::default().fg(BG).bg(mode_color).add_modifier(Modifier::BOLD)),
            Span::styled(" ", Style::default().bg(BG2)),
            Span::styled(
                format!(" {} ", b.display_name()),
                Style::default().fg(FG).bg(BG2),
            ),
            if b.modified {
                Span::styled(" ● ", Style::default().fg(WARN).bg(BG2))
            } else {
                Span::raw("")
            },
        ]
    } else {
        vec![Span::styled("  hdev ", Style::default().fg(ACCENT).bg(BG2))]
    };

    // Status message
    let status_style = match &app.status_kind {
        StatusKind::Info  => Style::default().fg(FG_DIM).bg(BG2),
        StatusKind::Ok    => Style::default().fg(Color::Rgb(0, 200, 100)).bg(BG2),
        StatusKind::Error => Style::default().fg(ERR).bg(BG2),
        StatusKind::Warn  => Style::default().fg(WARN).bg(BG2),
    };

    let right = if let Some(b) = buf {
        vec![
            Span::styled(
                format!(" {} ", b.language.display_name()),
                Style::default().fg(b.language.icon_color()).bg(BG3),
            ),
            Span::styled(" ", Style::default().bg(BG2)),
            Span::styled(
                format!(" {}:{} ", b.cursor_row + 1, b.cursor_col + 1),
                Style::default().fg(FG_DIM).bg(BG2),
            ),
            Span::styled(
                format!(" {} lines ", b.lines.len()),
                Style::default().fg(FG_DIM).bg(BG2),
            ),
        ]
    } else {
        vec![]
    };

    // Build full bar
    let status_msg = Span::styled(
        format!("  {}  ", &app.status_msg),
        status_style,
    );

    // Calculate widths
    let left_width: usize = left.iter().map(|s| s.content.len()).sum();
    let right_width: usize = right.iter().map(|s| s.content.len()).sum();
    let msg_width = app.status_msg.len() + 4;
    let total = area.width as usize;
    let pad = total.saturating_sub(left_width + msg_width + right_width);

    let mut spans = left;
    spans.push(status_msg);
    spans.push(Span::styled(" ".repeat(pad), Style::default().bg(BG2)));
    spans.extend(right);

    f.render_widget(
        Paragraph::new(Line::from(spans)).style(Style::default().bg(BG2)),
        area,
    );
}

// ─── DIALOGS ─────────────────────────────────────────────────────────────────
fn draw_input_dialog(f: &mut Frame, area: Rect, app: &App) {
    let (title, prompt, hint) = match &app.input_mode {
        InputMode::NewFileName => (
            "Nowy plik",
            "Podaj nazwe pliku (np. main.rs):",
            "Esc = anuluj   Enter = utwórz",
        ),
        InputMode::SaveAs => (
            "Zapisz jako",
            "Podaj pelna sciezke pliku:",
            "Esc = anuluj   Enter = zapisz",
        ),
        InputMode::OpenPath => (
            "Otworz",
            "Wpisz sciezke pliku lub folderu:",
            "Tab = autouzupelnianie   Esc = anuluj   Enter = otworz",
        ),
        _ => ("Input", "", ""),
    };

    let width  = area.width.min(72).max(50);
    let height = 6u16;
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;
    let dialog = Rect { x, y, width, height };

    f.render_widget(Clear, dialog);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER_ACTIVE))
        .title(Line::from(vec![
            Span::styled(format!(" {} ", title), Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
        ]))
        .style(Style::default().bg(BG3));

    let inner = block.inner(dialog);
    f.render_widget(block, dialog);

    // Truncate dialog_input dla wyswietlenia jesli za dlugi
    let max_input_w = (width as usize).saturating_sub(6);
    let display_input = if app.dialog_input.len() > max_input_w {
        // Pokazuj koniec stringa (gdzie uzytkownik pisze)
        let skip = app.dialog_input.len() - max_input_w;
        format!("...{}", &app.dialog_input[skip..])
    } else {
        app.dialog_input.clone()
    };

    let content = vec![
        Line::from(Span::styled(format!(" {}", prompt), Style::default().fg(FG_DIM))),
        Line::from(vec![
            Span::styled(" > ", Style::default().fg(ACCENT)),
            Span::styled(&display_input, Style::default().fg(FG)),
            Span::styled("_", Style::default().fg(ACCENT)),
        ]),
        Line::from(""),
        Line::from(Span::styled(format!(" {}", hint), Style::default().fg(FG_DIM))),
    ];
    f.render_widget(Paragraph::new(content), inner);
}

fn draw_search_bar(f: &mut Frame, area: Rect, app: &App) {
    let buf = match app.current_buffer() { Some(b) => b, None => return };
    let width = 40u16;
    let height = 3u16;
    let x = area.width.saturating_sub(width + 2);
    let y = 2u16;
    let bar = Rect { x, y, width, height };
    f.render_widget(Clear, bar);

    let count = buf.search_matches.len();
    let idx = if count > 0 { buf.search_idx + 1 } else { 0 };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(WARN))
        .title(Line::from(vec![
            Span::styled(format!("  {}/{} ", idx, count), Style::default().fg(WARN)),
        ]))
        .style(Style::default().bg(BG3));

    let inner = block.inner(bar);
    f.render_widget(block, bar);

    let line = Line::from(vec![
        Span::styled(" ❯ ", Style::default().fg(WARN)),
        Span::styled(&app.dialog_input, Style::default().fg(FG)),
        Span::styled("█", Style::default().fg(WARN)),
    ]);
    f.render_widget(Paragraph::new(line), inner);
}

fn draw_command_bar(f: &mut Frame, area: Rect, app: &App) {
    let bar = Rect { x: 0, y: area.height.saturating_sub(2), width: area.width, height: 1 };
    f.render_widget(Clear, bar);
    let line = Line::from(vec![
        Span::styled(" : ", Style::default().fg(ACCENT).bg(BG3)),
        Span::styled(&app.dialog_input, Style::default().fg(FG).bg(BG3)),
        Span::styled("█", Style::default().fg(ACCENT).bg(BG3)),
    ]);
    f.render_widget(Paragraph::new(line).style(Style::default().bg(BG3)), bar);
}

fn draw_confirm_dialog(f: &mut Frame, area: Rect, app: &App) {
    let width = 50u16;
    let height = 5u16;
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;
    let dialog = Rect { x, y, width, height };
    f.render_widget(Clear, dialog);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(ERR))
        .title(Line::from(Span::styled(" WARN Confirm Delete ", Style::default().fg(ERR).add_modifier(Modifier::BOLD))))
        .style(Style::default().bg(BG3));

    let inner = block.inner(dialog);
    f.render_widget(block, dialog);

    let path = app.file_tree.selected_path()
        .map(|p| p.file_name().and_then(|n| n.to_str()).unwrap_or("?").to_string())
        .unwrap_or_default();
    let content = vec![
        Line::from(vec![Span::styled(format!(" Delete: {}", path), Style::default().fg(FG))]),
        Line::from(""),
        Line::from(vec![
            Span::styled(" [ Enter → Yes ]  [ Esc → Cancel ]", Style::default().fg(ERR)),
        ]),
    ];
    f.render_widget(Paragraph::new(content), inner);
}


// ─── AUTOCOMPLETE POPUP ───────────────────────────────────────────────────────
fn draw_autocomplete(f: &mut Frame, area: Rect, app: &App) {
    let ac = &app.autocomplete;
    if ac.items.is_empty() { return; }

    // Znajdź pozycję kursora na ekranie
    let (cur_screen_row, cur_screen_col) = if let Some(buf) = app.current_buffer() {
        let tree_w = if app.show_file_tree { 28u16 } else { 0u16 };
        let ln_w   = (format!("{}", buf.lines.len()).len() as u16 + 2).max(4);
        let col    = tree_w + ln_w + (buf.cursor_col - buf.scroll_col) as u16;
        // +1 title, +2 tabs, +1 border
        let row    = 3u16 + (buf.cursor_row - buf.scroll_row) as u16 + 1;
        (row, col)
    } else { return; };

    let max_label  = ac.items.iter().map(|i| i.label.len() + i.detail.len() + 6).max().unwrap_or(30);
    let popup_w    = (max_label as u16 + 4).min(50).max(28);
    let popup_h    = (ac.items.len() as u16 + 2).min(12);

    // Wybierz pozycję popup: pod kursorem lub nad
    let popup_y = if cur_screen_row + popup_h + 1 < area.height {
        cur_screen_row + 1
    } else {
        cur_screen_row.saturating_sub(popup_h)
    };
    let popup_x = cur_screen_col.min(area.width.saturating_sub(popup_w));

    let popup_area = Rect {
        x: popup_x, y: popup_y,
        width: popup_w, height: popup_h,
    };

    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER_ACTIVE))
        .style(Style::default().bg(BG3));

    let inner = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let items: Vec<ratatui::widgets::ListItem> = ac.items.iter().enumerate().map(|(i, item)| {
        let is_sel = i == ac.selected;
        let item_bg = if is_sel { Color::Rgb(30, 50, 70) } else { BG3 };
        let item_fg = if is_sel { ACCENT } else { FG };

        let kind_color = item.kind.color();
        let kind_icon  = item.kind.icon();

        let label_w = inner.width.saturating_sub(item.detail.len() as u16 + 5) as usize;
        let label_truncated = if item.label.len() > label_w {
            format!("{}", &item.label[..label_w.saturating_sub(1)])
        } else {
            item.label.clone()
        };

        let line = ratatui::text::Line::from(vec![
            Span::styled(format!("{} ", kind_icon), Style::default().fg(kind_color).bg(item_bg)),
            Span::styled(format!("{:<w$}", label_truncated, w = label_w), Style::default().fg(item_fg).bg(item_bg)),
            Span::styled(format!(" {}", item.detail), Style::default().fg(FG_DIM).bg(item_bg)),
        ]).style(Style::default().bg(item_bg));

        ratatui::widgets::ListItem::new(line)
    }).collect();

    let mut state = ratatui::widgets::ListState::default();
    state.select(Some(ac.selected));

    f.render_stateful_widget(
        ratatui::widgets::List::new(items),
        inner,
        &mut state,
    );
}

fn draw_help_overlay(f: &mut Frame, area: Rect) {
    let width = 52u16;
    let pairs = keybind_help();
    let height = (pairs.len() as u16) + 4;
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;
    let dialog = Rect { x, y, width, height };
    f.render_widget(Clear, dialog);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER_ACTIVE))
        .title(Line::from(Span::styled(" ⌨ Skróty klawiszowe — Ctrl+H zamknij ", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))))
        .style(Style::default().bg(BG3));

    let inner = block.inner(dialog);
    f.render_widget(block, dialog);

    let lines: Vec<Line> = pairs.iter().map(|(k, desc)| {
        Line::from(vec![
            Span::styled(format!("  {:<20}", k), Style::default().fg(ACCENT)),
            Span::styled(*desc, Style::default().fg(FG)),
        ])
    }).collect();
    f.render_widget(Paragraph::new(lines), inner);
}
