use ratatui::style::{Color, Modifier, Style};
use crate::languages::Language;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Keyword,
    String,
    Comment,
    Number,
    Operator,
    Function,
    Type,
    Variable,
    Punctuation,
    Normal,
    SpecialHL,   // Hacker Lang operators ~> :: etc
    ConfigKey,   // YAML/TOML/JSON keys
    ConfigValue,
    HtmlTag,
    HtmlAttr,
    CssProperty,
    CssValue,
    HkDirective,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub kind: TokenKind,
}

pub struct Highlighter;

impl Highlighter {
    pub fn highlight_line(line: &str, lang: &Language) -> Vec<(usize, usize, Style)> {
        let mut spans = Vec::new();

        match lang {
            Language::HackerLang | Language::HackerLangPlusPlus => {
                highlight_hacker_lang(line, &mut spans);
            }
            Language::HSharp => {
                highlight_hsharp(line, &mut spans);
            }
            Language::Json => {
                highlight_json(line, &mut spans);
            }
            Language::Yaml => {
                highlight_yaml(line, &mut spans);
            }
            Language::Toml => {
                highlight_toml(line, &mut spans);
            }
            Language::Xml | Language::Html => {
                highlight_xml_html(line, &mut spans);
            }
            Language::Css => {
                highlight_css(line, &mut spans);
            }
            Language::Hk => {
                highlight_hk(line, &mut spans);
            }
            Language::Shell => {
                highlight_shell(line, &mut spans);
            }
            _ => {
                highlight_generic(line, lang, &mut spans);
            }
        }

        spans
    }

    pub fn style_for_kind(kind: &TokenKind) -> Style {
        match kind {
            TokenKind::Keyword     => Style::default().fg(Color::Rgb(200, 120, 255)).add_modifier(Modifier::BOLD),
            TokenKind::String      => Style::default().fg(Color::Rgb(100, 220, 100)),
            TokenKind::Comment     => Style::default().fg(Color::Rgb(100, 110, 130)).add_modifier(Modifier::ITALIC),
            TokenKind::Number      => Style::default().fg(Color::Rgb(255, 180, 80)),
            TokenKind::Operator    => Style::default().fg(Color::Rgb(255, 200, 100)),
            TokenKind::Function    => Style::default().fg(Color::Rgb(80, 180, 255)),
            TokenKind::Type        => Style::default().fg(Color::Rgb(100, 220, 220)),
            TokenKind::Variable    => Style::default().fg(Color::Rgb(220, 220, 220)),
            TokenKind::Punctuation => Style::default().fg(Color::Rgb(180, 180, 180)),
            TokenKind::Normal      => Style::default().fg(Color::Rgb(210, 210, 210)),
            TokenKind::SpecialHL   => Style::default().fg(Color::Rgb(0, 255, 150)).add_modifier(Modifier::BOLD),
            TokenKind::ConfigKey   => Style::default().fg(Color::Rgb(100, 200, 255)),
            TokenKind::ConfigValue => Style::default().fg(Color::Rgb(100, 220, 100)),
            TokenKind::HtmlTag     => Style::default().fg(Color::Rgb(255, 100, 80)),
            TokenKind::HtmlAttr    => Style::default().fg(Color::Rgb(255, 200, 80)),
            TokenKind::CssProperty => Style::default().fg(Color::Rgb(100, 200, 255)),
            TokenKind::CssValue    => Style::default().fg(Color::Rgb(100, 220, 100)),
            TokenKind::HkDirective => Style::default().fg(Color::Rgb(0, 255, 200)).add_modifier(Modifier::BOLD),
        }
    }
}

fn push_span(spans: &mut Vec<(usize, usize, Style)>, start: usize, end: usize, kind: TokenKind) {
    if start < end {
        spans.push((start, end, Highlighter::style_for_kind(&kind)));
    }
}

