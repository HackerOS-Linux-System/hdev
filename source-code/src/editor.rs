use std::path::PathBuf;
use crate::languages::Language;
use crate::highlight::Highlighter;
use ratatui::style::Style;

#[derive(Debug, Clone, PartialEq)]
pub enum EditorMode {
    Normal,
    Insert,
    Visual,
    Command,
    Search,
}

#[derive(Debug, Clone)]
pub struct EditorBuffer {
    pub path: Option<PathBuf>,
    pub lines: Vec<String>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub scroll_row: usize,
    pub scroll_col: usize,
    pub language: Language,
    pub modified: bool,
    pub mode: EditorMode,
    pub selection_start: Option<(usize, usize)>,
    pub search_query: String,
    pub search_matches: Vec<(usize, usize)>,
    pub search_idx: usize,
    pub undo_stack: Vec<UndoEntry>,
    pub redo_stack: Vec<UndoEntry>,
    pub tab_size: usize,
}

#[derive(Debug, Clone)]
pub struct UndoEntry {
    pub lines: Vec<String>,
    pub cursor_row: usize,
    pub cursor_col: usize,
}

impl EditorBuffer {
    pub fn new() -> Self {
        Self {
            path: None,
            lines: vec![String::new()],
            cursor_row: 0,
            cursor_col: 0,
            scroll_row: 0,
            scroll_col: 0,
            language: Language::PlainText,
            modified: false,
            mode: EditorMode::Normal,
            selection_start: None,
            search_query: String::new(),
            search_matches: Vec::new(),
            search_idx: 0,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            tab_size: 4,
        }
    }

    pub fn from_file(path: PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(&path)
            .unwrap_or_default();
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        let language = Language::from_extension(ext);
        let lines: Vec<String> = if content.is_empty() {
            vec![String::new()]
        } else {
            content.lines().map(|l| l.to_string()).collect()
        };

        Ok(Self {
            path: Some(path),
            lines,
            language,
            ..Self::new()
        })
    }

    pub fn display_name(&self) -> String {
        if let Some(p) = &self.path {
            p.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("untitled")
                .to_string()
        } else {
            "untitled".to_string()
        }
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        if let Some(path) = &self.path {
            let content = self.lines.join("\n");
            std::fs::write(path, content)?;
            self.modified = false;
        }
        Ok(())
    }

    pub fn save_as(&mut self, path: PathBuf) -> anyhow::Result<()> {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_string();
        self.language = Language::from_extension(&ext);
        self.path = Some(path);
        self.save()
    }

    pub fn insert_char(&mut self, c: char) {
        self.push_undo();
        let row = self.cursor_row;
        let col = self.cursor_col;
        if row < self.lines.len() {
            self.lines[row].insert(col, c);
            self.cursor_col += 1;
            self.modified = true;
        }
    }

    pub fn insert_newline(&mut self) {
        self.push_undo();
        let row = self.cursor_row;
        let col = self.cursor_col;
        if row < self.lines.len() {
            let rest = self.lines[row][col..].to_string();
            self.lines[row].truncate(col);
            // auto-indent
            let indent = get_indent(&self.lines[row]);
            let new_line = format!("{}{}", indent, rest);
            self.lines.insert(row + 1, new_line.clone());
            self.cursor_row += 1;
            self.cursor_col = indent.len();
            self.modified = true;
        }
    }

    pub fn delete_char_before(&mut self) {
        let row = self.cursor_row;
        let col = self.cursor_col;
        if col > 0 {
            self.push_undo();
            self.lines[row].remove(col - 1);
            self.cursor_col -= 1;
            self.modified = true;
        } else if row > 0 {
            self.push_undo();
            let line = self.lines.remove(row);
            let prev_len = self.lines[row - 1].len();
            self.lines[row - 1].push_str(&line);
            self.cursor_row -= 1;
            self.cursor_col = prev_len;
            self.modified = true;
        }
    }

