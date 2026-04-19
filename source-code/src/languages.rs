use ratatui::style::Color;

#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    HackerLang,
    HackerLangPlusPlus,
    HSharp,
    C,
    Cpp,
    Python,
    Go,
    Odin,
    Crystal,
    Shell,
    Vala,
    Nim,
    Html,
    Css,
    JavaScript,
    TypeScript,
    Dart,
    Kotlin,
    Lua,
    Rust,
    Java,
    Yaml,
    Json,
    Toml,
    Hcl,
    Xml,
    Hk,
    PlainText,
}

impl Language {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "hl"   => Language::HackerLang,
            "hlpp" | "hl++" => Language::HackerLangPlusPlus,
            "hs"   => Language::HSharp,
            "c"    => Language::C,
            "cpp" | "cc" | "cxx" | "h" | "hpp" => Language::Cpp,
            "py"   => Language::Python,
            "go"   => Language::Go,
            "odin" => Language::Odin,
            "cr"   => Language::Crystal,
            "sh" | "bash" | "zsh" => Language::Shell,
            "vala" => Language::Vala,
            "nim"  => Language::Nim,
            "html" | "htm" => Language::Html,
            "css"  => Language::Css,
            "js"   => Language::JavaScript,
            "ts"   => Language::TypeScript,
            "dart" => Language::Dart,
            "kt" | "kts" => Language::Kotlin,
            "lua"  => Language::Lua,
            "rs"   => Language::Rust,
            "java" => Language::Java,
            "yaml" | "yml" => Language::Yaml,
            "json" => Language::Json,
            "toml" => Language::Toml,
            "hcl" | "tf" => Language::Hcl,
            "xml"  => Language::Xml,
            "hk"   => Language::Hk,
            _ => Language::PlainText,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::HackerLang         => "Hacker Lang",
            Language::HackerLangPlusPlus => "Hacker Lang++ (BETA)",
            Language::HSharp             => "H#",
            Language::C                  => "C",
            Language::Cpp                => "C++",
            Language::Python             => "Python",
            Language::Go                 => "Go",
            Language::Odin               => "Odin",
            Language::Crystal            => "Crystal",
            Language::Shell              => "Shell",
            Language::Vala               => "Vala",
            Language::Nim                => "Nim",
            Language::Html               => "HTML",
            Language::Css                => "CSS",
            Language::JavaScript         => "JavaScript",
            Language::TypeScript         => "TypeScript",
            Language::Dart               => "Dart",
            Language::Kotlin             => "Kotlin",
            Language::Lua                => "Lua",
            Language::Rust               => "Rust",
            Language::Java               => "Java",
            Language::Yaml               => "YAML",
            Language::Json               => "JSON",
            Language::Toml               => "TOML",
            Language::Hcl                => "HCL",
            Language::Xml                => "XML",
            Language::Hk                 => "HK Plugin",
            Language::PlainText          => "Plain Text",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Language::HackerLang         => "HL",
            Language::HackerLangPlusPlus => "HL+",
            Language::HSharp             => "H#",
            Language::C                  => "C ",
            Language::Cpp                => "C+",
            Language::Python             => "PY",
            Language::Go                 => "GO",
            Language::Odin               => "OD",
            Language::Crystal            => "CR",
            Language::Shell              => "SH",
            Language::Vala               => "VL",
            Language::Nim                => "NM",
            Language::Html               => "HT",
            Language::Css                => "CS",
            Language::JavaScript         => "JS",
            Language::TypeScript         => "TS",
            Language::Dart               => "DT",
            Language::Kotlin             => "KT",
            Language::Lua                => "LU",
            Language::Rust               => "RS",
            Language::Java               => "JV",
            Language::Yaml               => "YM",
            Language::Json               => "JN",
            Language::Toml               => "TM",
            Language::Hcl                => "HC",
            Language::Xml                => "XM",
            Language::Hk                 => "HK",
            Language::PlainText          => "TX",
        }
    }

    pub fn icon_color(&self) -> Color {
        match self {
            Language::HackerLang         => Color::Rgb(0, 255, 100),
            Language::HackerLangPlusPlus => Color::Rgb(0, 220, 80),
            Language::HSharp             => Color::Rgb(80, 200, 255),
            Language::C                  => Color::Rgb(100, 150, 255),
            Language::Cpp                => Color::Rgb(100, 130, 255),
            Language::Python             => Color::Rgb(255, 213, 50),
            Language::Go                 => Color::Rgb(0, 172, 215),
            Language::Odin               => Color::Rgb(180, 90, 255),
            Language::Crystal            => Color::Rgb(0, 220, 200),
            Language::Shell              => Color::Rgb(100, 255, 100),
            Language::Vala               => Color::Rgb(180, 100, 200),
            Language::Nim                => Color::Rgb(255, 216, 0),
            Language::Html               => Color::Rgb(255, 100, 50),
            Language::Css                => Color::Rgb(50, 150, 255),
            Language::JavaScript         => Color::Rgb(255, 220, 0),
            Language::TypeScript         => Color::Rgb(0, 120, 215),
            Language::Dart               => Color::Rgb(0, 180, 216),
            Language::Kotlin             => Color::Rgb(180, 80, 255),
            Language::Lua                => Color::Rgb(50, 100, 200),
            Language::Rust               => Color::Rgb(255, 100, 50),
            Language::Java               => Color::Rgb(255, 160, 50),
            Language::Yaml               => Color::Rgb(200, 100, 50),
            Language::Json               => Color::Rgb(200, 200, 50),
            Language::Toml               => Color::Rgb(150, 200, 50),
            Language::Hcl                => Color::Rgb(100, 100, 220),
            Language::Xml                => Color::Rgb(180, 180, 100),
            Language::Hk                 => Color::Rgb(0, 255, 200),
            Language::PlainText          => Color::Gray,
        }
    }

    pub fn keywords(&self) -> Vec<&'static str> {
        match self {
            Language::HackerLang | Language::HackerLangPlusPlus => vec![
                "def", "done", "true", "false",
                "~>", ">", "^>", "->", "^->", ">>", "^>>", "->>",
                "::", "//", "///", ";;" , "? ok", "? err",
                "--", ":", "%", "@",
            ],
            Language::HSharp => vec![
                "fn", "let", "mut", "if", "else", "elsif", "while", "for",
                "return", "struct", "impl", "trait", "enum", "match",
                "use", "mod", "pub", "true", "false", "null",
                "unsafe", "arena", "optional", "spawn", "in", "as",
                "type", "const", "static", "where", "self", "Self",
            ],
            Language::Rust => vec![
                "fn", "let", "mut", "if", "else", "while", "for", "loop",
                "return", "struct", "impl", "trait", "enum", "match",
                "use", "mod", "pub", "crate", "super", "self", "Self",
                "true", "false", "const", "static", "type", "where",
                "unsafe", "async", "await", "dyn", "move", "ref",
                "in", "as", "break", "continue",
            ],
            Language::Python => vec![
                "def", "class", "if", "elif", "else", "while", "for",
                "return", "import", "from", "as", "with", "pass",
                "True", "False", "None", "not", "and", "or", "in",
                "is", "lambda", "yield", "async", "await", "try",
                "except", "finally", "raise", "del", "global", "nonlocal",
            ],
            Language::Go => vec![
                "func", "var", "const", "type", "struct", "interface",
                "map", "chan", "if", "else", "for", "range", "return",
                "import", "package", "go", "defer", "select", "switch",
                "case", "default", "break", "continue", "fallthrough",
                "goto", "nil", "true", "false", "make", "new", "len", "cap",
            ],
            Language::C | Language::Cpp => vec![
                "int", "char", "float", "double", "void", "long", "short",
                "unsigned", "signed", "static", "const", "extern", "register",
                "if", "else", "while", "for", "do", "switch", "case", "default",
                "return", "break", "continue", "goto", "struct", "union", "enum",
                "typedef", "sizeof", "include", "define", "ifdef", "endif",
                "class", "public", "private", "protected", "virtual", "override",
                "namespace", "template", "typename", "auto", "nullptr", "true", "false",
            ],
            Language::JavaScript | Language::TypeScript => vec![
                "const", "let", "var", "function", "class", "if", "else",
                "while", "for", "of", "in", "return", "import", "export",
                "default", "new", "delete", "typeof", "instanceof",
                "true", "false", "null", "undefined", "async", "await",
                "try", "catch", "finally", "throw", "switch", "case",
                "break", "continue", "extends", "implements", "interface",
                "type", "enum", "namespace", "from",
            ],
            Language::Java => vec![
                "public", "private", "protected", "static", "final", "abstract",
                "class", "interface", "extends", "implements", "new", "return",
                "if", "else", "while", "for", "do", "switch", "case", "default",
                "break", "continue", "try", "catch", "finally", "throw", "throws",
                "import", "package", "void", "null", "true", "false",
                "int", "long", "short", "byte", "char", "float", "double", "boolean",
            ],
            Language::Kotlin => vec![
                "fun", "val", "var", "class", "object", "interface", "if", "else",
                "when", "while", "for", "return", "import", "package",
                "null", "true", "false", "is", "as", "in", "out",
                "override", "open", "sealed", "data", "companion", "by",
                "init", "constructor", "this", "super", "it",
            ],
            Language::Lua => vec![
                "and", "break", "do", "else", "elseif", "end", "false",
                "for", "function", "goto", "if", "in", "local", "nil",
                "not", "or", "repeat", "return", "then", "true", "until", "while",
            ],
            Language::Shell => vec![
                "if", "then", "else", "elif", "fi", "for", "do", "done",
                "while", "until", "case", "esac", "function", "return",
                "export", "local", "readonly", "unset", "shift", "echo",
                "exit", "source", "alias", "cd", "ls", "grep", "awk", "sed",
            ],
            Language::Nim => vec![
                "proc", "func", "method", "iterator", "macro", "template",
                "type", "var", "let", "const", "if", "elif", "else",
                "when", "while", "for", "in", "do", "return", "yield",
                "import", "from", "export", "include", "of", "as",
                "nil", "true", "false", "object", "enum", "tuple",
            ],
            Language::Crystal => vec![
                "def", "class", "module", "struct", "lib", "if", "elsif",
                "else", "unless", "while", "until", "for", "in", "do",
                "return", "yield", "require", "include", "extend", "macro",
                "nil", "true", "false", "end", "then", "begin", "rescue",
                "ensure", "raise", "abstract", "private", "protected",
            ],
            Language::Dart => vec![
                "var", "final", "const", "late", "void", "null",
                "true", "false", "if", "else", "while", "for", "do",
                "switch", "case", "default", "return", "break", "continue",
                "class", "extends", "implements", "mixin", "abstract",
                "import", "export", "library", "part", "async", "await",
                "try", "catch", "finally", "throw", "new", "this", "super",
            ],
            Language::Vala => vec![
                "class", "struct", "interface", "enum", "namespace", "using",
                "public", "private", "protected", "static", "abstract", "virtual",
                "override", "new", "delete", "null", "true", "false",
                "if", "else", "while", "for", "foreach", "in", "do",
                "switch", "case", "default", "return", "break", "continue",
                "var", "const", "string", "int", "bool", "void", "signal",
            ],
            Language::Odin => vec![
                "proc", "struct", "union", "enum", "bit_field", "if", "else",
                "when", "for", "in", "switch", "case", "return", "import",
                "package", "foreign", "using", "defer", "new", "free",
                "make", "delete", "nil", "true", "false", "do", "break", "continue",
                "fallthrough", "map", "dynamic", "auto_cast", "cast", "transmute",
            ],
            Language::Hcl => vec![
                "resource", "data", "variable", "output", "locals", "module",
                "provider", "terraform", "required_providers", "backend",
                "true", "false", "null", "for", "if", "in",
            ],
            _ => vec![],
        }
    }

    pub fn comment_single(&self) -> Option<&'static str> {
        match self {
            Language::HackerLang | Language::HackerLangPlusPlus => Some(";;"),
            Language::HSharp | Language::Rust | Language::Go | Language::C |
            Language::Cpp | Language::JavaScript | Language::TypeScript |
            Language::Java | Language::Kotlin | Language::Dart | Language::Vala |
            Language::Odin | Language::Crystal => Some("//"),
            Language::Python | Language::Shell | Language::Yaml | Language::Toml |
            Language::Nim => Some("#"),
            Language::Lua => Some("--"),
            Language::Json | Language::Xml | Language::Hcl => None,
            _ => Some("//"),
        }
    }

    pub fn string_delimiters(&self) -> Vec<char> {
        vec!['"', '\'', '`']
    }
}

// workaround for missing Ruby variant - Shell covers it
impl Language {
    pub fn extra_kw_color(&self) -> Color {
        match self {
            Language::HackerLang | Language::HackerLangPlusPlus => Color::Rgb(0, 255, 100),
            Language::HSharp => Color::Rgb(80, 200, 255),
            Language::Rust => Color::Rgb(255, 100, 50),
            Language::Python => Color::Rgb(255, 213, 50),
            Language::Go => Color::Rgb(0, 172, 215),
            _ => Color::Rgb(200, 150, 255),
        }
    }
}
