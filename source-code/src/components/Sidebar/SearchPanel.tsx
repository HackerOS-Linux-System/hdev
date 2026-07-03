import { useMemo, useState } from "react";
import { useEditorStore } from "@/stores/editorStore";

export default function SearchPanel() {
  const [query, setQuery] = useState("");
  const tabs = useEditorStore((s) => s.tabs);
  const activeTab = useEditorStore((s) => s.activeTab());
  const requestJump = useEditorStore((s) => s.requestJump);
  const setActiveTab = useEditorStore((s) => s.setActiveTab);

  const results = useMemo(() => {
    if (!query.trim()) return [];
    const q = query.toLowerCase();
    const hits: { tabId: string; tabName: string; line: number; text: string }[] = [];
    for (const tab of tabs) {
      const lines = tab.content.split("\n");
      lines.forEach((line, idx) => {
        if (line.toLowerCase().includes(q)) {
          hits.push({ tabId: tab.id, tabName: tab.name, line: idx + 1, text: line.trim() });
        }
      });
    }
    return hits.slice(0, 300);
  }, [query, tabs]);

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100%" }}>
      <div style={{ padding: "0 10px 8px" }}>
        <input
          autoFocus
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Szukaj w otwartych plikach…"
          style={{
            width: "100%",
            background: "var(--bg-2)",
            border: "1px solid var(--border)",
            borderRadius: "var(--radius-sm)",
            color: "var(--fg-0)",
            padding: "6px 8px",
            fontSize: 12,
            fontFamily: "var(--font-code)",
          }}
        />
        {!activeTab && (
          <p style={{ fontSize: 11, color: "var(--fg-muted)", marginTop: 6 }}>
            Otworz plik, aby przeszukac tez jego zawartosc.
          </p>
        )}
      </div>
      <div className="scroll-y" style={{ flex: 1 }}>
        {results.map((r, i) => (
          <div
            key={i}
            onClick={() => {
              setActiveTab(r.tabId);
              requestJump(r.line);
            }}
            style={{
              padding: "5px 10px",
              fontSize: 12,
              cursor: "pointer",
              borderBottom: "1px solid var(--bg-2)",
            }}
            onMouseEnter={(e) => (e.currentTarget.style.background = "var(--bg-2)")}
            onMouseLeave={(e) => (e.currentTarget.style.background = "transparent")}
          >
            <div style={{ color: "var(--accent)", fontSize: 10.5 }}>
              {r.tabName}:{r.line}
            </div>
            <div className="mono-tight" style={{ color: "var(--fg-1)", whiteSpace: "nowrap", overflow: "hidden", textOverflow: "ellipsis" }}>
              {r.text || "​"}
            </div>
          </div>
        ))}
        {query.trim() && results.length === 0 && (
          <p style={{ padding: 10, fontSize: 12, color: "var(--fg-muted)" }}>Brak wynikow.</p>
        )}
      </div>
    </div>
  );
}
