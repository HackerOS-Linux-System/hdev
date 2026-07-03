import { useUiStore } from "@/stores/uiStore";
import FileTree from "@/components/FileTree/FileTree";
import SearchPanel from "@/components/Sidebar/SearchPanel";
import Marketplace from "@/components/Marketplace/Marketplace";
import PluginsPanel from "@/components/Marketplace/PluginsPanel";
import Settings from "@/components/Settings/Settings";

const TITLES: Record<string, string> = {
  explorer: "Eksplorator",
  search: "Szukaj",
  marketplace: "Marketplace",
  plugins: "Pluginy",
  settings: "Ustawienia",
};

export default function Sidebar() {
  const activePanel = useUiStore((s) => s.activePanel);

  if (!activePanel) return null;

  return (
    <div
      style={{
        width: 280,
        minWidth: 220,
        maxWidth: 480,
        borderRight: "1px solid var(--border)",
        background: "var(--bg-1)",
        display: "flex",
        flexDirection: "column",
        overflow: "hidden",
      }}
    >
      <div
        style={{
          padding: "8px 10px 4px",
          fontSize: 11,
          fontWeight: 700,
          letterSpacing: "0.08em",
          textTransform: "uppercase",
          color: "var(--fg-muted)",
        }}
      >
        {TITLES[activePanel]}
      </div>
      <div style={{ flex: 1, minHeight: 0 }}>
        {activePanel === "explorer" && <FileTree />}
        {activePanel === "search" && <SearchPanel />}
        {activePanel === "marketplace" && <Marketplace />}
        {activePanel === "plugins" && <PluginsPanel />}
        {activePanel === "settings" && <Settings />}
      </div>
    </div>
  );
}
