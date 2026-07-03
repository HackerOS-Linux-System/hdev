import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEditorStore } from "@/stores/editorStore";

const appWindow = (() => {
  try {
    return getCurrentWindow();
  } catch {
    return null;
  }
})();

export default function TitleBar() {
  const activeTab = useEditorStore((s) => s.activeTab());
  const [isMaximized, setIsMaximized] = useState(false);

  useEffect(() => {
    if (!appWindow) return;
    appWindow.isMaximized().then(setIsMaximized).catch(() => {});
    const un = appWindow.onResized(() => {
      appWindow.isMaximized().then(setIsMaximized).catch(() => {});
    });
    return () => {
      un.then((f) => f());
    };
  }, []);

  const title = activeTab
    ? `${activeTab.name}${activeTab.isDirty ? " *" : ""} — hdev`
    : "hdev";

  return (
    <div
      data-tauri-drag-region
      style={{
        height: "var(--titlebar-h)",
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        background: "var(--bg-1)",
        borderBottom: "1px solid var(--border)",
        paddingLeft: 12,
        fontSize: 12,
        color: "var(--fg-muted)",
      }}
    >
      <div data-tauri-drag-region style={{ display: "flex", alignItems: "center", gap: 8 }}>
        <span style={{ color: "var(--accent)", fontWeight: 700 }}>hdev</span>
        <span style={{ opacity: 0.5 }}>/</span>
        <span data-tauri-drag-region style={{ color: "var(--fg-1)" }}>{title}</span>
      </div>
      {appWindow && (
        <div style={{ display: "flex", height: "100%" }}>
          <TitleBtn label="—" onClick={() => appWindow?.minimize()} />
          <TitleBtn label={isMaximized ? "❐" : "☐"} onClick={() => appWindow?.toggleMaximize()} />
          <TitleBtn label="✕" danger onClick={() => appWindow?.close()} />
        </div>
      )}
    </div>
  );
}

function TitleBtn({
  label,
  onClick,
  danger,
}: {
  label: string;
  onClick: () => void;
  danger?: boolean;
}) {
  return (
    <button
      onClick={onClick}
      style={{
        width: 44,
        height: "100%",
        background: "transparent",
        border: "none",
        color: "var(--fg-muted)",
        fontSize: 13,
      }}
      onMouseEnter={(e) => {
        e.currentTarget.style.background = danger ? "var(--danger)" : "var(--bg-3)";
        e.currentTarget.style.color = danger ? "#fff" : "var(--fg-0)";
      }}
      onMouseLeave={(e) => {
        e.currentTarget.style.background = "transparent";
        e.currentTarget.style.color = "var(--fg-muted)";
      }}
    >
      {label}
    </button>
  );
}
