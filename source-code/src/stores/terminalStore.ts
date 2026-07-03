import { create } from "zustand";
import { terminalApi } from "@/lib/ipc";
import type { TerminalEventPayload } from "@/types";

export type TermLineKind = "output" | "error" | "input" | "info";

export interface TermLine {
  text: string;
  kind: TermLineKind;
}

interface TerminalState {
  lines: TermLine[];
  cwd: string;
  cmdHistory: string[];
  historyIdx: number;
  running: boolean;
  currentSessionId: string | null;
  hasPrintedForSession: boolean;

  init: (cwd: string) => void;
  execute: (raw: string, shell: string) => Promise<void>;
  historyUp: () => string | null;
  historyDown: () => string | null;
  handleEvent: (evt: TerminalEventPayload) => void;
  clear: () => void;
}

let unsubscribed = false;

export const useTerminalStore = create<TerminalState>((set, get) => ({
  lines: [],
  cwd: ".",
  cmdHistory: [],
  historyIdx: 0,
  running: false,
  currentSessionId: null,
  hasPrintedForSession: false,

  init: (cwd) => {
    if (get().lines.length > 0) return; // juz zainicjalizowany
    set({
      cwd,
      lines: [
        { text: "hdev terminal — Ctrl+B zamknij", kind: "info" },
        { text: `cwd: ${cwd}`, kind: "info" },
        { text: "──────────────────────────────", kind: "info" },
      ],
    });
    if (!unsubscribed) {
      unsubscribed = true;
      terminalApi.onEvent((evt) => get().handleEvent(evt));
    }
  },

  execute: async (raw, shell) => {
    const cmd = raw.trim();
    if (!cmd || get().running) return;

    set((s) => ({
      lines: [...s.lines, { text: `❯ ${cmd}`, kind: "input" }],
      cmdHistory: s.cmdHistory[0] === cmd ? s.cmdHistory : [cmd, ...s.cmdHistory].slice(0, 500),
      historyIdx: 0,
    }));

    // Wbudowane: cd
    if (cmd === "cd" || cmd.startsWith("cd ")) {
      const dir = cmd === "cd" ? "~" : cmd.slice(3).trim();
      const cwd = get().cwd;
      let next: string;
      if (dir === "~") next = "~";
      else if (dir.startsWith("/")) next = dir;
      else next = `${cwd}/${dir}`.replace(/\/+/g, "/");
      set((s) => ({
        cwd: next,
        lines: [...s.lines, { text: `→ ${next}`, kind: "info" }],
      }));
      return;
    }

    // Wbudowane: clear / cls
    if (cmd === "clear" || cmd === "cls") {
      set({ lines: [{ text: "Terminal wyczyszczony.", kind: "info" }] });
      return;
    }

    const sessionId = crypto.randomUUID();
    set({ running: true, currentSessionId: sessionId, hasPrintedForSession: false });
    try {
      await terminalApi.run(sessionId, cmd, get().cwd, shell);
    } catch (e) {
      set((s) => ({ lines: [...s.lines, { text: String(e), kind: "error" }] }));
      set({ running: false, currentSessionId: null });
    }
  },

  historyUp: () => {
    const { cmdHistory, historyIdx } = get();
    if (historyIdx >= cmdHistory.length) return null;
    set({ historyIdx: historyIdx + 1 });
    return cmdHistory[historyIdx];
  },

  historyDown: () => {
    const { historyIdx, cmdHistory } = get();
    if (historyIdx === 0) return "";
    const nextIdx = historyIdx - 1;
    set({ historyIdx: nextIdx });
    return nextIdx === 0 ? "" : cmdHistory[nextIdx - 1];
  },

  handleEvent: (evt) => {
    const { currentSessionId } = get();
    if (evt.session_id !== currentSessionId) return;

    if (evt.kind === "stdout") {
      set((s) => ({
        lines: [...s.lines, { text: evt.line, kind: "output" }],
        hasPrintedForSession: true,
      }));
    } else if (evt.kind === "stderr") {
      set((s) => ({
        lines: [...s.lines, { text: evt.line, kind: "error" }],
        hasPrintedForSession: true,
      }));
    } else if (evt.kind === "exit") {
      const printed = get().hasPrintedForSession;
      set((s) => {
        if (evt.code === -1 && !printed) {
          // sygnal "clear" z backendu (patrz terminal.rs) — na wszelki wypadek
          return { lines: s.lines, running: false, currentSessionId: null };
        }
        const summary: TermLine[] = printed
          ? []
          : [
              evt.code === 0
                ? { text: "OK", kind: "info" as const }
                : { text: `exit ${evt.code}`, kind: "error" as const },
            ];
        return { lines: [...s.lines, ...summary], running: false, currentSessionId: null };
      });
    } else if (evt.kind === "error") {
      set((s) => ({
        lines: [...s.lines, { text: evt.message, kind: "error" }],
        running: false,
        currentSessionId: null,
      }));
    }
  },

  clear: () => set({ lines: [] }),
}));