fn highlight_hacker_lang(line: &str, spans: &mut Vec<(usize, usize, Style)>) {
    let trimmed = line.trim_start();

    // Comments
    if trimmed.starts_with(";;;") || trimmed.starts_with("///") {
        push_span(spans, 0, line.len(), TokenKind::Comment);
        return;
    }
    if trimmed.starts_with(";;") {
        let off = line.len() - trimmed.len();
        push_span(spans, 0, off, TokenKind::Normal);
        push_span(spans, off, line.len(), TokenKind::Comment);
        return;
    }

    let mut i = 0;
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut last_normal = 0usize;

    while i < len {
        // HL special operators  ~>  ::  %  @  ->  ^>  >>  ->>
        if i < len {
            let two: String = chars[i..std::cmp::min(i+3, len)].iter().collect();
            let special = if two.starts_with("~>") { 2 }
                else if two.starts_with("::") { 2 }
                else if two.starts_with("->>") { 3 }
                else if two.starts_with("^>>") { 3 }
                else if two.starts_with("^->") { 3 }
                else if two.starts_with("->") { 2 }
                else if two.starts_with("^>") { 2 }
                else if two.starts_with(">>") { 2 }
                else { 0 };

            if special > 0 {
                if last_normal < i { push_span(spans, last_normal, i, TokenKind::Normal); }
                push_span(spans, i, i + special, TokenKind::SpecialHL);
                i += special;
                last_normal = i;
                continue;
            }
        }

        // @variable
        if chars[i] == '@' {
            if last_normal < i { push_span(spans, last_normal, i, TokenKind::Normal); }
            let start = i;
            i += 1;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            push_span(spans, start, i, TokenKind::Variable);
            last_normal = i;
            continue;
        }

        // %variable declaration
        if chars[i] == '%' {
            if last_normal < i { push_span(spans, last_normal, i, TokenKind::Normal); }
            let start = i;
            i += 1;
            while i < len && chars[i] == ' ' { i += 1; }
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            push_span(spans, start, i, TokenKind::Operator);
            last_normal = i;
            continue;
        }

        // Strings
        if chars[i] == '"' || chars[i] == '\'' {
            let quote = chars[i];
            if last_normal < i { push_span(spans, last_normal, i, TokenKind::Normal); }
            let start = i;
            i += 1;
            while i < len {
                if chars[i] == '\\' { i += 2; continue; }
                if chars[i] == quote { i += 1; break; }
                i += 1;
            }
            push_span(spans, start, i, TokenKind::String);
            last_normal = i;
            continue;
        }

        // Numbers
        if chars[i].is_ascii_digit() {
            if last_normal < i { push_span(spans, last_normal, i, TokenKind::Normal); }
            let start = i;
            while i < len && (chars[i].is_ascii_digit() || chars[i] == '.') { i += 1; }
            push_span(spans, start, i, TokenKind::Number);
            last_normal = i;
            continue;
        }

        // Keywords (def, done, true, false)
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') { i += 1; }
            let word: String = chars[start..i].iter().collect();
            let is_kw = matches!(word.as_str(), "def" | "done" | "true" | "false" | "ok" | "err");
            if last_normal < start { push_span(spans, last_normal, start, TokenKind::Normal); }
            if is_kw {
                push_span(spans, start, i, TokenKind::Keyword);
            } else {
                push_span(spans, start, i, TokenKind::Normal);
            }
            last_normal = i;
            continue;
        }

        i += 1;
    }
    if last_normal < len {
        push_span(spans, last_normal, len, TokenKind::Normal);
    }
}

fn highlight_hsharp(line: &str, spans: &mut Vec<(usize, usize, Style)>) {
    highlight_generic_with_comment(line, "//", &[
        "fn", "let", "mut", "if", "else", "elsif", "while", "for", "return",
        "struct", "impl", "trait", "enum", "match", "use", "mod", "pub",
        "true", "false", "null", "unsafe", "arena", "optional", "spawn",
        "in", "as", "type", "const", "static", "where", "self", "Self",
        "i8","i16","i32","i64","u8","u16","u32","u64","f32","f64","bool","str","String","char",
    ], spans);
}

