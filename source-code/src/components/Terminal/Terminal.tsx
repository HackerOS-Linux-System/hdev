import { useEffect, useRef, useState } from "react";
import { useTerminalStore } from "@/stores/terminalStore";
import { useUiStore } from "@/stores/uiStore";
import { useSettingsStore } from "@/stores/settingsStore";

const KIND_COLOR: Record<string, string> = {
  output: "var(--fg-1)",
  error: "var(--danger)",
  input: "var(--accent)",
  info: "var(--fg-muted)",
};

export default function Terminal() {
  const { lines, cwd, running, execute, historyUp, historyDown, init } = useTerminalStore();
  const { isTerminalOpen, setTerminalOpen, rootDir } = useUiStore();
  const shell = useSettingsStore((s) => s.config.terminal_shell);
  const fontSize = useSettingsStore((s) => s.config.terminal_font_size);
  const [input, setInput] = useState("");
  const scrollRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    init(rootDir ?? "~");
  }, [rootDir, init]);

  useEffect(() => {
    if (isTerminalOpen) inputRef.current?.focus();
  }, [isTerminalOpen]);

  useEffect(() => {
    scrollRef.current?.scrollTo({ top: scrollRef.current.scrollHeight });
  }, [lines]);

  if (!isTerminalOpen) return null;

  const submit = () => {
    if (!input.trim() || running) return;
    execute(input, shell || "sh");
    setInput("");
  };

  return (
    <div
      style={{
        height: 260,
        borderTop: "1px solid var(--border)",
        background: "var(--bg-0)",
        display: "flex",
        flexDirection: "column",
        flexShrink: 0,
      }}
    >
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          padding: "4px 10px",
          borderBottom: "1px solid var(--border)",
          background: "var(--bg-1)",
        }}
      >
        <span style={{ fontSize: 11, color: "var(--fg-muted)", textTransform: "uppercase", letterSpacing: "0.06em" }}>
          Terminal {running && "· dziala…"}
        </span>
        <div style={{ display: "flex", gap: 2 }}>
          <button className="icon-btn" title="Wyczysc" onClick={() => useTerminalStore.getState().clear()}>
            ⌫
          </button>
          <button className="icon-btn" title="Zamknij (Ctrl+B)" onClick={() => setTerminalOpen(false)}>
            ✕
          </button>
        </div>
      </div>

      <div
        ref={scrollRef}
        className="scroll-y mono-tight"
        style={{ flex: 1, padding: "6px 10px", fontSize }}
      >
        {lines.map((line, i) => (
          <div key={i} style={{ color: KIND_COLOR[line.kind], whiteSpace: "pre-wrap", lineHeight: 1.5 }}>
            {line.text}
          </div>
        ))}
      </div>

      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: 8,
          padding: "6px 10px",
          borderTop: "1px solid var(--border)",
        }}
      >
        <span className="mono-tight" style={{ color: "var(--accent)", fontSize }}>
          {cwd} ❯
        </span>
        <input
          ref={inputRef}
          value={input}
          disabled={running}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter") submit();
            else if (e.key === "ArrowUp") {
              e.preventDefault();
              const v = historyUp();
              if (v != null) setInput(v);
            } else if (e.key === "ArrowDown") {
              e.preventDefault();
              const v = historyDown();
              if (v != null) setInput(v);
            }
          }}
          className="mono-tight"
          style={{
            flex: 1,
            background: "transparent",
            border: "none",
            outline: "none",
            color: "var(--fg-0)",
            fontSize,
          }}
          placeholder={running ? "wykonywanie…" : "wpisz komende…"}
        />
      </div>
    </div>
  );
}
