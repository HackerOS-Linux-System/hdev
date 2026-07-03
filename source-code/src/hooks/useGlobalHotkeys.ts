import { useEffect } from "react";
import { save as saveDialog, open as openDialog } from "@tauri-apps/plugin-dialog";
import { useEditorStore } from "@/stores/editorStore";
import { useUiStore } from "@/stores/uiStore";

/**
 * Skroty klawiszowe 1:1 z wersja TUI (src/keybinds.rs), z dodatkami typowymi
 * dla GUI (Ctrl+Shift+P paleta komend — patrz CommandPalette.tsx).
 * Ctrl+Z / Ctrl+Y sa celowo pominiete tutaj: CodeMirror obsluguje je natywnie
 * dla aktywnego bufora.
 */
export function useGlobalHotkeys() {
  const { newFile, closeTab, activeTab, saveTab, nextTab, prevTab } = useEditorStore();
  const { toggleTerminal, togglePanel, toggleHelp, setRootDir } = useUiStore();

  useEffect(() => {
    const handler = async (e: KeyboardEvent) => {
      const ctrl = e.ctrlKey || e.metaKey;
      if (!ctrl) return;

      switch (e.key.toLowerCase()) {
        case "t":
          e.preventDefault();
          newFile();
          break;
        case "w": {
          e.preventDefault();
          const tab = activeTab();
          if (tab) closeTab(tab.id);
          break;
        }
        case "s": {
          e.preventDefault();
          const tab = activeTab();
          if (!tab) break;
          if (e.shiftKey || !tab.path) {
            try {
              const path = await saveDialog({ defaultPath: tab.name });
              if (typeof path === "string") saveTab(tab.id, path);
            } catch {
              /* poza Tauri */
            }
          } else {
            saveTab(tab.id);
          }
          break;
        }
        case "o": {
          e.preventDefault();
          try {
            const dir = await openDialog({ directory: true });
            if (typeof dir === "string") setRootDir(dir);
          } catch {
            /* poza Tauri */
          }
          break;
        }
        case "b":
          e.preventDefault();
          toggleTerminal();
          break;
        case "r":
          e.preventDefault();
          window.dispatchEvent(new CustomEvent("hdev:refresh-tree"));
          break;
        case "m":
          e.preventDefault();
          togglePanel("marketplace");
          break;
        case ",":
          e.preventDefault();
          togglePanel("settings");
          break;
        case "h":
          e.preventDefault();
          toggleHelp();
          break;
        case "f":
          e.preventDefault();
          togglePanel("search");
          break;
        case "n":
          e.preventDefault();
          nextTab();
          break;
        case "p":
          e.preventDefault();
          prevTab();
          break;
        default:
          break;
      }
    };

    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [newFile, closeTab, activeTab, saveTab, nextTab, prevTab, toggleTerminal, togglePanel, toggleHelp, setRootDir]);
}
