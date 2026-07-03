import { useEditorStore } from "@/stores/editorStore";
import { useSettingsStore } from "@/stores/settingsStore";
import { useUiStore } from "@/stores/uiStore";

export default function Welcome() {
  const newFile = useEditorStore((s) => s.newFile);
  const openFile = useEditorStore((s) => s.openFile);
  const recent = useSettingsStore((s) => s.config.recent_files);
  const toggleCommandPalette = useUiStore((s) => s.toggleCommandPalette);

  return (
    <div
      style={{
        height: "100%",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        gap: 22,
        color: "var(--fg-muted)",
      }}
    >
      <div style={{ textAlign: "center" }}>
        <div style={{ fontSize: 46, color: "var(--accent)", fontWeight: 800, letterSpacing: "-0.02em" }}>
          hdev
        </div>
        <div style={{ fontSize: 12.5, marginTop: 4 }}>notatnik + IDE dla HackerOS</div>
      </div>

      <div style={{ display: "flex", gap: 10 }}>
        <ActionButton label="Nowy plik" hint="Ctrl+N" onClick={newFile} />
        <ActionButton label="Paleta komend" hint="Ctrl+Shift+P" onClick={toggleCommandPalette} />
      </div>

      {recent.length > 0 && (
        <div style={{ width: 360 }}>
          <div style={{ fontSize: 11, textTransform: "uppercase", letterSpacing: "0.06em", marginBottom: 8 }}>
            Ostatnie
          </div>
          <div style={{ display: "flex", flexDirection: "column", gap: 4 }}>
            {recent.slice(0, 6).map((path) => (
              <button
                key={path}
                onClick={() => openFile(path)}
                style={{
                  textAlign: "left",
                  background: "var(--bg-1)",
                  border: "1px solid var(--border)",
                  borderRadius: "var(--radius-sm)",
                  padding: "7px 10px",
                  color: "var(--fg-1)",
                  fontSize: 12,
                  fontFamily: "var(--font-code)",
                  overflow: "hidden",
                  textOverflow: "ellipsis",
                  whiteSpace: "nowrap",
                }}
                onMouseEnter={(e) => (e.currentTarget.style.borderColor = "var(--accent)")}
                onMouseLeave={(e) => (e.currentTarget.style.borderColor = "var(--border)")}
              >
                {path}
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

function ActionButton({ label, hint, onClick }: { label: string; hint: string; onClick: () => void }) {
  return (
    <button
      onClick={onClick}
      style={{
        display: "flex",
        flexDirection: "column",
        gap: 2,
        alignItems: "center",
        background: "var(--bg-1)",
        border: "1px solid var(--border)",
        borderRadius: "var(--radius-md)",
        padding: "10px 18px",
        color: "var(--fg-0)",
      }}
      onMouseEnter={(e) => (e.currentTarget.style.borderColor = "var(--accent)")}
      onMouseLeave={(e) => (e.currentTarget.style.borderColor = "var(--border)")}
    >
      <span style={{ fontSize: 12.5, fontWeight: 600 }}>{label}</span>
      <span style={{ fontSize: 10.5, color: "var(--fg-muted)" }}>{hint}</span>
    </button>
  );
}
