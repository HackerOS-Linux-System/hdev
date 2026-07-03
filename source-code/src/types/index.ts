export interface FsEntry {
  name: string;
  path: string;
  is_dir: boolean;
  is_symlink: boolean;
  size: number;
  modified: number | null;
}

export interface HdevConfig {
  theme: string;
  font_size: number;
  tab_size: number;
  auto_save: boolean;
  show_line_numbers: boolean;
  show_file_tree: boolean;
  word_wrap: boolean;
  last_opened_path: string | null;
  recent_files: string[];
  installed_plugins: string[];
  marketplace_url: string;
  terminal_shell: string;
  default_language_override: string;
  autocomplete_enabled: boolean;
  minimap_enabled: boolean;
  open_tabs_on_startup: boolean;
  terminal_font_size: number;
}

export interface SessionData {
  open_files: string[];
  active_file: string | null;
  panel_state: string;
  terminal_history: string[];
}

export interface MarketplacePlugin {
  name: string;
  description: string;
  download: string;
  author: string;
  version: string;
  category: string;
  tags: string[];
}

export interface LoadedPlugin {
  id: string;
  name: string;
  version: string;
  author: string;
  description: string;
  path: string;
  active: boolean;
  error: string | null;
  syntax_extensions: string[];
  hooks: Record<string, string>;
}

export interface EditorTab {
  id: string;
  path: string | null; // null = niezapisany "Untitled"
  name: string;
  content: string;
  language: string;
  isDirty: boolean;
  cursorLine: number;
  cursorCol: number;
}

export type TerminalEventPayload =
  | { kind: "stdout"; session_id: string; line: string }
  | { kind: "stderr"; session_id: string; line: string }
  | { kind: "exit"; session_id: string; code: number }
  | { kind: "error"; session_id: string; message: string };

export type PanelId = "explorer" | "search" | "marketplace" | "plugins" | "settings";

export const THEMES = [
  "hacker-dark",
  "hacker-green",
  "cyberpunk",
  "matrix",
  "nord",
  "solarized-dark",
  "dracula",
  "monokai",
  "gruvbox",
  "one-dark",
] as const;

export type ThemeName = (typeof THEMES)[number];