fn highlight_json(line: &str, spans: &mut Vec<(usize, usize, Style)>) {
    let mut i = 0;
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut last_normal = 0;
    let mut is_key = true;

    while i < len {
        if chars[i] == '"' {
            if last_normal < i { push_span(spans, last_normal, i, TokenKind::Normal); }
            let start = i;
            i += 1;
            while i < len {
                if chars[i] == '\\' { i += 2; continue; }
                if chars[i] == '"' { i += 1; break; }
                i += 1;
            }
            // Determine if it's a key (followed by :) or value
            let mut j = i;
            while j < len && chars[j] == ' ' { j += 1; }
            let kind = if j < len && chars[j] == ':' && is_key {
                is_key = false;
                TokenKind::ConfigKey
            } else {
                is_key = true;
                TokenKind::String
            };
            push_span(spans, start, i, kind);
            last_normal = i;
            continue;
        }
        if chars[i] == ':' || chars[i] == ',' || chars[i] == '{' || chars[i] == '}' || chars[i] == '[' || chars[i] == ']' {
            if chars[i] == ',' || chars[i] == '{' { is_key = true; }
            if last_normal < i { push_span(spans, last_normal, i, TokenKind::Normal); }
            push_span(spans, i, i+1, TokenKind::Punctuation);
            last_normal = i + 1;
            i += 1;
            continue;
        }
        if chars[i].is_ascii_digit() || chars[i] == '-' {
            if last_normal < i { push_span(spans, last_normal, i, TokenKind::Normal); }
            let start = i;
            if chars[i] == '-' { i += 1; }
            while i < len && (chars[i].is_ascii_digit() || chars[i] == '.' || chars[i] == 'e' || chars[i] == 'E') { i += 1; }
            push_span(spans, start, i, TokenKind::Number);
            last_normal = i;
            continue;
        }
        // true/false/null
        if i + 4 <= len {
            let word: String = chars[i..std::cmp::min(i+5, len)].iter().collect();
            if word.starts_with("true") || word.starts_with("null") {
                if last_normal < i { push_span(spans, last_normal, i, TokenKind::Normal); }
                let end = if word.starts_with("null") { i+4 } else { i+4 };
                push_span(spans, i, end, TokenKind::Keyword);
                last_normal = end;
                i = end;
                continue;
            }
            if word.starts_with("false") {
                if last_normal < i { push_span(spans, last_normal, i, TokenKind::Normal); }
                push_span(spans, i, i+5, TokenKind::Keyword);
                last_normal = i+5;
                i = i+5;
                continue;
            }
        }
        i += 1;
    }
    if last_normal < len { push_span(spans, last_normal, len, TokenKind::Normal); }
}

fn highlight_yaml(line: &str, spans: &mut Vec<(usize, usize, Style)>) {
    let trimmed = line.trim_start();
    let offset = line.len() - trimmed.len();

    if trimmed.starts_with('#') {
        if offset > 0 { push_span(spans, 0, offset, TokenKind::Normal); }
        push_span(spans, offset, line.len(), TokenKind::Comment);
        return;
    }
    if trimmed.starts_with("- ") || trimmed.starts_with('-') {
        push_span(spans, 0, offset+1, TokenKind::Operator);
        push_span(spans, offset+1, line.len(), TokenKind::Normal);
        return;
    }
    if let Some(colon) = trimmed.find(": ") {
        let key_end = offset + colon;
        push_span(spans, 0, key_end, TokenKind::ConfigKey);
        push_span(spans, key_end, key_end+2, TokenKind::Punctuation);
        push_span(spans, key_end+2, line.len(), TokenKind::ConfigValue);
    } else if trimmed.ends_with(':') {
        push_span(spans, 0, line.len()-1, TokenKind::ConfigKey);
        push_span(spans, line.len()-1, line.len(), TokenKind::Punctuation);
    } else {
        push_span(spans, 0, line.len(), TokenKind::Normal);
    }
}

fn highlight_toml(line: &str, spans: &mut Vec<(usize, usize, Style)>) {
    let trimmed = line.trim_start();
    let offset = line.len() - trimmed.len();

    if trimmed.starts_with('#') {
        if offset > 0 { push_span(spans, 0, offset, TokenKind::Normal); }
        push_span(spans, offset, line.len(), TokenKind::Comment);
        return;
    }
    if trimmed.starts_with('[') {
        push_span(spans, 0, line.len(), TokenKind::Type);
        return;
    }
    if let Some(eq) = trimmed.find(" = ") {
        let key_end = offset + eq;
        push_span(spans, 0, key_end, TokenKind::ConfigKey);
        push_span(spans, key_end, key_end+3, TokenKind::Punctuation);
        push_span(spans, key_end+3, line.len(), TokenKind::ConfigValue);
    } else {
        push_span(spans, 0, line.len(), TokenKind::Normal);
    }
}

