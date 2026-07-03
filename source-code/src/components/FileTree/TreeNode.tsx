import { useState } from "react";
import type { FsEntry } from "@/types";
import { fsApi } from "@/lib/ipc";
import { fileIcon } from "@/utils/language";
import { useEditorStore } from "@/stores/editorStore";

interface Props {
  entry: FsEntry;
  depth: number;
  onContextMenu: (entry: FsEntry, x: number, y: number) => void;
}

export default function TreeNode({ entry, depth, onContextMenu }: Props) {
  const [expanded, setExpanded] = useState(false);
  const [children, setChildren] = useState<FsEntry[] | null>(null);
  const [loading, setLoading] = useState(false);
  const openFile = useEditorStore((s) => s.openFile);
  const activePath = useEditorStore((s) => s.activeTab()?.path);

  const toggle = async () => {
    if (!entry.is_dir) {
      openFile(entry.path);
      return;
    }
    if (!expanded && children === null) {
      setLoading(true);
      try {
        const list = await fsApi.readDir(entry.path);
        setChildren(list);
      } finally {
        setLoading(false);
      }
    }
    setExpanded(!expanded);
  };

  const isActive = !entry.is_dir && activePath === entry.path;

  return (
    <div>
      <div
        onClick={toggle}
        onContextMenu={(e) => {
          e.preventDefault();
          onContextMenu(entry, e.clientX, e.clientY);
        }}
        style={{
          display: "flex",
          alignItems: "center",
          gap: 6,
          padding: "2px 8px",
          paddingLeft: 8 + depth * 14,
          cursor: "pointer",
          fontSize: 12.5,
          whiteSpace: "nowrap",
          background: isActive ? "var(--bg-3)" : "transparent",
          color: isActive ? "var(--accent)" : "var(--fg-1)",
          borderLeft: isActive ? "2px solid var(--accent)" : "2px solid transparent",
        }}
        onMouseEnter={(e) => {
          if (!isActive) e.currentTarget.style.background = "var(--bg-2)";
        }}
        onMouseLeave={(e) => {
          if (!isActive) e.currentTarget.style.background = "transparent";
        }}
      >
        {entry.is_dir ? (
          <span style={{ width: 10, display: "inline-block", opacity: 0.7 }}>
            {loading ? "⋯" : expanded ? "▾" : "▸"}
          </span>
        ) : (
          <span style={{ width: 10, display: "inline-block" }} />
        )}
        <span>{fileIcon(entry.path, entry.is_dir)}</span>
        <span
          style={{
            overflow: "hidden",
            textOverflow: "ellipsis",
          }}
        >
          {entry.name}
        </span>
      </div>
      {entry.is_dir && expanded && children && (
        <div>
          {children.map((child) => (
            <TreeNode
              key={child.path}
              entry={child}
              depth={depth + 1}
              onContextMenu={onContextMenu}
            />
          ))}
          {children.length === 0 && (
            <div
              style={{
                paddingLeft: 8 + (depth + 1) * 14,
                fontSize: 11.5,
                color: "var(--fg-muted)",
                fontStyle: "italic",
              }}
            >
              (pusto)
            </div>
          )}
        </div>
      )}
    </div>
  );
}
