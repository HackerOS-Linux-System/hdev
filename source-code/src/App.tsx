import { useEffect } from "react";
import TitleBar from "@/components/Layout/TitleBar";
import ActivityBar from "@/components/Sidebar/ActivityBar";
import Sidebar from "@/components/Sidebar/Sidebar";
import TabBar from "@/components/TabBar/TabBar";
import Editor from "@/components/Editor/Editor";
import Terminal from "@/components/Terminal/Terminal";
import StatusBar from "@/components/StatusBar/StatusBar";
import CommandPalette from "@/components/CommandPalette/CommandPalette";
import HelpOverlay from "@/components/Layout/HelpOverlay";
import { useSettingsStore } from "@/stores/settingsStore";
import { useUiStore } from "@/stores/uiStore";
import { useGlobalHotkeys } from "@/hooks/useGlobalHotkeys";

export default function App() {
  const loadSettings = useSettingsStore((s) => s.load);
  const settingsLoaded = useSettingsStore((s) => s.loaded);
  const showFileTree = useSettingsStore((s) => s.config.show_file_tree);
  const { activePanel, togglePanel } = useUiStore();

  useGlobalHotkeys();

  useEffect(() => {
    loadSettings();
  }, [loadSettings]);

  useEffect(() => {
    if (settingsLoaded && !showFileTree && activePanel === "explorer") {
      togglePanel("explorer");
    }
    // uruchamiane tylko raz po zaladowaniu configu — celowo bez activePanel w deps
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [settingsLoaded, showFileTree]);

  if (!settingsLoaded) {
    return (
      <div
        style={{
          height: "100vh",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          background: "#0a0d0a",
          color: "#35e07a",
          fontFamily: "monospace",
        }}
      >
        hdev — ladowanie…
      </div>
    );
  }

  return (
    <div className="app-shell">
      <TitleBar />
      <div className="app-body">
        <ActivityBar />
        <Sidebar />
        <div style={{ display: "flex", flexDirection: "column", minWidth: 0, overflow: "hidden" }}>
          <TabBar />
          <div style={{ flex: 1, minHeight: 0 }}>
            <Editor />
          </div>
          <Terminal />
        </div>
      </div>
      <StatusBar />
      <CommandPalette />
      <HelpOverlay />
    </div>
  );
}
