# hdev

Narzędzie deweloperskie ekosystemu **HackerOS**, napisane w 100% w H#:
edytor tekstu w terminalu (TUI) **oraz** przeglądarka kodu źródłowego organizacji
[HackerOS-Linux-System](https://github.com/HackerOS-Linux-System) (GUI).

## Użycie

```
hdev              # przeglądarka plików + edytor TUI (styl Visual Studio)
hdev <plik>       # edycja wybranego pliku
hdev app          # GUI: frontend do GitHuba dla organizacji HackerOS-Linux-System
hdev --help
```

### Edytor (TUI)
| Klawisz | Akcja |
|---|---|
| `Ctrl+S` | zapis + snapshot w lokalnej historii |
| `Ctrl+H` | panel lokalnej historii (hgit) |
| `Esc` | powrót do przeglądarki plików |
| `Ctrl+A` / `Ctrl+E` | początek / koniec linii |
| `Ctrl+C` | wyjście (niezapisane zmiany przepadają) |

### GUI (`hdev app`)
Lista repozytoriów organizacji → drzewo plików → podgląd pliku. Rozpoznaje języki,
których GitHub nie zna:

| Język | Rozszerzenie | Kolor |
|---|---|---|
| H# | `.h#` | ciemnoczerwony `#8B0000` |
| HackerScript | `.hcs` | szary `#808080` |
| Hacker Lang | `.hl` | fioletowy `#800080` |

Kolory żyją w `config/languages.hk` (można nadpisać w `~/.config/hdev/languages.hk`
lub `/usr/share/hdev/`), więc nowy język to trzy linie konfiguracji.

## Biblioteki (bytes.io)
- `tui` – edytor (Model/Update/View, lista plików, viewport historii)
- `git` (hgit) – lokalna historia edycji
- `silver` – okno GUI przeglądarki

`hdev` nie deklaruje żadnego bloku `extern` — cały FFI jest już w bibliotekach.

## Budowanie
```
bytes install
bytes build
./build/hdev app
```
Do GUI potrzebny jest plik `.ttf` (domyślnie DejaVu Sans z systemu, albo `HDEV_FONT=/ścieżka.ttf`).

## Ograniczenia (świadome)
- **Kod nie został skompilowany ani uruchomiony** — powstał na podstawie źródeł
  H#, tui, git i silver; możliwe są drobne błędy składni/typów do poprawienia.
- **GitHub API bez tokenu**: `std -> net_http` nie wspiera własnych nagłówków, więc
  obowiązuje limit 60 zapytań/godz. na IP; widoczne max 100 repo i 80 wpisów katalogu.
- **hgit ≠ prawdziwy git** (własny format na dysku). Dlatego historia edycji trzyma się
  w `~/.local/share/hdev/history/`, poza repozytorium projektu, a znaczek gałęzi
  czyta wyłącznie refy. Przeglądarka GUI nie klonuje repo — czyta je przez API.
- Silver nie ma pętli w szablonach i `on-click` nie niesie argumentów, więc GUI używa
  puli komend `item_0..item_79` i odświeżania przez `on_tick` (opis w `src/gui.h#`).
- Podświetlanie składni jest heurystyczne (komentarze, stringi, słowa kluczowe H#).
- Edytor: brak zaznaczania, schowka, cofania (Ctrl+Z) i szukania.
- Pliki w GUI powyżej ~20 000 znaków są obcinane w podglądzie („Save a local copy” zapisuje całość do `~/hdev-downloads`).

## Licencja
MIT
