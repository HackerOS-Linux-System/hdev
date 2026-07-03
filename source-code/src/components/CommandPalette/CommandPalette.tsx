import { Command } from "cmdk";
import { useEffect } from "react";
import type { ReactNode, CSSProperties } from "react";
import { open as openDialog, save as saveDialog } from "@tauri-apps/plugin-dialog";
import { useUiStore } from "@/stores/uiStore";
import { useEditorStore } from "@/stores/editorStore";
import { useSettingsStore } from "@/stores/settingsStore";
import { THEMES } from "@/types";

export default function CommandPalette() {
  const { isCommandPaletteOpen, setCommandPaletteOpen, togglePanel, toggleTerminal, setRootDir } =
    useUiStore();
  const { newFile, activeTab, saveTab, closeTab, nextTab, prevTab } = useEditorStore();
  const { config, update } = useSettingsStore();

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key.toLowerCase() === "p") {
        e.preventDefault();
        setCommandPaletteOpen(!isCommandPaletteOpen);
      }
      if (e.key === "Escape" && isCommandPaletteOpen) {
        setCommandPaletteOpen(false);
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [isCommandPaletteOpen, setCommandPaletteOpen]);

  if (!isCommandPaletteOpen) return null;

  const run = (fn: () => void) => {
    fn();
    setCommandPaletteOpen(false);
  };

  const openFolder = async () => {
    try {
      const dir = await openDialog({ directory: true });
      if (typeof dir === "string") setRootDir(dir);
    } catch {
      /* poza Tauri */
    }
  };

  const saveAs = async () => {
    const active = activeTab();
    if (!active) return;
    try {
      const path = await saveDialog({ defaultPath: active.name });
      if (typeof path === "string") saveTab(active.id, path);
    } catch {
      /* poza Tauri */
    }
  };

  return (
    <div
      onClick={() => setCommandPaletteOpen(false)}
      style={{
        position: "fixed",
        inset: 0,
        background: "rgba(0,0,0,0.5)",
        zIndex: 1000,
        display: "flex",
        justifyContent: "center",
        paddingTop: "12vh",
      }}
    >
      <div onClick={(e) => e.stopPropagation()} style={{ width: 560 }}>
        <Command
          label="Paleta komend"
          style={{
            background: "var(--bg-2)",
            border: "1px solid var(--border)",
            borderRadius: "var(--radius-md)",
            boxShadow: "0 20px 60px rgba(0,0,0,0.55)",
            overflow: "hidden",
          }}
        >
          <Command.Input
            autoFocus
            placeholder="Wpisz komende…"
            style={{
              width: "100%",
              padding: "12px 14px",
              background: "var(--bg-1)",
              border: "none",
              borderBottom: "1px solid var(--border)",
              color: "var(--fg-0)",
              fontSize: 13,
              outline: "none",
              fontFamily: "var(--font-ui)",
            }}
          />
          <Command.List style={{ maxHeight: 360, overflowY: "auto", padding: 6 }}>
            <Command.Empty style={{ padding: 14, fontSize: 12, color: "var(--fg-muted)" }}>
              Brak wynikow.
            </Command.Empty>

            <Command.Group heading="Plik" style={groupStyle}>
              <Item onSelect={() => run(newFile)}>Nowy plik</Item>
              <Item onSelect={() => run(openFolder)}>Otworz folder…</Item>
              <Item onSelect={() => run(() => activeTab() && saveTab(activeTab()!.id))}>
                Zapisz
              </Item>
              <Item onSelect={() => run(saveAs)}>Zapisz jako…</Item>
              <Item onSelect={() => run(() => activeTab() && closeTab(activeTab()!.id))}>
                Zamknij karte
              </Item>
            </Command.Group>

            <Command.Group heading="Widok" style={groupStyle}>
              <Item onSelect={() => run(() => togglePanel("explorer"))}>Przelacz eksplorator</Item>
              <Item onSelect={() => run(() => togglePanel("search"))}>Przelacz szukanie</Item>
              <Item onSelect={() => run(() => togglePanel("marketplace"))}>Otworz Marketplace</Item>
              <Item onSelect={() => run(() => togglePanel("settings"))}>Otworz Ustawienia</Item>
              <Item onSelect={() => run(toggleTerminal)}>Przelacz terminal</Item>
              <Item onSelect={() => run(nextTab)}>Nastepna karta</Item>
              <Item onSelect={() => run(prevTab)}>Poprzednia karta</Item>
            </Command.Group>

            <Command.Group heading="Motyw" style={groupStyle}>
              {THEMES.map((theme) => (
                <Item key={theme} onSelect={() => run(() => update({ theme }))}>
                  {theme} {config.theme === theme ? "✓" : ""}
                </Item>
              ))}
            </Command.Group>
          </Command.List>
        </Command>
      </div>
    </div>
  );
}

const groupStyle: CSSProperties = {
  fontSize: 10.5,
  color: "var(--fg-muted)",
  textTransform: "uppercase",
  letterSpacing: "0.05em",
  padding: "6px 8px 2px",
};

function Item({ children, onSelect }: { children: ReactNode; onSelect: () => void }) {
  return (
    <Command.Item
      onSelect={onSelect}
      style={{
        padding: "8px 10px",
        borderRadius: "var(--radius-sm)",
        fontSize: 12.5,
        color: "var(--fg-0)",
        cursor: "pointer",
      }}
    >
      {children}
    </Command.Item>
  );
}
