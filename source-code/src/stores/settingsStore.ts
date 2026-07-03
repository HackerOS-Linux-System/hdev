import { create } from "zustand";
import type { HdevConfig } from "@/types";
import { configApi } from "@/lib/ipc";

const DEFAULT_CONFIG: HdevConfig = {
  theme: "hacker-dark",
  font_size: 14,
  tab_size: 4,
  auto_save: true,
  show_line_numbers: true,
  show_file_tree: true,
  word_wrap: false,
  last_opened_path: null,
  recent_files: [],
  installed_plugins: [],
  marketplace_url:
    "https://raw.githubusercontent.com/HackerOS-Linux-System/hdev/main/community/marketplace.json",
  terminal_shell: "sh",
  default_language_override: "auto",
  autocomplete_enabled: true,
  minimap_enabled: true,
  open_tabs_on_startup: true,
  terminal_font_size: 13,
};

interface SettingsState {
  config: HdevConfig;
  loaded: boolean;
  load: () => Promise<void>;
  update: (patch: Partial<HdevConfig>) => Promise<void>;
  addRecentFile: (path: string) => Promise<void>;
}

export const useSettingsStore = create<SettingsState>((set, get) => ({
  config: DEFAULT_CONFIG,
  loaded: false,

  load: async () => {
    try {
      const config = await configApi.load();
      set({ config, loaded: true });
      document.documentElement.dataset.theme = config.theme;
    } catch {
      // Poza Tauri (np. podglad w przegladarce podczas developmentu UI) po
      // prostu zostajemy przy domyslnej konfiguracji w pamieci.
      set({ config: DEFAULT_CONFIG, loaded: true });
      document.documentElement.dataset.theme = DEFAULT_CONFIG.theme;
    }
  },

  update: async (patch) => {
    const next = { ...get().config, ...patch };
    set({ config: next });
    document.documentElement.dataset.theme = next.theme;
    try {
      await configApi.save(next);
    } catch {
      /* tryb bez backendu Tauri — ignorujemy blad zapisu */
    }
  },

  addRecentFile: async (path) => {
    const current = get().config.recent_files.filter((p) => p !== path);
    const recent_files = [path, ...current].slice(0, 20);
    await get().update({ recent_files, last_opened_path: path });
  },
}));