fn highlight_xml_html(line: &str, spans: &mut Vec<(usize, usize, Style)>) {
    let mut i = 0;
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut last_normal = 0;

    while i < len {
        if i + 3 < len && chars[i] == '<' && chars[i+1] == '!' && chars[i+2] == '-' && chars[i+3] == '-' {
            if last_normal < i { push_span(spans, last_normal, i, TokenKind::Normal); }
            let start = i;
            while i < len {
                if i+2 < len && chars[i] == '-' && chars[i+1] == '-' && chars[i+2] == '>' { i += 3; break; }
                i += 1;
            }
            push_span(spans, start, i, TokenKind::Comment);
            last_normal = i;
            continue;
        }
        if chars[i] == '<' {
            if last_normal < i { push_span(spans, last_normal, i, TokenKind::Normal); }
            let start = i;
            i += 1;
            if i < len && chars[i] == '/' { i += 1; }
            while i < len && chars[i] != ' ' && chars[i] != '>' && chars[i] != '/' { i += 1; }
            push_span(spans, start, i, TokenKind::HtmlTag);
            // attributes
            while i < len && chars[i] != '>' {
                while i < len && chars[i] == ' ' { i += 1; }
                if chars[i] == '>' || chars[i] == '/' { break; }
                let attr_start = i;
                while i < len && chars[i] != '=' && chars[i] != ' ' && chars[i] != '>' { i += 1; }
                push_span(spans, attr_start, i, TokenKind::HtmlAttr);
                if i < len && chars[i] == '=' {
                    push_span(spans, i, i+1, TokenKind::Punctuation);
                    i += 1;
                    if i < len && (chars[i] == '"' || chars[i] == '\'') {
                        let q = chars[i];
                        let qs = i;
                        i += 1;
                        while i < len && chars[i] != q { i += 1; }
                        i += 1;
                        push_span(spans, qs, i, TokenKind::String);
                    }
                }
            }
            if i < len { push_span(spans, i, i+1, TokenKind::HtmlTag); i += 1; }
            last_normal = i;
            continue;
        }
        i += 1;
    }
    if last_normal < len { push_span(spans, last_normal, len, TokenKind::Normal); }
}

fn highlight_css(line: &str, spans: &mut Vec<(usize, usize, Style)>) {
    let trimmed = line.trim_start();
    if trimmed.starts_with("/*") || trimmed.starts_with("//") {
        push_span(spans, 0, line.len(), TokenKind::Comment);
        return;
    }
    if trimmed.ends_with('{') || trimmed.ends_with(',') {
        push_span(spans, 0, line.len(), TokenKind::HtmlTag);
        return;
    }
    if let Some(colon) = trimmed.find(": ") {
        let offset = line.len() - trimmed.len();
        let key_end = offset + colon;
        push_span(spans, 0, key_end, TokenKind::CssProperty);
        push_span(spans, key_end, key_end+2, TokenKind::Punctuation);
        push_span(spans, key_end+2, line.len(), TokenKind::CssValue);
    } else {
        push_span(spans, 0, line.len(), TokenKind::Normal);
    }
}

fn highlight_hk(line: &str, spans: &mut Vec<(usize, usize, Style)>) {
    let trimmed = line.trim_start();
    if trimmed.starts_with('#') {
        push_span(spans, 0, line.len(), TokenKind::Comment);
        return;
    }
    // HK directives: [section], key = value
    if trimmed.starts_with('[') {
        push_span(spans, 0, line.len(), TokenKind::HkDirective);
        return;
    }
    if let Some(eq) = trimmed.find(" = ") {
        let offset = line.len() - trimmed.len();
        let key_end = offset + eq;
        push_span(spans, 0, key_end, TokenKind::ConfigKey);
        push_span(spans, key_end, key_end+3, TokenKind::Operator);
        push_span(spans, key_end+3, line.len(), TokenKind::ConfigValue);
    } else {
        push_span(spans, 0, line.len(), TokenKind::Normal);
    }
}

