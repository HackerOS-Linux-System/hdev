use crate::languages::Language;

/// Jedna pozycja na liście podpowiedzi
#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label:   String,      // co się wstawia
    pub detail:  String,      // opis po prawej stronie
    pub kind:    CompKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompKind {
    Keyword,
    Snippet,
    Variable,
    Function,
    Type,
    Property,
}

impl CompKind {
    pub fn icon(&self) -> &'static str {
        match self {
            CompKind::Keyword  => "kw",
            CompKind::Snippet  => "sn",
            CompKind::Variable => "vr",
            CompKind::Function => "fn",
            CompKind::Type     => "ty",
            CompKind::Property => ". ",
        }
    }
    pub fn color(&self) -> ratatui::style::Color {
        use ratatui::style::Color;
        match self {
            CompKind::Keyword  => Color::Rgb(200, 120, 255),
            CompKind::Snippet  => Color::Rgb(0, 220, 150),
            CompKind::Variable => Color::Rgb(210, 210, 210),
            CompKind::Function => Color::Rgb(80, 180, 255),
            CompKind::Type     => Color::Rgb(100, 220, 220),
            CompKind::Property => Color::Rgb(180, 180, 180),
        }
    }
}

pub struct Autocomplete {
    pub enabled:  bool,
    pub items:    Vec<CompletionItem>,
    pub selected: usize,
    pub visible:  bool,
    /// Słowo które aktualnie wpisujemy (prefix)
    pub prefix:   String,
}

impl Autocomplete {
    pub fn new() -> Self {
        Self {
            enabled:  true,
            items:    Vec::new(),
            selected: 0,
            visible:  false,
            prefix:   String::new(),
        }
    }

    /// Wygeneruj podpowiedzi na podstawie aktualnej linii i języka
    pub fn update(&mut self, line: &str, cursor_col: usize, lang: &Language, all_words: &[String]) {
        if !self.enabled {
            self.visible = false;
            return;
        }

        // Wyodrębnij słowo przed kursorem
        let before = &line[..cursor_col.min(line.len())];
        let prefix: String = before.chars()
        .rev()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>()
        .chars()
        .rev()
        .collect();

        if prefix.len() < 1 {
            self.visible = false;
            self.items.clear();
            self.prefix.clear();
            return;
        }

        self.prefix = prefix.clone();
        self.items.clear();

        let p = prefix.to_lowercase();

        // 1. Słowa kluczowe języka
        for kw in lang.keywords() {
            if kw.to_lowercase().starts_with(&p) && kw != prefix.as_str() {
                self.items.push(CompletionItem {
                    label:  kw.to_string(),
                                detail: format!("{} keyword", lang.display_name()),
                                kind:   CompKind::Keyword,
                });
            }
        }

        // 2. Snippety specyficzne dla języka
        for (trigger, _body, desc) in lang_snippets(lang) {
            if trigger.to_lowercase().starts_with(&p) && trigger != prefix.as_str() {
                self.items.push(CompletionItem {
                    label:  trigger.to_string(),
                                detail: desc.to_string(),
                                kind:   CompKind::Snippet,
                });
            }
        }

        // 3. Słowa z otwartego pliku (zmienne, funkcje itp.)
        let mut seen = std::collections::HashSet::new();
        for word in all_words {
            if word.to_lowercase().starts_with(&p)
                && word != &prefix
                && word.len() > prefix.len()
                && !seen.contains(word.as_str())
                {
                    seen.insert(word.clone());
                    let kind = if word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                        CompKind::Type
                    } else {
                        CompKind::Variable
                    };
                    self.items.push(CompletionItem {
                        label:  word.clone(),
                                    detail: "w pliku".to_string(),
                                    kind,
                    });
                }
        }