    pub fn delete_char_at(&mut self) {
        let row = self.cursor_row;
        let col = self.cursor_col;
        if row < self.lines.len() {
            if col < self.lines[row].len() {
                self.push_undo();
                self.lines[row].remove(col);
                self.modified = true;
            } else if row + 1 < self.lines.len() {
                self.push_undo();
                let next = self.lines.remove(row + 1);
                self.lines[row].push_str(&next);
                self.modified = true;
            }
        }
    }

    pub fn insert_tab(&mut self) {
        let spaces: String = " ".repeat(self.tab_size);
        for c in spaces.chars() {
            self.insert_char(c);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.cursor_col = self.lines[self.cursor_row].len();
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_row < self.lines.len() {
            let line_len = self.lines[self.cursor_row].len();
            if self.cursor_col < line_len {
                self.cursor_col += 1;
            } else if self.cursor_row + 1 < self.lines.len() {
                self.cursor_row += 1;
                self.cursor_col = 0;
            }
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.clamp_cursor_col();
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor_row + 1 < self.lines.len() {
            self.cursor_row += 1;
            self.clamp_cursor_col();
        }
    }

    pub fn move_cursor_line_start(&mut self) {
        self.cursor_col = 0;
    }

    pub fn move_cursor_line_end(&mut self) {
        if self.cursor_row < self.lines.len() {
            self.cursor_col = self.lines[self.cursor_row].len();
        }
    }

    pub fn move_word_forward(&mut self) {
        let row = self.cursor_row;
        if row >= self.lines.len() { return; }
        let line = &self.lines[row];
        let chars: Vec<char> = line.chars().collect();
        let mut col = self.cursor_col;
        // skip current word chars
        while col < chars.len() && !chars[col].is_whitespace() { col += 1; }
        // skip whitespace
        while col < chars.len() && chars[col].is_whitespace() { col += 1; }
        self.cursor_col = col;
    }

    pub fn move_word_backward(&mut self) {
        let row = self.cursor_row;
        if row >= self.lines.len() { return; }
        let line = &self.lines[row];
        let chars: Vec<char> = line.chars().collect();
        let mut col = self.cursor_col;
        if col > 0 { col -= 1; }
        while col > 0 && chars[col].is_whitespace() { col -= 1; }
        while col > 0 && !chars[col-1].is_whitespace() { col -= 1; }
        self.cursor_col = col;
    }

    pub fn page_up(&mut self, height: usize) {
        if self.cursor_row >= height {
            self.cursor_row -= height;
        } else {
            self.cursor_row = 0;
        }
        self.clamp_cursor_col();
        self.scroll_to_cursor(height, 80);
    }

    pub fn page_down(&mut self, height: usize) {
        let new_row = (self.cursor_row + height).min(self.lines.len().saturating_sub(1));
        self.cursor_row = new_row;
        self.clamp_cursor_col();
        self.scroll_to_cursor(height, 80);
    }

    pub fn goto_line(&mut self, n: usize) {
        self.cursor_row = n.min(self.lines.len().saturating_sub(1));
        self.clamp_cursor_col();
    }

    pub fn goto_first_line(&mut self) {
        self.cursor_row = 0;
        self.cursor_col = 0;
    }

    pub fn goto_last_line(&mut self) {
        self.cursor_row = self.lines.len().saturating_sub(1);
        self.clamp_cursor_col();
    }

    pub fn scroll_to_cursor(&mut self, view_height: usize, view_width: usize) {
        if self.cursor_row < self.scroll_row {
            self.scroll_row = self.cursor_row;
        }
        if self.cursor_row >= self.scroll_row + view_height {
            self.scroll_row = self.cursor_row - view_height + 1;
        }
        if self.cursor_col < self.scroll_col {
            self.scroll_col = self.cursor_col;
        }
        if self.cursor_col >= self.scroll_col + view_width {
            self.scroll_col = self.cursor_col - view_width + 1;
        }
    }

    pub fn clamp_cursor_col(&mut self) {
        if self.cursor_row < self.lines.len() {
            let max = self.lines[self.cursor_row].len();
            if self.cursor_col > max {
                self.cursor_col = max;
            }
        }
    }

    pub fn search(&mut self, query: &str) {
        self.search_query = query.to_string();
        self.search_matches.clear();
        if query.is_empty() { return; }
        for (row, line) in self.lines.iter().enumerate() {
            let mut start = 0;
            while let Some(pos) = line[start..].find(query) {
                self.search_matches.push((row, start + pos));
                start += pos + query.len();
                if start >= line.len() { break; }
            }
        }
        self.search_idx = 0;
        if !self.search_matches.is_empty() {
            let (row, col) = self.search_matches[0];
            self.cursor_row = row;
            self.cursor_col = col;
        }
    }

    pub fn search_next(&mut self) {
        if self.search_matches.is_empty() { return; }
        self.search_idx = (self.search_idx + 1) % self.search_matches.len();
        let (row, col) = self.search_matches[self.search_idx];
        self.cursor_row = row;
        self.cursor_col = col;
    }

    pub fn search_prev(&mut self) {
        if self.search_matches.is_empty() { return; }
        if self.search_idx == 0 {
            self.search_idx = self.search_matches.len() - 1;
        } else {
            self.search_idx -= 1;
        }
        let (row, col) = self.search_matches[self.search_idx];
        self.cursor_row = row;
        self.cursor_col = col;
    }

    pub fn push_undo(&mut self) {
        self.undo_stack.push(UndoEntry {
            lines: self.lines.clone(),
            cursor_row: self.cursor_row,
            cursor_col: self.cursor_col,
        });
        if self.undo_stack.len() > 200 {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) {
        if let Some(entry) = self.undo_stack.pop() {
            self.redo_stack.push(UndoEntry {
                lines: self.lines.clone(),
                cursor_row: self.cursor_row,
                cursor_col: self.cursor_col,
            });
            self.lines = entry.lines;
            self.cursor_row = entry.cursor_row;
            self.cursor_col = entry.cursor_col;
            self.modified = true;
        }
    }

    pub fn redo(&mut self) {
        if let Some(entry) = self.redo_stack.pop() {
            self.undo_stack.push(UndoEntry {
                lines: self.lines.clone(),
                cursor_row: self.cursor_row,
                cursor_col: self.cursor_col,
            });
            self.lines = entry.lines;
            self.cursor_row = entry.cursor_row;
            self.cursor_col = entry.cursor_col;
            self.modified = true;
        }
    }

    pub fn delete_line(&mut self) {
        self.push_undo();
        if self.lines.len() > 1 {
            self.lines.remove(self.cursor_row);
            if self.cursor_row >= self.lines.len() {
                self.cursor_row = self.lines.len() - 1;
            }
        } else {
            self.lines[0].clear();
        }
        self.clamp_cursor_col();
        self.modified = true;
    }

    pub fn duplicate_line(&mut self) {
        self.push_undo();
        if self.cursor_row < self.lines.len() {
            let line = self.lines[self.cursor_row].clone();
            self.lines.insert(self.cursor_row + 1, line);
            self.cursor_row += 1;
            self.modified = true;
        }
    }

    pub fn get_highlighted_line(&self, row: usize) -> Vec<(String, Style)> {
        if row >= self.lines.len() { return Vec::new(); }
        let line = &self.lines[row];
        let raw_spans = Highlighter::highlight_line(line, &self.language);
        let chars: Vec<char> = line.chars().collect();

        if raw_spans.is_empty() {
            return vec![(line.clone(), Style::default().fg(ratatui::style::Color::Rgb(210, 210, 210)))];
        }

        let mut result = Vec::new();
        for (start, end, style) in &raw_spans {
            let s = *start;
            let e = (*end).min(chars.len());
            if s < e {
                let text: String = chars[s..e].iter().collect();
                result.push((text, *style));
            }
        }
        result
    }

    pub fn word_count(&self) -> usize {
        self.lines.iter()
            .flat_map(|l| l.split_whitespace())
            .count()
    }

    pub fn char_count(&self) -> usize {
        self.lines.iter().map(|l| l.len()).sum::<usize>() + self.lines.len().saturating_sub(1)
    }
}

fn get_indent(line: &str) -> String {
    line.chars().take_while(|c| *c == ' ' || *c == '\t').collect()
}
