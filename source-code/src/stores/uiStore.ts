import { create } from "zustand";
import type { PanelId } from "@/types";

interface UiState {
  activePanel: PanelId | null;
  isTerminalOpen: boolean;
  isCommandPaletteOpen: boolean;
  isHelpOpen: boolean;
  rootDir: string | null;

  togglePanel: (panel: PanelId) => void;
  toggleTerminal: () => void;
  setTerminalOpen: (open: boolean) => void;
  toggleCommandPalette: () => void;
  setCommandPaletteOpen: (open: boolean) => void;
  toggleHelp: () => void;
  setRootDir: (dir: string | null) => void;
}

export const useUiStore = create<UiState>((set, get) => ({
  activePanel: "explorer",
  isTerminalOpen: false,
  isCommandPaletteOpen: false,
  isHelpOpen: false,
  rootDir: null,

  togglePanel: (panel) =>
    set({ activePanel: get().activePanel === panel ? null : panel }),

  toggleTerminal: () => set({ isTerminalOpen: !get().isTerminalOpen }),
  setTerminalOpen: (open) => set({ isTerminalOpen: open }),

  toggleCommandPalette: () =>
    set({ isCommandPaletteOpen: !get().isCommandPaletteOpen }),
  setCommandPaletteOpen: (open) => set({ isCommandPaletteOpen: open }),

  toggleHelp: () => set({ isHelpOpen: !get().isHelpOpen }),

  setRootDir: (dir) => set({ rootDir: dir }),
}));
