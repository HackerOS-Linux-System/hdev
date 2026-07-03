import { create } from "zustand";
import type { EditorTab } from "@/types";
import { fsApi } from "@/lib/ipc";
import { languageFromPath } from "@/utils/language";

let untitledCounter = 1;

interface EditorState {
  tabs: EditorTab[];
  activeTabId: string | null;
  pendingJumpLine: number | null;
  requestJump: (line: number) => void;
  clearJump: () => void;

  openFile: (path: string) => Promise<void>;
  newFile: () => void;
  closeTab: (id: string) => void;
  setActiveTab: (id: string) => void;
  updateContent: (id: string, content: string) => void;
  setCursor: (id: string, line: number, col: number) => void;
  saveTab: (id: string, saveAsPath?: string) => Promise<void>;
  nextTab: () => void;
  prevTab: () => void;
  activeTab: () => EditorTab | undefined;
}

export const useEditorStore = create<EditorState>((set, get) => ({
  tabs: [],
  activeTabId: null,
  pendingJumpLine: null,
  requestJump: (line) => set({ pendingJumpLine: line }),
  clearJump: () => set({ pendingJumpLine: null }),

  openFile: async (path) => {
    const existing = get().tabs.find((t) => t.path === path);
    if (existing) {
      set({ activeTabId: existing.id });
      return;
    }
    const content = await fsApi.readFile(path);
    const id = crypto.randomUUID();
    const tab: EditorTab = {
      id,
      path,
      name: path.split(/[/\\]/).pop() ?? path,
      content,
      language: languageFromPath(path),
      isDirty: false,
      cursorLine: 1,
      cursorCol: 1,
    };
    set((s) => ({ tabs: [...s.tabs, tab], activeTabId: id }));
  },

  newFile: () => {
    const id = crypto.randomUUID();
    const name = `bez-nazwy-${untitledCounter++}`;
    const tab: EditorTab = {
      id,
      path: null,
      name,
      content: "",
      language: "Plain Text",
      isDirty: false,
      cursorLine: 1,
      cursorCol: 1,
    };
    set((s) => ({ tabs: [...s.tabs, tab], activeTabId: id }));
  },

  closeTab: (id) => {
    set((s) => {
      const idx = s.tabs.findIndex((t) => t.id === id);
      const tabs = s.tabs.filter((t) => t.id !== id);
      let activeTabId = s.activeTabId;
      if (activeTabId === id) {
        const fallback = tabs[idx] ?? tabs[idx - 1] ?? tabs[0];
        activeTabId = fallback ? fallback.id : null;
      }
      return { tabs, activeTabId };
    });
  },

  setActiveTab: (id) => set({ activeTabId: id }),

  updateContent: (id, content) =>
    set((s) => ({
      tabs: s.tabs.map((t) => (t.id === id ? { ...t, content, isDirty: true } : t)),
    })),

  setCursor: (id, cursorLine, cursorCol) =>
    set((s) => ({
      tabs: s.tabs.map((t) => (t.id === id ? { ...t, cursorLine, cursorCol } : t)),
    })),

  saveTab: async (id, saveAsPath) => {
    const tab = get().tabs.find((t) => t.id === id);
    if (!tab) return;
    const path = saveAsPath ?? tab.path;
    if (!path) return; // wolanie powinno przejsc przez dialog "Zapisz jako"
    await fsApi.writeFile(path, tab.content);
    set((s) => ({
      tabs: s.tabs.map((t) =>
        t.id === id
          ? {
              ...t,
              path,
              name: path.split(/[/\\]/).pop() ?? path,
              language: languageFromPath(path),
              isDirty: false,
            }
          : t,
      ),
    }));
  },

  nextTab: () => {
    const { tabs, activeTabId } = get();
    if (tabs.length === 0) return;
    const idx = tabs.findIndex((t) => t.id === activeTabId);
    const next = tabs[(idx + 1) % tabs.length];
    set({ activeTabId: next.id });
  },

  prevTab: () => {
    const { tabs, activeTabId } = get();
    if (tabs.length === 0) return;
    const idx = tabs.findIndex((t) => t.id === activeTabId);
    const prev = tabs[(idx - 1 + tabs.length) % tabs.length];
    set({ activeTabId: prev.id });
  },

  activeTab: () => get().tabs.find((t) => t.id === get().activeTabId),
}));
