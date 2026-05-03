# hdev

**TUI Code Editor dla HackerOS** — edytor kodu działający w terminalu, inspirowany Visual Studio Code.

Napisany w **Rust** z biblioteką **Ratatui**.

```
 hdev  v0.1.0  |  main.rs  |  Rust
 RS main.rs *   PY script.py   SH run.sh
 ┌──────────────────┐
 │ ▾ my-project     │   1   fn main() {
 │   ▾ src          │   2       println!("HackerOS");
 │     rs main.rs * │   3   _
 │     rs lib.rs    │
 └──────────────────┘
 NORMAL  main.rs *   Rust         Ln 3, Col 1
```

---

## Funkcje

- **Podswietlanie skladni** dla 28 jezykow i formatow
- **Autocomplete z Tab** — slowa kluczowe, snippety, slowa z pliku
- **Zintegrowany terminal** z obsługa `hsh` (shell HackerOS) lub `sh`
- **Drzewo plikow** z nawigacja klawiaturowa
- **Marketplace** — pobieranie pluginow `.hk` z JSON-owego repozytorium
- **Zakladki** (tabs) dla wielu otwartych plikow
- **Undo/Redo** do 200 krokow
- **Wyszukiwanie** z podswietlaniem wszystkich wynikow
- **Sesja** — zapamietuje otwarte pliki i histori komend terminala
- **Ustawienia** zapisywane w `~/.cache/HackerOS/hdev/config.json`

---

## Instalacja

### Wymagania

| Zaleznosc | Min. wersja |
|-----------|-------------|
| Rust      | 1.75        |
| Cargo     | 1.75        |

