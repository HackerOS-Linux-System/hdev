import { useUiStore } from "@/stores/uiStore";
import type { PanelId } from "@/types";

const ITEMS: { id: PanelId; icon: string; label: string }[] = [
  { id: "explorer", icon: "📁", label: "Eksplorator (Ctrl+R)" },
  { id: "search", icon: "🔍", label: "Szukaj (Ctrl+F)" },
  { id: "marketplace", icon: "🛒", label: "Marketplace (Ctrl+M)" },
  { id: "plugins", icon: "🧩", label: "Pluginy" },
  { id: "settings", icon: "⚙️", label: "Ustawienia (Ctrl+,)" },
];

export default function ActivityBar() {
  const { activePanel, togglePanel, isTerminalOpen, toggleTerminal } = useUiStore();

  return (
    <div
      style={{
        width: "var(--activitybar-w)",
        background: "var(--bg-1)",
        borderRight: "1px solid var(--border)",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        paddingTop: 8,
        justifyContent: "space-between",
      }}
    >
      <div style={{ display: "flex", flexDirection: "column", gap: 4 }}>
        {ITEMS.map((item) => (
          <button
            key={item.id}
            title={item.label}
            className={`icon-btn ${activePanel === item.id ? "active" : ""}`}
            style={{ fontSize: 17, width: 34, height: 34, position: "relative" }}
            onClick={() => togglePanel(item.id)}
          >
            {item.icon}
            {activePanel === item.id && (
              <span
                style={{
                  position: "absolute",
                  left: -8,
                  top: 4,
                  bottom: 4,
                  width: 2,
                  background: "var(--accent)",
                  borderRadius: 2,
                }}
              />
            )}
          </button>
        ))}
      </div>
      <button
        title="Terminal (Ctrl+B)"
        className={`icon-btn ${isTerminalOpen ? "active" : ""}`}
        style={{ fontSize: 17, width: 34, height: 34, marginBottom: 10 }}
        onClick={toggleTerminal}
      >
        ⌨
      </button>
    </div>
  );
}
