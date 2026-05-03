use std::path::Path;

pub fn truncate_str(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max.saturating_sub(1)])
    }
}

pub fn center_text(text: &str, width: usize) -> String {
    if text.len() >= width {
        return text.to_string();
    }
    let pad = (width - text.len()) / 2;
    format!("{}{}", " ".repeat(pad), text)
}

pub fn format_file_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1}KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1}MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

pub fn file_icon(path: &Path) -> &'static str {
    if path.is_dir() { return "▸ "; }
    match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "rs"   => "rs",
        "py"   => "py",
        "js" | "ts" => "js",
        "html" => "ht",
        "css"  => "cs",
        "json" => "jn",
        "toml" | "yaml" | "yml" => "cf",
        "hl" | "hlpp" => "hl",
        "hs"   => "kt",
        "hk"   => "hk",
        "go"   => "go",
        "c" | "cpp" | "h" | "hpp" => "c ",
        "sh" | "bash" => ">>",
        "md"   => "md",
        "txt"  => "tx",
        "lua"  => "lu",
        "nim"  => "ni",
        "kt"   => "kt",
        "java" => "jv",
        "dart" => "dt",
        "cr"   => "cr",
        "odin" => "hl",
        "vala" => "kt",
        _      => "  ",
    }
}

pub fn stars(rating: f32) -> String {
    let full = rating.floor() as usize;
    let empty = 5 - full.min(5);
    format!("{}{}",  "*".repeat(full), ".".repeat(empty))
}