Rust instalujesz przez [rustup.rs](https://rustup.rs).

### Budowanie

```sh
git clone https://github.com/HackerOS-Linux-System/hdev
cd hdev
cargo build --release
sudo cp target/release/hdev /usr/local/bin/
```

### Uruchamianie

```sh
hdev                        # ekran powitalny
hdev src/main.rs            # otworz konkretny plik
hdev ~/projects/moj-projekt # otworz folder
```

---

## Skroty klawiszowe

### Globalne

| Skrot       | Akcja                        |
|-------------|------------------------------|
| `Ctrl+Q`    | Wyjdz z hdev                 |
| `Ctrl+H`    | Pomoc — lista skrotow        |
| `Esc`       | Zamknij panel / anuluj       |

### Pliki

| Skrot          | Akcja                        |
|----------------|------------------------------|
| `Ctrl+T`       | Nowy plik                    |
| `Ctrl+O`       | Otworz plik lub folder       |
| `Ctrl+S`       | Zapisz                       |
| `Ctrl+Shift+S` | Zapisz jako                  |
| `Ctrl+W`       | Zamknij zakladke / usun plik |

### Widok i panele

| Skrot    | Akcja                        |
|----------|------------------------------|
| `Ctrl+B` | Pokaz / ukryj terminal       |
| `Ctrl+R` | Odswiez drzewo plikow        |
| `Ctrl+M` | Marketplace                  |
| `Ctrl+,` | Ustawienia                   |

### Zakladki

| Skrot         | Akcja              |
|---------------|--------------------|
| `Ctrl+N`      | Nastepna zakladka  |
| `Ctrl+P`      | Poprzednia zakladka |
| `Alt+1`–`Alt+9` | Zakladka nr 1–9  |

### Edycja

| Skrot           | Akcja                     |
|-----------------|---------------------------|
| `Tab`           | Autocomplete lub wciecie  |
| `Ctrl+Z`        | Cofnij (Undo)             |
| `Ctrl+Y`        | Ponow (Redo)              |
| `Ctrl+D`        | Duplikuj linie            |
| `Ctrl+F`        | Szukaj w pliku            |
| `Home` / `End`  | Poczatek / koniec linii   |
| `Ctrl+←/→`     | Skocz o slowo             |
| `PgUp` / `PgDn` | Przewin strone            |

### Terminal (gdy aktywny)

| Skrot        | Akcja                       |
|--------------|-----------------------------|
| `Enter`      | Wykonaj komende             |
| `Up` / `Down` | Historia komend            |
| `PgUp/PgDn`  | Przewin output              |
| `Ctrl+C`     | Anuluj wpisywanie           |
| `Esc`        | Wróc fokus do edytora       |
| `Ctrl+B`     | Zamknij terminal            |

---

## Autocomplete

hdev oferuje autouzupelnianie inspirowane VSCode IntelliSense:

- **Slowa kluczowe jezyka** — np. `fn`, `let`, `match` dla Rust
- **Snippety** — wpisz `fn` i Tab — wstawia caly szkielet funkcji
- **Slowa z pliku** — zmienne i typy zdefiniowane w otwartym pliku

**Uzycie:** wpisz pierwsze litery, pojawi sie popup. `Tab` akceptuje podpowiedz, `↑↓` wybiera, `Esc` zamyka.

Mozna wylaczyc w ustawieniach (`Ctrl+,` -> "Autocomplete (Tab)").

---

## Obslugiwane jezyki

### Jezyki programowania

| Jezyk          | Rozszerzenie      |
|----------------|-------------------|
| Hacker Lang    | `.hl`             |
| Hacker Lang++  | `.hlpp` (BETA)    |
| H#             | `.hs`             |
| Rust           | `.rs`             |
| Python         | `.py`             |
| Go             | `.go`             |
| C              | `.c`, `.h`        |
| C++            | `.cpp`, `.cc`     |
| JavaScript     | `.js`             |
| TypeScript     | `.ts`             |
| Java           | `.java`           |
| Kotlin         | `.kt`             |
| Dart           | `.dart`           |
| Lua            | `.lua`            |
| Nim            | `.nim`            |
| Crystal        | `.cr`             |
| Odin           | `.odin`           |
| Shell          | `.sh`, `.bash`    |
| Vala           | `.vala`           |
| HTML           | `.html`           |
| CSS            | `.css`            |

### Formaty konfiguracyjne

| Format   | Rozszerzenie     |
|----------|------------------|
| JSON     | `.json`          |
| YAML     | `.yaml`, `.yml`  |
| TOML     | `.toml`          |
| HCL      | `.hcl`, `.tf`    |
| XML      | `.xml`           |
| HK Plugin | `.hk`           |

Wiecej jezykow mozna dodac przez Marketplace.

---

## Marketplace i pluginy

Marketplace laduje liste pluginow z pliku JSON:

```
https://github.com/HackerOS-Linux-System/hdev/blob/main/community/marketplace.json
```

Format pliku `marketplace.json`:

```json
{
  "marketplace": [
    {
      "name": "Nazwa pluginu",
      "description": "Krotki opis pluginu.",
      "download": "https://przyklad.pl/plugin.hk",
      "author": "Autor",
      "version": "1.0.0",
      "category": "language",
      "tags": ["rust", "lsp"]
    }
  ]
}
```

Pole `download` to URL do pliku `.hk`. Po nacisnieciu `Enter` w Marketplace, hdev pobiera plik przez `curl` lub `wget` i zapisuje do:

```
~/.cache/HackerOS/hdev/plugins/nazwa-pluginu.hk
```

### Format pluginu `.hk`

Pliki `.hk` sa parsowane przez biblioteke [hk-parser](https://crates.io/crates/hk-parser).

```ini
# Przykladowy plugin
[metadata]
name        = "moj-plugin"
version     = "1.0.0"
author      = "Autor"
description = "Opis pluginu"
hdev_min    = "0.1.0"

[syntax]
extensions  = ".myext"
comment     = "#"
keywords    = "if else for while return"

[hooks]
on_save     = "moj-linter $FILE"
on_open     = "echo Otwarto: $FILE"
```

Folder pluginow: `~/.cache/HackerOS/hdev/plugins/`

---

## Konfiguracja

Wszystkie ustawienia sa w `~/.cache/HackerOS/hdev/config.json`:

```json
{
  "theme": "hacker-dark",
  "tab_size": 4,
  "auto_save": true,
  "show_line_numbers": true,
  "show_file_tree": true,
  "word_wrap": false,
  "autocomplete_enabled": true,
  "terminal_shell": "hsh",
  "default_language_override": "auto",
  "recent_files": [],
  "installed_plugins": [],
  "marketplace_url": "https://raw.githubusercontent.com/..."
}
```

Dostepne motywy: `hacker-dark`, `hacker-green`, `cyberpunk`, `matrix`, `nord`, `solarized-dark`, `dracula`, `monokai`, `gruvbox`, `one-dark`.

Zmien motyw w ustawieniach: `Ctrl+,` -> "Motyw (Theme)" -> `←→`.

### Sciezki plikow

| Plik        | Sciezka                                         |
|-------------|-------------------------------------------------|
| Konfiguracja | `~/.cache/HackerOS/hdev/config.json`           |
| Sesja       | `~/.cache/HackerOS/hdev/session.json`           |
| Pluginy     | `~/.cache/HackerOS/hdev/plugins/*.hk`           |

---

## Terminal

Terminal hdev wykonuje komendy przez `hsh` (jesli zainstalowany) lub `sh`:

```sh
# Kazda komenda jest wykonywana jako:
hsh -c "twoja komenda"
# lub jesli brak hsh:
sh -c "twoja komenda"
```

Wbudowane komendy obsługiwane przez hdev (nie przez powloke):
- `cd <sciezka>` — zmiana katalogu
- `clear` / `cls` — czyszczenie outputu

---

## Architektura

```
src/
  main.rs          — punkt wejscia
  app.rs           — glowna logika, petla zdarzen
  ui.rs            — rendering TUI (Ratatui)
  editor.rs        — bufor tekstu, kursor, undo/redo
  highlight.rs     — silnik podswietlania skladni
  languages.rs     — definicje jezykow
  autocomplete.rs  — silnik autouzupelniania
  filetree.rs      — drzewo plikow
  terminal_panel.rs — panel terminala
  marketplace.rs   — marketplace z JSON
  plugins.rs       — skaner plikow .hk
  welcome.rs       — ekran powitalny
  keybinds.rs      — mapowanie klawiszy
  config.rs        — konfiguracja JSON
  utils.rs         — funkcje pomocnicze
```

---

## Licencja

MIT — HackerOS Team

---

*hdev jest czescia ekosystemu [HackerOS](https://hackeros-linux-system.github.io/HackerOS-Website/).*
