use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone)]
pub struct FileNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub depth: usize,
    pub expanded: bool,
    pub children: Vec<FileNode>,
}

impl FileNode {
    pub fn new(path: PathBuf, depth: usize) -> Self {
        let name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("?")
        .to_string();
        let is_dir = path.is_dir();
        Self { path, name, is_dir, depth, expanded: false, children: Vec::new() }
    }

    pub fn load_children(&mut self) {
        if !self.is_dir { return; }
        self.children.clear();
        if let Ok(entries) = fs::read_dir(&self.path) {
            let mut dirs = Vec::new();
            let mut files = Vec::new();
            for entry in entries.flatten() {
                let p = entry.path();
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
                if name.starts_with('.') { continue; }
                if p.is_dir() {
                    dirs.push(FileNode::new(p, self.depth + 1));
                } else {
                    files.push(FileNode::new(p, self.depth + 1));
                }
            }
            dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            self.children.extend(dirs);
            self.children.extend(files);
        }
    }

    pub fn collect_visible<'a>(&'a self, acc: &mut Vec<&'a FileNode>) {
        acc.push(self);
        if self.is_dir && self.expanded {
            for child in &self.children {
                child.collect_visible(acc);
            }
        }
    }

    pub fn collect_visible_mut<'a>(&'a mut self, acc: &mut Vec<*mut FileNode>) {
        acc.push(self as *mut FileNode);
        if self.is_dir && self.expanded {
            for child in &mut self.children {
                child.collect_visible_mut(acc);
            }
        }
    }
}

pub struct FileTree {
    pub root: Option<PathBuf>,
    pub nodes: Vec<FileNode>,
    pub selected: usize,
}

impl FileTree {
    pub fn new() -> Self {
        Self { root: None, nodes: Vec::new(), selected: 0 }
    }

    pub fn load(&mut self, path: &Path) {
        self.root = Some(path.to_path_buf());
        self.nodes.clear();
        let mut root = FileNode::new(path.to_path_buf(), 0);
        root.load_children();
        root.expanded = true;
        self.nodes.push(root);
        self.selected = 0;
    }

    pub fn refresh(&mut self) {
        if let Some(root) = self.root.clone() {
            self.load(&root);
        }
    }

    pub fn visible_count(&self) -> usize {
        let mut acc = Vec::new();
        for n in &self.nodes { n.collect_visible(&mut acc); }
        acc.len()
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 { self.selected -= 1; }
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.visible_count() { self.selected += 1; }
    }

    pub fn toggle_expand(&mut self) {
        let selected = self.selected;
        // Collect raw pointers to avoid borrow conflict
        let mut ptrs: Vec<*mut FileNode> = Vec::new();
        for n in &mut self.nodes { n.collect_visible_mut(&mut ptrs); }
        if selected < ptrs.len() {
            // SAFETY: we hold &mut self, no aliasing
            let node = unsafe { &mut *ptrs[selected] };
            if node.is_dir {
                let was_expanded = node.expanded;
                if !was_expanded && node.children.is_empty() {
                    node.load_children();
                }
                node.expanded = !was_expanded;
            }
        }
    }

    pub fn selected_path(&self) -> Option<PathBuf> {
        let mut acc = Vec::new();
        for n in &self.nodes { n.collect_visible(&mut acc); }
        acc.get(self.selected).map(|n| n.path.clone())
    }

    pub fn selected_is_dir(&self) -> bool {
        let mut acc = Vec::new();
        for n in &self.nodes { n.collect_visible(&mut acc); }
        acc.get(self.selected).map(|n| n.is_dir).unwrap_or(false)
    }

    pub fn visible_nodes(&self) -> Vec<&FileNode> {
        let mut acc = Vec::new();
        for n in &self.nodes { n.collect_visible(&mut acc); }
        acc
    }

    pub fn create_file(&mut self, name: &str) -> anyhow::Result<PathBuf> {
        let base = if let Some(p) = self.selected_path() {
            if p.is_dir() { p } else { p.parent().unwrap_or(Path::new(".")).to_path_buf() }
        } else {
            self.root.clone().unwrap_or_else(|| PathBuf::from("."))
        };
        let new_path = base.join(name);
        fs::File::create(&new_path)?;
        self.refresh();
        Ok(new_path)
    }

    pub fn delete_selected(&mut self) -> anyhow::Result<()> {
        if let Some(path) = self.selected_path() {
            if path.is_dir() { fs::remove_dir_all(&path)?; }
            else { fs::remove_file(&path)?; }
            if self.selected > 0 { self.selected -= 1; }
            self.refresh();
        }
        Ok(())
    }
}

impl FileTree {
    /// Zwróć katalog aktualnie wybranego elementu
    pub fn selected_dir_path(&self) -> Option<std::path::PathBuf> {
        let path = self.selected_path()?;
        if path.is_dir() {
            Some(path)
        } else {
            path.parent().map(|p| p.to_path_buf())
        }
    }
}