        // Ogranicz do 10 pozycji
        self.items.truncate(10);
        self.selected = 0;
        self.visible  = !self.items.is_empty();
    }

    pub fn close(&mut self) {
        self.visible = false;
        self.items.clear();
        self.prefix.clear();
    }

    pub fn select_next(&mut self) {
        if !self.items.is_empty() {
            self.selected = (self.selected + 1) % self.items.len();
        }
    }

    pub fn select_prev(&mut self) {
        if !self.items.is_empty() {
            if self.selected == 0 { self.selected = self.items.len() - 1; }
            else { self.selected -= 1; }
        }
    }

    /// Zwróć tekst do wstawienia (podmień prefix)
    pub fn accept(&mut self) -> Option<String> {
        if !self.visible || self.items.is_empty() { return None; }
        let item = &self.items[self.selected];
        // Zwracamy tylko to co dodajemy ponad prefix
        let suffix = item.label[self.prefix.len()..].to_string();
        self.close();
        Some(suffix)
    }
}

/// Wyodrębnij wszystkie słowa z buforów (dla podpowiedzi z pliku)
pub fn extract_words(lines: &[String]) -> Vec<String> {
    let mut words = std::collections::HashSet::new();
    for line in lines {
        for word in line.split(|c: char| !c.is_alphanumeric() && c != '_') {
            let w = word.trim();
            if w.len() >= 2 {
                words.insert(w.to_string());
            }
        }
    }
    let mut v: Vec<String> = words.into_iter().collect();
    v.sort();
    v
}

