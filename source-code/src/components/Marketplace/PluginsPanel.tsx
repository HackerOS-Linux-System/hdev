import { useEffect, useState } from "react";
import type { LoadedPlugin } from "@/types";
import { pluginsApi, marketplaceApi } from "@/lib/ipc";
import { useSettingsStore } from "@/stores/settingsStore";

export default function PluginsPanel() {
  const [plugins, setPlugins] = useState<LoadedPlugin[]>([]);
  const [loading, setLoading] = useState(false);
  const { config, update } = useSettingsStore();

  const load = async () => {
    setLoading(true);
    try {
      setPlugins(await pluginsApi.scan());
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    load();
  }, []);

  const uninstall = async (name: string) => {
    await marketplaceApi.uninstall(name);
    await update({ installed_plugins: config.installed_plugins.filter((n) => n !== name) });
    load();
  };

  return (
    <div className="scroll-y" style={{ height: "100%", padding: "0 10px" }}>
      {loading && <p style={{ fontSize: 12, color: "var(--fg-muted)" }}>Skanowanie…</p>}
      {!loading && plugins.length === 0 && (
        <p style={{ fontSize: 12, color: "var(--fg-muted)" }}>
          Brak zainstalowanych pluginow. Przejdz do Marketplace, aby cos dodac.
        </p>
      )}
      {plugins.map((p) => (
        <div
          key={p.id}
          style={{
            margin: "6px 0",
            padding: 10,
            border: "1px solid var(--border)",
            borderRadius: "var(--radius-md)",
            background: "var(--bg-2)",
          }}
        >
          <div style={{ display: "flex", justifyContent: "space-between" }}>
            <strong style={{ fontSize: 12.5 }}>{p.name}</strong>
            <span
              style={{
                fontSize: 9.5,
                color: p.error ? "var(--danger)" : "var(--accent)",
              }}
            >
              {p.error ? "blad" : "aktywny"}
            </span>
          </div>
          <p style={{ fontSize: 11, color: "var(--fg-1)", margin: "4px 0" }}>
            {p.description || "Brak opisu."}
          </p>
          <div style={{ fontSize: 10.5, color: "var(--fg-muted)" }}>
            v{p.version} {p.author ? `• ${p.author}` : ""}
          </div>
          {p.syntax_extensions.length > 0 && (
            <div style={{ fontSize: 10.5, color: "var(--fg-muted)", marginTop: 2 }}>
              rozszerzenia: {p.syntax_extensions.join(", ")}
            </div>
          )}
          {p.error && (
            <div style={{ fontSize: 10.5, color: "var(--danger)", marginTop: 4 }}>{p.error}</div>
          )}
          <button
            onClick={() => uninstall(p.name)}
            style={{
              marginTop: 8,
              width: "100%",
              fontSize: 11,
              padding: "5px 0",
              borderRadius: "var(--radius-sm)",
              border: "1px solid var(--border)",
              background: "transparent",
              color: "var(--danger)",
            }}
          >
            Odinstaluj
          </button>
        </div>
      ))}
    </div>
  );
}
