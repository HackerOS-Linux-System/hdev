import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  FsEntry,
  HdevConfig,
  SessionData,
  MarketplacePlugin,
  LoadedPlugin,
  TerminalEventPayload,
} from "@/types";

/**
 * Cala komunikacja z warstwa Rust (Tauri commands) przechodzi przez ten plik,
 * zeby reszta apki nie musiala znac nazw komend ani obslugiwac `invoke` recznie.
 */

// ── Filesystem ──────────────────────────────────────────────────────────────

export const fsApi = {
  readDir: (path: string) => invoke<FsEntry[]>("fs_read_dir", { path }),
  readFile: (path: string) => invoke<string>("fs_read_file", { path }),
  writeFile: (path: string, contents: string) =>
    invoke<void>("fs_write_file", { path, contents }),
  createFile: (path: string) => invoke<void>("fs_create_file", { path }),
  createDir: (path: string) => invoke<void>("fs_create_dir", { path }),
  delete: (path: string) => invoke<void>("fs_delete", { path }),
  rename: (from: string, to: string) => invoke<void>("fs_rename", { from, to }),
  exists: (path: string) => invoke<boolean>("fs_exists", { path }),
  isDir: (path: string) => invoke<boolean>("fs_is_dir", { path }),
};

// ── Config / sesja ──────────────────────────────────────────────────────────

export const configApi = {
  load: () => invoke<HdevConfig>("config_load"),
  save: (config: HdevConfig) => invoke<void>("config_save", { config }),
  loadSession: () => invoke<SessionData>("session_load"),
  saveSession: (session: SessionData) => invoke<void>("session_save", { session }),
  themes: () => invoke<string[]>("config_themes"),
};

// ── Terminal ────────────────────────────────────────────────────────────────

export const terminalApi = {
  run: (sessionId: string, command: string, cwd: string, shell: string) =>
    invoke<void>("terminal_run", { sessionId, command, cwd, shell }),

  /** Subskrybuje strumien zdarzen terminala (stdout/stderr/exit). */
  onEvent: (cb: (evt: TerminalEventPayload) => void): Promise<UnlistenFn> =>
    listen<TerminalEventPayload>("terminal://event", (e) => cb(e.payload)),
};

// ── Marketplace / pluginy ───────────────────────────────────────────────────

export const marketplaceApi = {
  fetch: (url: string) => invoke<MarketplacePlugin[]>("marketplace_fetch", { url }),
  install: (plugin: MarketplacePlugin) =>
    invoke<string>("marketplace_install", { plugin }),
  uninstall: (name: string) => invoke<void>("marketplace_uninstall", { name }),
};

export const pluginsApi = {
  scan: () => invoke<LoadedPlugin[]>("plugins_scan"),
};