/// Snippety dla każdego języka
fn lang_snippets(lang: &Language) -> Vec<(&'static str, &'static str, &'static str)> {
    match lang {
        Language::Rust => vec![
            ("fn",       "fn name() {\n    \n}",                      "Funkcja"),
            ("struct",   "struct Name {\n    \n}",                    "Struktura"),
            ("impl",     "impl Name {\n    \n}",                      "Implementacja"),
            ("enum",     "enum Name {\n    \n}",                      "Enum"),
            ("match",    "match expr {\n    _ => {}\n}",              "Match"),
            ("if",       "if condition {\n    \n}",                   "If"),
            ("for",      "for item in iter {\n    \n}",               "For"),
            ("while",    "while condition {\n    \n}",                "While"),
            ("println",  "println!(\"{}\", );",                       "Println"),
            ("vec",      "vec![]",                                     "Vec macro"),
            ("let",      "let name = ;",                              "Let binding"),
            ("use",      "use crate::;",                              "Use"),
            ("derive",   "#[derive(Debug, Clone)]",                   "Derive attr"),
            ("todo",     "todo!()",                                    "Todo macro"),
            ("unwrap",   ".unwrap()",                                  "Unwrap"),
        ],
        Language::Python => vec![
            ("def",      "def name():\n    pass",                     "Funkcja"),
            ("class",    "class Name:\n    pass",                     "Klasa"),
            ("if",       "if condition:\n    pass",                   "If"),
            ("for",      "for item in iterable:\n    pass",           "For"),
            ("while",    "while condition:\n    pass",                "While"),
            ("import",   "import module",                             "Import"),
            ("from",     "from module import name",                   "From import"),
            ("print",    "print()",                                    "Print"),
            ("with",     "with open('file') as f:\n    pass",        "With"),
            ("try",      "try:\n    pass\nexcept Exception as e:\n    pass", "Try/except"),
            ("lambda",   "lambda x: x",                               "Lambda"),
            ("list",     "[]",                                         "Lista"),
            ("dict",     "{}",                                         "Słownik"),
        ],
        Language::HackerLang | Language::HackerLangPlusPlus => vec![
            ("def",      ": nazwa def\n    ~> \ndone",                "Definicja bloku"),
            ("curl",     "// curl",                                   "Import curl"),
            ("var",      "% nazwa = wartość",                         "Zmienna"),
            ("ok",       "? ok\n    \ndone",                          "Handler ok"),
            ("err",      "? err\n    \ndone",                         "Handler err"),
            ("print",    "~> ",                                       "Wypisz"),
            ("run",      "> komenda",                                  "Uruchom cmd"),
        ],
        Language::HSharp => vec![
            ("fn",       "fn name() {\n    \n}",                      "Funkcja"),
            ("struct",   "struct Name {\n    \n}",                    "Struktura"),
            ("let",      "let name = ;",                              "Zmienna"),
            ("if",       "if condition {\n    \n}",                   "If"),
            ("match",    "match expr {\n    _ => {}\n}",              "Match"),
            ("for",      "for item in iter {\n    \n}",               "For"),
        ],
        Language::Go => vec![
            ("func",     "func name() {\n\t\n}",                      "Funkcja"),
            ("if",       "if condition {\n\t\n}",                     "If"),
            ("for",      "for i := 0; i < n; i++ {\n\t\n}",          "For"),
            ("struct",   "type Name struct {\n\t\n}",                 "Struct"),
            ("fmt",      "fmt.Println()",                              "Println"),
            ("err",      "if err != nil {\n\treturn err\n}",          "Error check"),
            ("goroutine","go func() {\n\t\n}()",                      "Goroutine"),
        ],
        Language::JavaScript | Language::TypeScript => vec![
            ("fn",       "function name() {\n    \n}",                "Funkcja"),
            ("arrow",    "const name = () => {\n    \n};",            "Arrow fn"),
            ("if",       "if (condition) {\n    \n}",                 "If"),
            ("for",      "for (let i = 0; i < n; i++) {\n    \n}",   "For"),
            ("forEach",  ".forEach(item => {\n    \n});",             "forEach"),
            ("const",    "const name = ;",                            "Const"),
            ("let",      "let name = ;",                              "Let"),
            ("console",  "console.log()",                             "Console log"),
            ("async",    "async function name() {\n    \n}",          "Async fn"),
            ("await",    "await ",                                     "Await"),
            ("import",   "import { } from '';",                       "Import"),
        ],
        Language::Html => vec![
            ("div",      "<div>\n    \n</div>",                       "Div"),
            ("span",     "<span></span>",                             "Span"),
            ("p",        "<p></p>",                                    "Paragraph"),
            ("a",        "<a href=\"\"></a>",                         "Link"),
            ("img",      "<img src=\"\" alt=\"\" />",                 "Image"),
            ("input",    "<input type=\"text\" />",                   "Input"),
            ("button",   "<button></button>",                         "Button"),
            ("ul",       "<ul>\n    <li></li>\n</ul>",               "Lista"),
            ("form",     "<form>\n    \n</form>",                     "Formularz"),
        ],
        Language::Css => vec![
            ("flex",     "display: flex;\nalign-items: center;\njustify-content: center;", "Flexbox"),
            ("grid",     "display: grid;\ngrid-template-columns: repeat(auto-fill, minmax(200px, 1fr));", "Grid"),
            ("media",    "@media (max-width: 768px) {\n    \n}",      "Media query"),
            ("var",      "var(--name)",                               "CSS zmienna"),
            ("anim",     "@keyframes name {\n    from { }\n    to { }\n}", "Animacja"),
        ],
        Language::Shell => vec![
            ("if",       "if [ condition ]; then\n    \nfi",          "If"),
            ("for",      "for item in list; do\n    \ndone",          "For"),
            ("while",    "while [ condition ]; do\n    \ndone",       "While"),
            ("fn",       "function name() {\n    \n}",                "Funkcja"),
            ("case",     "case \"$var\" in\n    val) ;;\nesac",       "Case"),
        ],
        Language::Lua => vec![
            ("fn",       "function name()\n    \nend",                "Funkcja"),
            ("if",       "if condition then\n    \nend",              "If"),
            ("for",      "for i = 1, n do\n    \nend",                "For"),
            ("while",    "while condition do\n    \nend",             "While"),
            ("local",    "local name = ",                             "Local"),
        ],
        _ => vec![],
    }
}
