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
        "rs"   => "🦀",
        "py"   => "🐍",
        "js" | "ts" => "📜",
        "html" => "🌐",
        "css"  => "🎨",
        "json" => "📋",
        "toml" | "yaml" | "yml" => "⚙",
        "hl" | "hlpp" => "◈",
        "hs"   => "◆",
        "hk"   => "◎",
        "go"   => "🐹",
        "c" | "cpp" | "h" | "hpp" => "⚡",
        "sh" | "bash" => "❯",
        "md"   => "📝",
        "txt"  => "📄",
        "lua"  => "🌙",
        "nim"  => "👑",
        "kt"   => "◆",
        "java" => "☕",
        "dart" => "🎯",
        "cr"   => "💎",
        "odin" => "◈",
        "vala" => "◆",
        _      => "  ",
    }
}

pub fn stars(rating: f32) -> String {
    let full = rating.floor() as usize;
    let half = if rating - rating.floor() >= 0.5 { 1 } else { 0 };
    let empty = 5 - full - half;
    format!("{}{}{}", "★".repeat(full), if half == 1 { "½" } else { "" }, "☆".repeat(empty))
}
