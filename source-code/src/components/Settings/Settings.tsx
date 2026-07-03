import { useSettingsStore } from "@/stores/settingsStore";
import { THEMES } from "@/types";
import type { ReactNode, CSSProperties } from "react";

export default function Settings() {
  const { config, update } = useSettingsStore();

  return (
    <div className="scroll-y" style={{ height: "100%", padding: "0 12px 16px", fontSize: 12.5 }}>
      <Section title="Wyglad">
        <Field label="Motyw (Theme)">
          <select
            value={config.theme}
            onChange={(e) => update({ theme: e.target.value })}
            style={selectStyle}
          >
            {THEMES.map((t) => (
              <option key={t} value={t}>
                {t}
              </option>
            ))}
          </select>
        </Field>
        <Field label="Rozmiar czcionki edytora">
          <NumberInput
            value={config.font_size}
            min={9}
            max={32}
            onChange={(v) => update({ font_size: v })}
          />
        </Field>
        <Field label="Rozmiar czcionki terminala">
          <NumberInput
            value={config.terminal_font_size}
            min={9}
            max={28}
            onChange={(v) => update({ terminal_font_size: v })}
          />
        </Field>
        <ToggleField
          label="Numery linii"
          checked={config.show_line_numbers}
          onChange={(v) => update({ show_line_numbers: v })}
        />
        <ToggleField
          label="Zawijanie wierszy (word wrap)"
          checked={config.word_wrap}
          onChange={(v) => update({ word_wrap: v })}
        />
        <ToggleField
          label="Minimapa"
          checked={config.minimap_enabled}
          onChange={(v) => update({ minimap_enabled: v })}
        />
        <ToggleField
          label="Pokaz drzewo plikow przy starcie"
          checked={config.show_file_tree}
          onChange={(v) => update({ show_file_tree: v })}
        />
      </Section>

      <Section title="Edycja">
        <Field label="Rozmiar tabulacji">
          <NumberInput value={config.tab_size} min={1} max={12} onChange={(v) => update({ tab_size: v })} />
        </Field>
        <ToggleField
          label="Autocomplete (Tab)"
          checked={config.autocomplete_enabled}
          onChange={(v) => update({ autocomplete_enabled: v })}
        />
        <ToggleField
          label="Auto-zapis"
          checked={config.auto_save}
          onChange={(v) => update({ auto_save: v })}
        />
        <ToggleField
          label="Przywracaj otwarte karty przy starcie"
          checked={config.open_tabs_on_startup}
          onChange={(v) => update({ open_tabs_on_startup: v })}
        />
      </Section>

      <Section title="Terminal">
        <Field label="Powloka (shell)">
          <input
            value={config.terminal_shell}
            onChange={(e) => update({ terminal_shell: e.target.value })}
            placeholder="hsh lub sh"
            style={selectStyle}
          />
        </Field>
      </Section>

      <Section title="Marketplace">
        <Field label="URL marketplace.json">
          <input
            value={config.marketplace_url}
            onChange={(e) => update({ marketplace_url: e.target.value })}
            style={selectStyle}
          />
        </Field>
      </Section>
    </div>
  );
}

function Section({ title, children }: { title: string; children: ReactNode }) {
  return (
    <div style={{ marginTop: 14 }}>
      <div
        style={{
          fontSize: 10.5,
          fontWeight: 700,
          letterSpacing: "0.06em",
          textTransform: "uppercase",
          color: "var(--accent)",
          marginBottom: 8,
        }}
      >
        {title}
      </div>
      <div style={{ display: "flex", flexDirection: "column", gap: 10 }}>{children}</div>
    </div>
  );
}

function Field({ label, children }: { label: string; children: ReactNode }) {
  return (
    <label style={{ display: "flex", flexDirection: "column", gap: 4 }}>
      <span style={{ color: "var(--fg-1)" }}>{label}</span>
      {children}
    </label>
  );
}

function ToggleField({
  label,
  checked,
  onChange,
}: {
  label: string;
  checked: boolean;
  onChange: (v: boolean) => void;
}) {
  return (
    <label style={{ display: "flex", alignItems: "center", gap: 8, cursor: "pointer" }}>
      <input type="checkbox" checked={checked} onChange={(e) => onChange(e.target.checked)} />
      <span style={{ color: "var(--fg-1)" }}>{label}</span>
    </label>
  );
}

function NumberInput({
  value,
  min,
  max,
  onChange,
}: {
  value: number;
  min: number;
  max: number;
  onChange: (v: number) => void;
}) {
  return (
    <input
      type="number"
      min={min}
      max={max}
      value={value}
      onChange={(e) => onChange(Math.min(max, Math.max(min, Number(e.target.value) || min)))}
      style={selectStyle}
    />
  );
}

const selectStyle: CSSProperties = {
  background: "var(--bg-2)",
  border: "1px solid var(--border)",
  borderRadius: "var(--radius-sm)",
  color: "var(--fg-0)",
  padding: "6px 8px",
  fontSize: 12,
  fontFamily: "var(--font-ui)",
};