fn highlight_shell(line: &str, spans: &mut Vec<(usize, usize, Style)>) {
    let trimmed = line.trim_start();
    if trimmed.starts_with('#') {
        push_span(spans, 0, line.len(), TokenKind::Comment);
        return;
    }
    highlight_generic_with_comment(line, "#", &[
        "if", "then", "else", "elif", "fi", "for", "do", "done", "while",
        "until", "case", "esac", "function", "return", "export", "local",
        "readonly", "unset", "echo", "exit", "source", "alias",
    ], spans);
}

fn highlight_generic(line: &str, lang: &Language, spans: &mut Vec<(usize, usize, Style)>) {
    let comment = lang.comment_single().unwrap_or("//");
    let keywords = lang.keywords();
    let kw_refs: Vec<&str> = keywords.iter().map(|s| *s).collect();
    highlight_generic_with_comment(line, comment, &kw_refs, spans);
}

fn highlight_generic_with_comment(
    line: &str,
    comment_prefix: &str,
    keywords: &[&str],
    spans: &mut Vec<(usize, usize, Style)>
) {
    // Check for full-line comment
    let trimmed = line.trim_start();
    if trimmed.starts_with(comment_prefix) {
        let offset = line.len() - trimmed.len();
        if offset > 0 { push_span(spans, 0, offset, TokenKind::Normal); }
        push_span(spans, offset, line.len(), TokenKind::Comment);
        return;
    }

    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut last_normal = 0;

    while i < len {
        // Inline comment
        let remaining: String = chars[i..].iter().collect();
        if remaining.starts_with(comment_prefix) {
            if last_normal < i { push_span(spans, last_normal, i, TokenKind::Normal); }
            push_span(spans, i, len, TokenKind::Comment);
            return;
        }

        // String
        if chars[i] == '"' || chars[i] == '\'' || chars[i] == '`' {
            let quote = chars[i];
            if last_normal < i { push_span(spans, last_normal, i, TokenKind::Normal); }
            let start = i;
            i += 1;
            while i < len {
                if chars[i] == '\\' { i += 2; continue; }
                if chars[i] == quote { i += 1; break; }
                i += 1;
            }
            push_span(spans, start, i, TokenKind::String);
            last_normal = i;
            continue;
        }

        // Number
        if chars[i].is_ascii_digit() {
            if last_normal < i { push_span(spans, last_normal, i, TokenKind::Normal); }
            let start = i;
            while i < len && (chars[i].is_ascii_digit() || chars[i] == '.' || chars[i] == 'x' ||
                chars[i] == 'b' || chars[i] == 'o' || (chars[i] >= 'a' && chars[i] <= 'f') ||
                (chars[i] >= 'A' && chars[i] <= 'F') || chars[i] == '_') { i += 1; }
            push_span(spans, start, i, TokenKind::Number);
            last_normal = i;
            continue;
        }

        // Identifier / keyword
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') { i += 1; }
            let word: String = chars[start..i].iter().collect();
            if last_normal < start { push_span(spans, last_normal, start, TokenKind::Normal); }

            // Check uppercase = type
            let is_type = word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
                && word.len() > 1;
            let is_kw = keywords.contains(&word.as_str());

            // Check if next non-space char is ( = function call
            let mut j = i;
            while j < len && chars[j] == ' ' { j += 1; }
            let is_func = j < len && chars[j] == '(';

            let kind = if is_kw { TokenKind::Keyword }
                else if is_type { TokenKind::Type }
                else if is_func { TokenKind::Function }
                else { TokenKind::Normal };

            push_span(spans, start, i, kind);
            last_normal = i;
            continue;
        }

        // Operators
        if "+-*/%=<>!&|^~".contains(chars[i]) {
            if last_normal < i { push_span(spans, last_normal, i, TokenKind::Normal); }
            push_span(spans, i, i+1, TokenKind::Operator);
            last_normal = i + 1;
            i += 1;
            continue;
        }

        i += 1;
    }

    if last_normal < len {
        push_span(spans, last_normal, len, TokenKind::Normal);
    }
}
