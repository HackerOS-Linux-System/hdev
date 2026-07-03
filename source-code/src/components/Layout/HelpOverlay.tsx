import { useUiStore } from "@/stores/uiStore";

const SHORTCUTS: [string, string][] = [
  ["Ctrl+H", "Pomoc (ta lista)"],
  ["Ctrl+T", "Nowy plik"],
  ["Ctrl+W", "Zamknij plik"],
  ["Ctrl+S", "Zapisz"],
  ["Ctrl+Shift+S", "Zapisz jako"],
  ["Ctrl+O", "Otworz folder"],
  ["Ctrl+B", "Pokaz / ukryj terminal"],
  ["Ctrl+R", "Odswiez drzewo plikow"],
  ["Ctrl+M", "Marketplace"],
  ["Ctrl+,", "Ustawienia"],
  ["Ctrl+F", "Szukaj w plikach"],
  ["Ctrl+N", "Nastepna zakladka"],
  ["Ctrl+P", "Poprzednia zakladka"],
  ["Ctrl+Shift+P", "Paleta komend"],
  ["Ctrl+Z / Ctrl+Y", "Cofnij / ponow"],
  ["Esc", "Zamknij panel / paleta"],
];

export default function HelpOverlay() {
  const { isHelpOpen, toggleHelp } = useUiStore();
  if (!isHelpOpen) return null;

  return (
    <div
      onClick={toggleHelp}
      style={{
        position: "fixed",
        inset: 0,
        background: "rgba(0,0,0,0.55)",
        zIndex: 1000,
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <div
        onClick={(e) => e.stopPropagation()}
        style={{
          width: 420,
          background: "var(--bg-2)",
          border: "1px solid var(--border)",
          borderRadius: "var(--radius-md)",
          padding: 16,
          boxShadow: "0 20px 60px rgba(0,0,0,0.55)",
        }}
      >
        <h2 style={{ margin: "0 0 10px", fontSize: 14, color: "var(--accent)" }}>
          Skroty klawiszowe
        </h2>
        <div style={{ display: "flex", flexDirection: "column", gap: 4, maxHeight: 420, overflowY: "auto" }}>
          {SHORTCUTS.map(([key, desc]) => (
            <div key={key} style={{ display: "flex", justifyContent: "space-between", fontSize: 12 }}>
              <code
                className="mono-tight"
                style={{
                  background: "var(--bg-1)",
                  border: "1px solid var(--border)",
                  borderRadius: "var(--radius-sm)",
                  padding: "1px 6px",
                  color: "var(--fg-0)",
                }}
              >
                {key}
              </code>
              <span style={{ color: "var(--fg-1)" }}>{desc}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
