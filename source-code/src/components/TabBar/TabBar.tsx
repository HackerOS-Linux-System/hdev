import { useEditorStore } from "@/stores/editorStore";
import { fileIcon } from "@/utils/language";

export default function TabBar() {
  const { tabs, activeTabId, setActiveTab, closeTab, saveTab } = useEditorStore();

  if (tabs.length === 0) return null;

  return (
    <div
      className="scroll-y"
      style={{
        height: "var(--tabbar-h)",
        display: "flex",
        overflowX: "auto",
        overflowY: "hidden",
        background: "var(--bg-1)",
        borderBottom: "1px solid var(--border)",
        flexShrink: 0,
      }}
    >
      {tabs.map((tab) => {
        const active = tab.id === activeTabId;
        return (
          <div
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            onMouseDown={(e) => {
              if (e.button === 1) {
                e.preventDefault();
                closeTab(tab.id);
              }
            }}
            title={tab.path ?? tab.name}
            style={{
              display: "flex",
              alignItems: "center",
              gap: 6,
              padding: "0 8px 0 12px",
              minWidth: 110,
              maxWidth: 220,
              height: "100%",
              cursor: "pointer",
              fontSize: 12,
              borderRight: "1px solid var(--border)",
              background: active ? "var(--bg-2)" : "transparent",
              color: active ? "var(--fg-0)" : "var(--fg-muted)",
              position: "relative",
            }}
          >
            {active && (
              <span
                className="hdev-scanline"
                style={{ position: "absolute", left: 0, right: 0, top: 0 }}
              />
            )}
            <span>{fileIcon(tab.path ?? tab.name, false)}</span>
            <span
              style={{
                flex: 1,
                overflow: "hidden",
                textOverflow: "ellipsis",
                whiteSpace: "nowrap",
              }}
            >
              {tab.name}
            </span>
            {tab.isDirty && (
              <span style={{ color: "var(--accent)", fontSize: 16, lineHeight: 0 }} title="Niezapisane zmiany">
                •
              </span>
            )}
            <button
              onClick={(e) => {
                e.stopPropagation();
                if (tab.isDirty && tab.path) {
                  saveTab(tab.id);
                }
                closeTab(tab.id);
              }}
              className="icon-btn"
              style={{ width: 18, height: 18, fontSize: 12, padding: 0 }}
              title="Zamknij (Ctrl+W)"
            >
              ✕
            </button>
          </div>
        );
      })}
    </div>
  );
}
