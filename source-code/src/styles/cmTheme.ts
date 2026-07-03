import { EditorView } from "@codemirror/view";
import { HighlightStyle, syntaxHighlighting } from "@codemirror/language";
import { tags as t } from "@lezer/highlight";

/**
 * Zamiast twardo kodowac kolory dla kazdego z 10 motywow, referencjujemy
 * zmienne CSS (var(--accent) itd.) zdefiniowane w styles/themes.css.
 * Dzieki temu przelaczenie motywu w Ustawieniach natychmiast przekolorowuje
 * edytor, bez przebudowy instancji CodeMirror.
 */
export const hdevEditorTheme = EditorView.theme(
  {
    "&": {
      color: "var(--fg-0)",
      backgroundColor: "var(--bg-0)",
      height: "100%",
      fontSize: "var(--cm-font-size, 14px)",
    },
    ".cm-content": {
      fontFamily: "var(--font-code)",
      caretColor: "var(--accent)",
      padding: "10px 0",
    },
    ".cm-cursor, .cm-dropCursor": { borderLeftColor: "var(--accent)" },
    "&.cm-focused .cm-selectionBackground, .cm-selectionBackground, .cm-content ::selection": {
      backgroundColor: "color-mix(in srgb, var(--accent) 28%, transparent)",
    },
    ".cm-activeLine": { backgroundColor: "var(--bg-1)" },
    ".cm-activeLineGutter": { backgroundColor: "var(--bg-1)", color: "var(--fg-0)" },
    ".cm-gutters": {
      backgroundColor: "var(--bg-0)",
      color: "var(--fg-muted)",
      border: "none",
      borderRight: "1px solid var(--border)",
    },
    ".cm-lineNumbers .cm-gutterElement": { padding: "0 8px 0 12px" },
    ".cm-foldPlaceholder": {
      background: "var(--bg-3)",
      border: "1px solid var(--border)",
      color: "var(--fg-muted)",
    },
    ".cm-matchingBracket, .cm-nonmatchingBracket": {
      backgroundColor: "color-mix(in srgb, var(--accent) 25%, transparent)",
      outline: "1px solid var(--accent)",
    },
    ".cm-searchMatch": {
      backgroundColor: "color-mix(in srgb, var(--warning) 35%, transparent)",
    },
    ".cm-searchMatch.cm-searchMatch-selected": {
      backgroundColor: "color-mix(in srgb, var(--warning) 60%, transparent)",
    },
    ".cm-tooltip": {
      backgroundColor: "var(--bg-2)",
      border: "1px solid var(--border)",
      color: "var(--fg-0)",
    },
    ".cm-tooltip-autocomplete ul li[aria-selected]": {
      backgroundColor: "var(--bg-3)",
      color: "var(--accent)",
    },
    ".cm-panels": { backgroundColor: "var(--bg-1)", color: "var(--fg-0)" },
  },
  { dark: true },
);

export const hdevHighlightStyle = HighlightStyle.define([
  { tag: t.keyword, color: "var(--accent)", fontWeight: "600" },
  { tag: [t.name, t.deleted, t.character, t.propertyName, t.macroName], color: "var(--fg-0)" },
  { tag: [t.function(t.variableName), t.labelName], color: "var(--info)" },
  { tag: [t.color, t.constant(t.name), t.standard(t.name)], color: "var(--warning)" },
  { tag: [t.definition(t.name), t.separator], color: "var(--fg-0)" },
  { tag: [t.typeName, t.className, t.number, t.changed, t.annotation, t.modifier, t.self, t.namespace], color: "var(--warning)" },
  { tag: [t.operator, t.operatorKeyword, t.url, t.escape, t.regexp, t.link, t.special(t.string)], color: "var(--accent-dim)" },
  { tag: [t.meta, t.comment], color: "var(--fg-muted)", fontStyle: "italic" },
  { tag: t.strong, fontWeight: "bold" },
  { tag: t.emphasis, fontStyle: "italic" },
  { tag: t.strikethrough, textDecoration: "line-through" },
  { tag: t.link, color: "var(--info)", textDecoration: "underline" },
  { tag: t.heading, fontWeight: "bold", color: "var(--accent)" },
  { tag: [t.atom, t.bool, t.special(t.variableName)], color: "var(--warning)" },
  { tag: [t.processingInstruction, t.string, t.inserted], color: "var(--accent)" },
  { tag: t.invalid, color: "var(--danger)" },
]);

export const editorExtensions = [hdevEditorTheme, syntaxHighlighting(hdevHighlightStyle)];
