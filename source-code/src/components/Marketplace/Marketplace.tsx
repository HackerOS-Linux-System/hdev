import { useEffect, useState } from "react";
import type { MarketplacePlugin } from "@/types";
import { marketplaceApi } from "@/lib/ipc";
import { useSettingsStore } from "@/stores/settingsStore";

const CATEGORY_COLOR: Record<string, string> = {
  language: "var(--info)",
  lang: "var(--info)",
  theme: "#c07bff",
  formatter: "#5cff8a",
  linter: "var(--warning)",
  git: "#ff8a50",
  productivity: "#4fe0c0",
  hackeros: "var(--accent)",
};

export default function Marketplace() {
  const { config, update } = useSettingsStore();
  const [plugins, setPlugins] = useState<MarketplacePlugin[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState<string | null>(null);
  const [filter, setFilter] = useState("");

  const load = async () => {
    setLoading(true);
    setError(null);
    try {
      const list = await marketplaceApi.fetch(config.marketplace_url);
      setPlugins(list);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    load();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const install = async (plugin: MarketplacePlugin) => {
    setBusy(plugin.name);
    try {
      await marketplaceApi.install(plugin);
      if (!config.installed_plugins.includes(plugin.name)) {
        await update({ installed_plugins: [...config.installed_plugins, plugin.name] });
      }
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(null);
    }
  };

  const visible = plugins.filter(
    (p) =>
      p.name.toLowerCase().includes(filter.toLowerCase()) ||
      p.tags.some((t) => t.toLowerCase().includes(filter.toLowerCase())),
  );

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100%" }}>
      <div style={{ padding: "0 10px 8px", display: "flex", gap: 6 }}>
        <input
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
          placeholder="Filtruj pluginy…"
          style={{
            flex: 1,
            background: "var(--bg-2)",
            border: "1px solid var(--border)",
            borderRadius: "var(--radius-sm)",
            color: "var(--fg-0)",
            padding: "6px 8px",
            fontSize: 12,
          }}
        />
        <button className="icon-btn" title="Odswiez" onClick={load}>
          ↻
        </button>
      </div>

      {loading && <p style={{ padding: "0 10px", fontSize: 12, color: "var(--fg-muted)" }}>Ladowanie…</p>}
      {error && (
        <p style={{ padding: "0 10px", fontSize: 11.5, color: "var(--danger)" }}>
          Blad: {error}
        </p>
      )}

      <div className="scroll-y" style={{ flex: 1 }}>
        {visible.map((p) => {
          const installed = config.installed_plugins.includes(p.name);
          return (
            <div
              key={p.name}
              style={{
                margin: "6px 10px",
                padding: 10,
                border: "1px solid var(--border)",
                borderRadius: "var(--radius-md)",
                background: "var(--bg-2)",
              }}
            >
              <div style={{ display: "flex", justifyContent: "space-between", alignItems: "flex-start" }}>
                <strong style={{ fontSize: 12.5 }}>{p.name}</strong>
                <span
                  style={{
                    fontSize: 9.5,
                    padding: "1px 6px",
                    borderRadius: 10,
                    color: CATEGORY_COLOR[p.category?.toLowerCase()] ?? "var(--fg-muted)",
                    border: `1px solid ${CATEGORY_COLOR[p.category?.toLowerCase()] ?? "var(--border)"}`,
                  }}
                >
                  {p.category || "plugin"}
                </span>
              </div>
              <p style={{ fontSize: 11.5, color: "var(--fg-1)", margin: "4px 0" }}>{p.description}</p>
              <div style={{ fontSize: 10.5, color: "var(--fg-muted)", marginBottom: 6 }}>
                {p.author} • v{p.version}
              </div>
              <button
                disabled={busy === p.name}
                onClick={() => install(p)}
                style={{
                  width: "100%",
                  fontSize: 11,
                  padding: "5px 0",
                  borderRadius: "var(--radius-sm)",
                  border: "1px solid var(--border)",
                  background: installed ? "var(--bg-3)" : "var(--accent)",
                  color: installed ? "var(--fg-1)" : "var(--accent-contrast)",
                  fontWeight: 600,
                }}
              >
                {busy === p.name ? "Instalowanie…" : installed ? "Zainstalowano ✓ (reinstaluj)" : "Zainstaluj"}
              </button>
            </div>
          );
        })}
        {!loading && visible.length === 0 && !error && (
          <p style={{ padding: 10, fontSize: 12, color: "var(--fg-muted)" }}>Brak pluginow.</p>
        )}
      </div>
    </div>
  );
}
