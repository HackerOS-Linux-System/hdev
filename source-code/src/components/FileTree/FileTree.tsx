import { useEffect, useRef, useState } from "react";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import type { FsEntry } from "@/types";
import { fsApi } from "@/lib/ipc";
import { useUiStore } from "@/stores/uiStore";
import { useSettingsStore } from "@/stores/settingsStore";
import TreeNode from "./TreeNode";

interface MenuState {
  entry: FsEntry;
  x: number;
  y: number;
}

export default function FileTree() {
  const { rootDir, setRootDir } = useUiStore();
  const addRecentFile = useSettingsStore((s) => s.addRecentFile);
  const [entries, setEntries] = useState<FsEntry[]>([]);
  const [menu, setMenu] = useState<MenuState | null>(null);
  const [refreshKey, setRefreshKey] = useState(0);
  const containerRef = useRef<HTMLDivElement>(null);

  const load = async (dir: string) => {
    const list = await fsApi.readDir(dir);
    setEntries(list);
  };

  useEffect(() => {
    if (rootDir) load(rootDir);
  }, [rootDir, refreshKey]);

  useEffect(() => {
    const close = () => setMenu(null);
    window.addEventListener("click", close);
    return () => window.removeEventListener("click", close);
  }, []);

  useEffect(() => {
    const onRefresh = () => setRefreshKey((k) => k + 1);
    window.addEventListener("hdev:refresh-tree", onRefresh);
    return () => window.removeEventListener("hdev:refresh-tree", onRefresh);
  }, []);

  const pickFolder = async () => {
    try {
      const dir = await openDialog({ directory: true, multiple: false });
      if (typeof dir === "string") {
        setRootDir(dir);
        await addRecentFile(dir);
      }
    } catch {
      /* dialog niedostepny poza Tauri */
    }
  };

  const refresh = () => setRefreshKey((k) => k + 1);

  const handleContextMenu = (entry: FsEntry, x: number, y: number) => {
    setMenu({ entry, x, y });
  };

  const parentOf = (path: string) => path.split(/[/\\]/).slice(0, -1).join("/");

  const newFileAt = async (dirPath: string) => {
    const name = window.prompt("Nazwa nowego pliku:");
    if (!name) return;
    await fsApi.createFile(`${dirPath}/${name}`);
    refresh();
  };

  const newFolderAt = async (dirPath: string) => {
    const name = window.prompt("Nazwa nowego folderu:");
    if (!name) return;
    await fsApi.createDir(`${dirPath}/${name}`);
    refresh();
  };

  const renamePath = async (path: string) => {
    const currentName = path.split(/[/\\]/).pop() ?? "";
    const name = window.prompt("Nowa nazwa:", currentName);
    if (!name || name === currentName) return;
    await fsApi.rename(path, `${parentOf(path)}/${name}`);
    refresh();
  };

  const deletePath = async (path: string) => {
    if (!window.confirm(`Usunac "${path}"? Tej operacji nie mozna cofnac.`)) return;
    await fsApi.delete(path);
    refresh();
  };

  if (!rootDir) {
    return (
      <div style={{ padding: 16, display: "flex", flexDirection: "column", gap: 10 }}>
        <p style={{ fontSize: 12, color: "var(--fg-muted)" }}>
          Nie otwarto zadnego folderu.
        </p>
        <button
          onClick={pickFolder}
          style={{
            background: "var(--accent)",
            color: "var(--accent-contrast)",
            border: "none",
            borderRadius: "var(--radius-sm)",
            padding: "8px 10px",
            fontWeight: 600,
            fontSize: 12,
          }}
        >
          Otworz folder…
        </button>
      </div>
    );
  }

  return (
    <div ref={containerRef} style={{ height: "100%", position: "relative" }}>
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          padding: "6px 10px",
          fontSize: 11,
          textTransform: "uppercase",
          letterSpacing: "0.06em",
          color: "var(--fg-muted)",
        }}
      >
        <span title={rootDir} style={{ overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>
          {rootDir.split(/[/\\]/).pop()}
        </span>
        <div style={{ display: "flex", gap: 2 }}>
          <button className="icon-btn" title="Nowy plik" onClick={() => newFileAt(rootDir)}>
            📄+
          </button>
          <button className="icon-btn" title="Nowy folder" onClick={() => newFolderAt(rootDir)}>
            📁+
          </button>
          <button className="icon-btn" title="Odswiez (Ctrl+R)" onClick={refresh}>
            ↻
          </button>
        </div>
      </div>
      <div className="scroll-y" style={{ height: "calc(100% - 30px)" }}>
        {entries.map((entry) => (
          <TreeNode key={entry.path} entry={entry} depth={0} onContextMenu={handleContextMenu} />
        ))}
      </div>

      {menu && (
        <div
          style={{
            position: "fixed",
            left: menu.x,
            top: menu.y,
            background: "var(--bg-2)",
            border: "1px solid var(--border)",
            borderRadius: "var(--radius-sm)",
            boxShadow: "0 8px 24px rgba(0,0,0,0.45)",
            zIndex: 100,
            minWidth: 180,
            fontSize: 12,
            padding: 4,
          }}
        >
          {menu.entry.is_dir && (
            <>
              <MenuItem label="Nowy plik" onClick={() => newFileAt(menu.entry.path)} />
              <MenuItem label="Nowy folder" onClick={() => newFolderAt(menu.entry.path)} />
              <Divider />
            </>
          )}
          <MenuItem label="Zmien nazwe" onClick={() => renamePath(menu.entry.path)} />
          <MenuItem label="Usun" danger onClick={() => deletePath(menu.entry.path)} />
        </div>
      )}
    </div>
  );
}

function MenuItem({
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
      onClick={(e) => {
        e.stopPropagation();
        onClick();
      }}
      style={{
        display: "block",
        width: "100%",
        textAlign: "left",
        background: "transparent",
        border: "none",
        padding: "6px 10px",
        borderRadius: "var(--radius-sm)",
        color: danger ? "var(--danger)" : "var(--fg-0)",
      }}
      onMouseEnter={(e) => (e.currentTarget.style.background = "var(--bg-3)")}
      onMouseLeave={(e) => (e.currentTarget.style.background = "transparent")}
    >
      {label}
    </button>
  );
}

function Divider() {
  return <div style={{ height: 1, background: "var(--border)", margin: "4px 2px" }} />;
}
