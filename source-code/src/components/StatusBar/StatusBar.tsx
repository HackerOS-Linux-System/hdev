import { useEditorStore } from "@/stores/editorStore";
import { useSettingsStore } from "@/stores/settingsStore";
import { useUiStore } from "@/stores/uiStore";

export default function StatusBar() {
  const activeTab = useEditorStore((s) => s.activeTab());
  const config = useSettingsStore((s) => s.config);
  const { isTerminalOpen, toggleTerminal, rootDir } = useUiStore();

  return (
    <div
      style={{
        height: "var(--statusbar-h)",
        background: "var(--accent)",
        color: "var(--accent-contrast)",
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        padding: "0 10px",
        fontSize: 11,
        fontWeight: 600,
      }}
    >
      <div style={{ display: "flex", gap: 14, alignItems: "center" }}>
        <StatusItem onClick={toggleTerminal} label={`⌨ ${isTerminalOpen ? "Ukryj terminal" : "Terminal"}`} />
        {rootDir && <StatusItem label={`📁 ${rootDir.split(/[/\\]/).pop()}`} />}
        {activeTab?.isDirty && <StatusItem label="● niezapisane" />}
      </div>
      <div style={{ display: "flex", gap: 14, alignItems: "center" }}>
        {activeTab && (
          <>
            <StatusItem label={`Ln ${activeTab.cursorLine}, Kol ${activeTab.cursorCol}`} />
            <StatusItem label={activeTab.language} />
          </>
        )}
        <StatusItem label={`Tab: ${config.tab_size}`} />
        <StatusItem label={config.theme} />
      </div>
    </div>
  );
}

function StatusItem({ label, onClick }: { label: string; onClick?: () => void }) {
  return (
    <span
      onClick={onClick}
      style={{ cursor: onClick ? "pointer" : "default", opacity: 0.92 }}
    >
      {label}
    </span>
  );
}
